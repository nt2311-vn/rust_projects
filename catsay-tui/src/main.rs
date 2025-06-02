use cursive::{
    event::Key,
    view::Nameable,
    views::{Checkbox, Dialog, EditView, ListView},
    Cursive,
};

struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

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

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the xcat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Dead?", Checkbox::new().with_name("dead")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_name("message", |t: &mut EditView| t.get_content())
                    .unwrap();

                let is_dead = s
                    .call_on_name("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();

                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead,
                };
            }),
    );
}
