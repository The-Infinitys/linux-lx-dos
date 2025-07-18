use gtk::{glib::ExitCode, prelude::GtkWindowExt};

use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    let app = App::default();
    let gui = app.gui.clone(); // guiをクローンしてクロージャに移動
    app.gui.connect_open(move |_application, _files, _str| {
        let window = gui.build_window("Lx-DOS");
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
