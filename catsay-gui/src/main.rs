use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Image, Label, Orientation};

fn main() {
    let app = Application::new(Some("nt2311-vn.catsay-gui"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);
        let layout_box = GtkBox::new(Orientation::Vertical, 0);
        let label = Label::new(Some("Meow!\n    \\\n    \\"));
        layout_box.add(&label);

        let image_path = "images/cat.png";

        let cat_image = Image::from_file(image_path);
        layout_box.add(&cat_image);
        window.add(&layout_box);
        window.show_all();
    });

    app.run();
}
