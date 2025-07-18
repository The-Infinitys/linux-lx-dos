use gtk::glib::ExitCode;

use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    let app = App::default();
    let gui = app.gui.clone(); // guiをクローンしてクロージャに移動
    app.gui.connect_open(move |_application, _files, _str| {
        use gtk::prelude::*;
        use gtk::Button;
        let button = Button::builder()
            .label("Press me!")
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .build();

        // Connect to "clicked" signal of `button`
        button.connect_clicked(|button| {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        });

        let window = gui
            .window_builder("Lx-DOS")
            .height_request(600)
            .width_request(800)
            .child(&button)
            .build();
        window.present();
    });
    let result = app.gui.run();
    if result == ExitCode::new(0) {
        Ok(())
    } else {
        Err(LxDosError::Message(format!(
            "The GTK process was exited with {}",
            result.get()
        )))
    }
}
