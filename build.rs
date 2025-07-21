fn main() {
    // Build qt-tray using CMake
    let dst = cmake::build("lib/qt-tray");

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=qt-tray");

    // Generate bindings for qt-tray
    let bindings = bindgen::Builder::default()
        .header("lib/qt-tray/src/lib.hpp")
        .header("lib/qt-tray/src/modules/tray.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("qt-tray_bindings.rs"))
        .expect("Couldn't write bindings!");
}
