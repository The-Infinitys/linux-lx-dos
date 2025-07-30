use super::super::modules::app::App;
use crate::LxDosError;
pub fn run() -> Result<(), LxDosError> {
    App::start()
}
