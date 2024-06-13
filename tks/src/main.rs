use clap::{self, Parser, Subcommand};
use tks::util::KernelInfo;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    List {},
    Search {},
    Build {},
}

fn main() {
    let args = Args::parse();

    KernelInfo::new();



    println!("Hello, world!");
}
