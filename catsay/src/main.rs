use std::env::args;
fn main() {
    let message = args()
        .nth(1)
        .expect("Missing the message. Usage: catsay <message>");

    println!("{}", message);
    println!(" \\");
    println!(" \\");
    println!("    /\\_/\\");
    println!("   ( O O )");
    println!("   =( I )=");
}
