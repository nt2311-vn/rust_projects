use cursive::{event::Key, views::Dialog};

fn main() {
    let mut siv = cursive::default();
    let cat_text = "Meow!
\\
 \\
    /\\_/\\
   ( O O )
   =( I )=";

    siv.add_layer(Dialog::text(cat_text).button("OK", |s| s.quit()));

    // Listen to Key::Esc and quit
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}
