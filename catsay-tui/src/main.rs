use cursive::{event::Key, views::TextView};

fn main() {
    let mut siv = cursive::default();
    let cat_text = "Meow!
\\
 \\
    /\\_/\\
   ( O O )
   =( I )=";

    siv.add_layer(TextView::new(cat_text));

    // Listen to Key::Esc and quit
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}
