pub mod command;
pub mod modules;
pub mod utils;
pub use utils::error::LxDosError;
pub mod qt6 {
    #[allow(warnings)]
    #[allow(non_upper_case_globals)]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    mod bind {
        include!(concat!(env!("OUT_DIR"), "/qt6-bind.rs"));
    }
}
