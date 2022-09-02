use clap::{Parser, builder::Resettable};

#[derive(Parser)]
#[clap(author, version, about, long_about = Resettable::Reset)]
struct Cli {
    #[clap(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name.as_deref());
}
