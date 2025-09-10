mod git;

use git::reflog::get_reflog;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 10, short, long)]
    count: usize,
}

fn main() {
    let cli = Cli::parse();

    let recents = get_reflog(cli.count);

    for rec in recents.iter() {
        rec.display();
    }
}
