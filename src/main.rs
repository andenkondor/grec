mod git;

use git::reflog::get_reflog;

use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 10, short, long)]
    count: usize,
    #[arg(default_value_t = true, short, long)]
    prompt_checkout: bool,
}

fn main() {
    let cli = Cli::parse();

    let recents = get_reflog(cli.count);

    for (idx, rec) in recents.iter().enumerate() {
        rec.display(idx);
    }

    if cli.prompt_checkout {
        println!("Please insert the number of the line for checkout:");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read input");
        let x: i32 = input_line.trim().parse().expect("Input not an integer");
        let rec = recents.get((x - 1) as usize).expect("invalid index");
        rec.check_out();
    }
}
