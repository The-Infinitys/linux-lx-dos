use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

pub fn welcome() -> Result<(), crate::LxDosError> {
    let application = Application::builder()
        .application_id("com.example.welcome")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Welcome to LX-DOS")
            .default_width(400)
            .default_height(300)
            .build();

        let welcome_message = Label::new(Some("Welcome to LX-DOS!\n\nThis is a placeholder message. In a real application, this would display useful information or a setup guide."));
        window.set_child(Some(&welcome_message));

        window.show();
    });

    application.run_with_args::<&str>(&[]);

    Ok(())
}
