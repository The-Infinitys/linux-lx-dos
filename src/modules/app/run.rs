use super::App;
use crate::LxDosError;

pub fn run(app: &mut App) -> Result<(), LxDosError> {
    println!("{:#?}", app);
    Ok(())
}
