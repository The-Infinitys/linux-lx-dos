use super::App;
use crate::LxDosError;
pub fn run(app: &App) -> Result<(), LxDosError> {
    println!("{:#?}", app);
    Ok(())
}
