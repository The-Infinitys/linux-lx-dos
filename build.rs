use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("lib/qt-lx-dos");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // staticライブラリとして他に利用するライブラリはなし
    println!("cargo:rustc-link-lib=static=qt-lx-dos");

    // C++ソースコードの場合は必ずこれを追加すること
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // CMakeLists.txt内の記述とは別に、その他のライブラリは必要なものを全て記述する必要あり
    println!("cargo:rustc-link-lib=dylib=Qt6Core");
    println!("cargo:rustc-link-lib=dylib=Qt6Gui");
    println!("cargo:rustc-link-lib=dylib=Qt6Widgets");
    println!("cargo:rustc-link-lib=dylib=EGL");
    println!("cargo:rustc-link-lib=dylib=GLESv2");
    println!("cargo:rustc-link-lib=dylib=X11");

    println!("cargo:rerun-if-changed=lib/qt-lx-dos/**/*.hpp");
    println!("cargo:rerun-if-changed=lib/qt-lx-dos/**/*.cpp");
    println!("cargo:rerun-if-changed=lib/qt-lx-dos/**/*.qrc");
    println!("cargo:rerun-if-changed=lib/qt-lx-dos/CMakeLists.txt");
    let bindings = bindgen::Builder::default()
        .header("lib/qt-lx-dos/api/qt-lx-dos.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
