// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

pub mod modules;
pub mod utils;
pub mod qt_lx_dos {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
