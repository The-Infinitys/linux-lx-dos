fn main() {
    // Build qt-tray using CMake
    let mut config = cmake::Config::new("lib/qt-tray");
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".into());
    config.profile(&profile);
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=qt-tray");

    // Generate bindings for qt-tray
    let bindings = bindgen::Builder::default()
        .header("lib/qt-tray/src/lib.hpp")
        .clang_args(pkg_config::probe_library("Qt6Widgets").unwrap().include_paths.iter().map(|path| format!("-I{}", path.display())))
        .allowlist_function("create_qt_tray")
        .allowlist_function("destroy_qt_tray")
        .allowlist_function("qt_tray_set_icon")
        .allowlist_function("qt_tray_set_tool_tip")
        .allowlist_function("qt_tray_add_menu_item")
        .allowlist_function("qt_tray_on_activated")
        .allowlist_type("qt_tray::QtTray")
        .opaque_type("QObject")
        .opaque_type("QMenu")
        .opaque_type("QAction")
        .opaque_type("QSystemTrayIcon")
        .opaque_type("QString")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("qt-tray_bindings.rs"))
        .expect("Couldn't write bindings!");
}
