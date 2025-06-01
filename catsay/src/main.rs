use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::{
    fs,
    io::{self, Read},
    path,
};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow")]
    /// What does the cat say?
    messages: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat apear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified files
    catfile: Option<path::PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of argument
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.messages;
    };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };

    match &options.catfile {
        Some(path) => {
            let cat_template = fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;

            let eye = format!("{}", eye.white().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
            Ok(())
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!("    /\\_/\\");
            println!("   ( {eye} {eye} )");
            println!("   =( I )=");
            Ok(())
        }
    }
}
