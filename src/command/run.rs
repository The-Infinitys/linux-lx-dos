use gtk::prelude::WidgetExt;

use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    let app = App::default();
    let window = app.gui.bundle_window("Lx-DOS");
    window.activate();
    Ok(())
}
