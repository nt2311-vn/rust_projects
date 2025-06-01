use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow")]
    /// What does the cat say?
    messages: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat apear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.messages;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };
    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!(" \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )");
    println!("   =( I )=");
}
