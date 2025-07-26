// The bindgen-generated bindings will be included here.
// Make sure your build.rs correctly points to qt-app.hpp for binding generation.
// bindgenがqt-app.hppを処理して生成したバインディングをインクルード
// OUT_DIRはbuild.rsによって設定されます。
include!(concat!(env!("OUT_DIR"), "/qt6-bind.rs"));
