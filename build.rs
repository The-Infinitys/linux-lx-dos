use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("lib/qt6-bind");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // staticライブラリとして他に利用するライブラリはなし
    println!("cargo:rustc-link-lib=static=qt6-bind");

    // C++ソースコードの場合は必ずこれを追加すること
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // CMakeLists.txt内の記述とは別に、その他のライブラリは必要なものを全て記述する必要あり
    println!("cargo:rustc-link-lib=dylib=Qt6Core");
    println!("cargo:rustc-link-lib=dylib=Qt6Gui");
    println!("cargo:rustc-link-lib=dylib=Qt6Widgets");
    println!("cargo:rustc-link-lib=dylib=EGL");
    println!("cargo:rustc-link-lib=dylib=GLESv2");
    println!("cargo:rustc-link-lib=dylib=X11");

    println!("cargo:rerun-if-changed=lib/qt-tray/src/**/*.hpp");
    println!("cargo:rerun-if-changed=lib/qt-tray/src/**/*.cpp");
    println!("cargo:rerun-if-changed=lib/qt6-bind/CMakeLists.txt");
    println!("cargo:rerun-if-changed=lib/qt6-bind/src/lib.hpp"); // lib.hppが変更されたら再ビルド

    // --- ここからが重要な修正点です ---

    // Qt6のインクルードパスをbindgenに渡す
    // 環境変数 QT_DIR が設定されていればそれを使用、なければ一般的なパスを試す
    let qt_dir = env::var("QT_DIR")
        .or_else(|_| env::var("QT_ROOT")) // QT_ROOT も試す
        .unwrap_or_else(|_| {
            // ここはあなたのQtのインストールパスに合わせて**必ず**調整してください。
            // 例: "/opt/Qt/6.5.0/gcc_64" や "/home/youruser/Qt/6.6.1/gcc_64" など
            // もしQtをシステムにインストールしている（例: apt install qt6-base-dev）なら、
            // /usr/include/qt6 や /usr/include/x86_64-linux-gnu/qt6 などになるかもしれません。
            eprintln!("WARNING: QT_DIR or QT_ROOT environment variable not found. Trying common Qt6 install paths.");
            // 以下のパスはあくまで例です。あなたの環境に合わせて変更してください。
            // 多くのLinuxディストリビューションでは、Qtは /usr/include/qt6 にインストールされます。
            // Qt公式インストーラーを使用した場合、ユーザーのホームディレクトリや /opt にインストールされることが多いです。
            "/usr/include".to_string() // 一般的なLinuxのシステムインストールパスの基点
        });

    let qt_include_path = PathBuf::from(&qt_dir).join("include");

    // Qt6の主要なモジュールのインクルードパスを追加
    let qt_core_include = qt_include_path.join("QtCore");
    let qt_gui_include = qt_include_path.join("QtGui");
    let qt_widgets_include = qt_include_path.join("QtWidgets");

    let mut builder = bindgen::Builder::default()
        .header("lib/qt6-bind/src/lib.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // QtのインクルードパスをClangに渡す
        .clang_arg(format!("-I{}", qt_include_path.display()))
        .clang_arg(format!("-I{}", qt_core_include.display()))
        .clang_arg(format!("-I{}", qt_gui_include.display()))
        .clang_arg(format!("-I{}", qt_widgets_include.display()));

    // QtのMOC (Meta-Object Compiler) が生成するヘッダーファイルのパスも追加
    // これらは通常、cmake::buildの出力ディレクトリ内に生成されます
    // `target/debug/build/linux-lx-dos-xxxxxxxxxxxx/out/include` のようなパス
    let cmake_build_out_include = dst.join("include");
    if cmake_build_out_include.exists() {
        builder = builder.clang_arg(format!("-I{}", cmake_build_out_include.display()));
        // 特定のモジュールのMOCヘッダーがサブディレクトリにある場合も考慮
        let cmake_build_out_qt6_bind_include = cmake_build_out_include.join("qt6-bind");
        if cmake_build_out_qt6_bind_include.exists() {
            builder = builder.clang_arg(format!("-I{}", cmake_build_out_qt6_bind_include.display()));
        }
        let cmake_build_out_qtcore_include = cmake_build_out_include.join("QtCore");
        if cmake_build_out_qtcore_include.exists() {
            builder = builder.clang_arg(format!("-I{}", cmake_build_out_qtcore_include.display()));
        }
        let cmake_build_out_qtgui_include = cmake_build_out_include.join("QtGui");
        if cmake_build_out_qtgui_include.exists() {
            builder = builder.clang_arg(format!("-I{}", cmake_build_out_qtgui_include.display()));
        }
        let cmake_build_out_qtwidgets_include = cmake_build_out_include.join("QtWidgets");
        if cmake_build_out_qtwidgets_include.exists() {
            builder = builder.clang_arg(format!("-I{}", cmake_build_out_qtwidgets_include.display()));
        }
    }

    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("qt6-bind.rs"))
        .expect("Couldn't write bindings!");
}
