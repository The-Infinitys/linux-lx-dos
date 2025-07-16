use crate::utils::args::Args;
use crate::qt_lx_dos;
use crate::utils::error::LxDosError;
pub struct App {}

impl App {
    pub fn run(&self, args: Args)->Result<(), LxDosError> {
        if args.gui{
            unsafe {
                qt_lx_dos::run_qt_app();
            }            
        }
        if args.cli{
            println!("Hello, World!");
        }
        return Ok(())
    }
}
impl Default for App {
    fn default() -> Self {
        Self {}
    }
}
