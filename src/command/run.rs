use gtk::prelude::GtkWindowExt;

use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    let app = App::default();
    let gui = app.gui.clone(); // guiをクローンしてクロージャに移動
    app.gui.connect_activate(move |_application| {
        let window = gui.bundle_window("Lx-DOS");
        window.present();
    });
    app.gui.run(std::env::args().collect());
    Ok(())
}
