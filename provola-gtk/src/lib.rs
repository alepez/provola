use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn run() {
    let application = Application::builder()
        .application_id("dev.alepez.provola")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Provola")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");

        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        window.add(&button);

        window.show_all();
    });

    application.run();
}
