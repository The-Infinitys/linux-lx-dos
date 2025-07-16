use super::vm;
use crate::qt_lx_dos;
use crate::utils::args::Args;
use crate::utils::error::LxDosError;
pub struct App {
    vm: vm::QemuSystem,
}

impl App {
    pub fn run(&self, args: Args) -> Result<(), LxDosError> {
        println!("{:#?}", self.vm);
        if args.gui {
            unsafe {
                qt_lx_dos::run_qt_app();
            }
        }
        if args.cli {
            println!("Hello, World!");
        }
        return Ok(());
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            vm: vm::QemuSystem::default(),
        }
    }
}
