use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();
    let cat_text = "Meow!
\\
 \\
    /\\_/\\
   ( O O )
   =( I )=";

    siv.add_layer(TextView::new(cat_text));
    siv.run();
}
