use colored::Colorize;
use core::str;
use regex::Regex;
use std::collections::HashSet;
use std::process::Command;

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
    pub fn check_out(&self) {
        Command::new("git")
            .arg("checkout")
            .arg(self.recent_checkout.git_ref.clone())
            .output()
            .expect("error while checking out");
    }
    pub fn display(&self, idx: usize) {
        println!(
            "{:>2}: {:<50} {:<2} {:<4} --- {:<20} --- {}",
            idx + 1,
            self.recent_checkout.git_ref.clone(),
            if self.has_upstream {
                "UP".green()
            } else {
                "".normal()
            },
            if !self.locally_accessible {
                "GONE".red()
            } else {
                "".normal()
            },
            self.recent_checkout.relative_checkout_time.clone(),
            self.commit_message.clone().split('\n').next().unwrap()
        );
    }
}

pub fn get_reflog(count: usize) -> Vec<RecentCheckoutWithMetadata> {
    let mut unique_ids = HashSet::new();
    let re_checkout_line = Regex::new(r"checkout: moving from \S* to (\S*) HEAD@\{(.*)\}").unwrap();
    let re_head_symbol = Regex::new(r"^(?:HEAD|ORIG_HEAD)(?:(?:@|^|~).*)?$").unwrap();

    get_reflog_lines()
        .iter()
        .filter_map(|line| re_checkout_line.captures(line))
        .map(|caps| RecentCheckout {
            git_ref: caps[1].to_string(),
            relative_checkout_time: caps[2].parse().unwrap(),
        })
        .skip(1)
        .filter(|co| !re_head_symbol.is_match(&co.git_ref))
        .filter(|item| unique_ids.insert(item.git_ref.to_string()))
        .map(|item| create_checkout_with_metadata(&item))
        .take(count)
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

    let output_string = String::from_utf8(output.stdout);

    match output_string {
        Ok(x) => x.split("\n").map(String::from).collect(),
        Err(_) => Default::default(),
    }
}

fn create_checkout_with_metadata(checkout: &RecentCheckout) -> RecentCheckoutWithMetadata {
    RecentCheckoutWithMetadata {
        recent_checkout: RecentCheckout {
            git_ref: checkout.git_ref.clone(),
            relative_checkout_time: checkout.relative_checkout_time.clone(),
        },
        commit_message: get_commit_message(&checkout.git_ref),
        has_upstream: has_upstream(&checkout.git_ref),
        locally_accessible: is_locally_accessible(&checkout.git_ref),
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
    let remote_prefix: String = "refs/remotes/origin/".to_owned();
    let remote_reference = remote_prefix + reference;
    let output = Command::new("git")
        .args(["show-branch", &remote_reference[..]])
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
