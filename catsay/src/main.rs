use clap::Parser;

#[derive(Parser)]
struct Options {
    messages: String,
}
fn main() {
    let options = Options::parse();
    let message = options.messages;
    println!("{}", message);
    println!(" \\");
    println!(" \\");
    println!("    /\\_/\\");
    println!("   ( O O )");
    println!("   =( I )=");
}
