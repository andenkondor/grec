use core::str;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::process::Command;

static RE_CHECKOUT_LINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"checkout: moving from \S* to (\S*) HEAD@\{(.*)\}").unwrap());

static RE_HEAD_SYMBOL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?:HEAD|ORIG_HEAD)(?:(?:@|^|~).*)?$").unwrap());

#[derive(Debug)]
pub struct RecentCheckout {
    pub git_ref: String,
    pub relative_checkout_time: String,
}

#[derive(Debug)]
pub struct RecentCheckoutWithMetadata {
    pub recent_checkout: RecentCheckout,
    pub commit_message: String,
    pub has_upstream: bool,
    pub locally_accessible: bool,
}

impl RecentCheckoutWithMetadata {
    pub fn display(&self) {
        let first_line = self.commit_message.lines().next().unwrap_or("");

        println!(
            "{},{},{},{},{}",
            &self.recent_checkout.git_ref,
            if self.has_upstream { "UP" } else { "" },
            if !self.locally_accessible { "GONE" } else { "" },
            &self.recent_checkout.relative_checkout_time,
            first_line
        );
    }
}

pub fn get_reflog(count: usize) -> Vec<RecentCheckoutWithMetadata> {
    let mut unique_ids = HashSet::new();
    let current_branch = get_current_branch();

    let checkout_entries: Vec<RecentCheckout> = get_reflog_lines()
        .iter()
        .filter_map(|line| RE_CHECKOUT_LINE.captures(line))
        .map(|caps| RecentCheckout {
            git_ref: caps[1].to_string(),
            relative_checkout_time: caps[2].to_string(),
        })
        .filter(|item| item.git_ref != current_branch)
        .filter(|co| !RE_HEAD_SYMBOL.is_match(&co.git_ref))
        .filter(|item| unique_ids.insert(item.git_ref.clone()))
        .take(count)
        .collect();

    checkout_entries
        .into_par_iter()
        .map(|item| create_checkout_with_metadata(&item))
        .collect()
}

fn get_reflog_lines() -> Vec<String> {
    let output = Command::new("git")
        .args([
            "reflog",
            "show",
            "--pretty=format:%gs %gd",
            "--date=relative",
        ])
        .output()
        .expect("Failed to execute git command");

    match String::from_utf8(output.stdout) {
        Ok(x) => x.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    }
}

fn create_checkout_with_metadata(checkout: &RecentCheckout) -> RecentCheckoutWithMetadata {
    let git_ref = &checkout.git_ref;

    RecentCheckoutWithMetadata {
        recent_checkout: RecentCheckout {
            git_ref: git_ref.clone(),
            relative_checkout_time: checkout.relative_checkout_time.clone(),
        },
        commit_message: get_commit_message(git_ref),
        has_upstream: has_upstream(git_ref),
        locally_accessible: is_locally_accessible(git_ref),
    }
}

fn get_commit_message(reference: &str) -> String {
    let output = Command::new("git")
        .args(["log", "--format=%B", "-n", "1", reference])
        .output()
        .expect("Failed to execute git command");

    String::from_utf8(output.stdout).unwrap_or_default()
}

fn has_upstream(reference: &str) -> bool {
    let remote_reference = format!("refs/remotes/origin/{}", reference);
    let output = Command::new("git")
        .args(["show-branch", &remote_reference])
        .output();

    match output {
        Ok(outp) => outp.status.success(),
        Err(_) => false,
    }
}

fn is_locally_accessible(reference: &str) -> bool {
    let output = Command::new("git").args(["rev-parse", reference]).output();

    match output {
        Ok(outp) => outp.status.success(),
        Err(_) => false,
    }
}

fn get_current_branch() -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to execute git command");

    String::from_utf8(output.stdout)
        .unwrap_or_default()
        .trim()
        .to_string()
}
