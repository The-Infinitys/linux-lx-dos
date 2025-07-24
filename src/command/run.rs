use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    let app = App::default();
    app.gui.run_qt_app()
}
