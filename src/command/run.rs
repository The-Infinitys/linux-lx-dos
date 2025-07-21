use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    let app = App::default();
    app.run()
}
