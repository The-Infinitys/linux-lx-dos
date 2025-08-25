use resvg::tiny_skia;
use resvg::usvg;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=public/icon.svg");

    // OUT_DIRパスを取得
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);

    // アイコンのサイズリスト
    let sizes = [16, 24, 32, 48, 64, 128, 256, 512];

    // SVGを読み込み
    let svg_data = fs::read("public/icon.svg").expect("Failed to read SVG file");
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(&svg_data, &opt).expect("Failed to parse SVG");

    for size in sizes.iter() {
        let size_str = format!("{}x{}", size, size);
        let pixmap_size = tiny_skia::Size::from_wh(*size as f32, *size as f32).unwrap();

        // Pixmapを可変（mutable）で作成
        let mut pixmap =
            tiny_skia::Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
                .unwrap();

        // resvg::renderに可変参照を渡してレンダリング
        resvg::render(
            &rtree,
            tiny_skia::Transform::identity(),
            &mut pixmap.as_mut(),
        );

        // OUT_DIRにPNGを保存
        let png_path = out_dir_path.join(format!("{}.png", size_str));
        pixmap.save_png(&png_path).expect("Failed to save PNG");
    }
}
