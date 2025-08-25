use resvg::tiny_skia;
use resvg::usvg;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Re-run the build script if icon.svg changes.
    println!("cargo:rerun-if-changed=public/icon.svg");

    // Get the OUT_DIR path.
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    // List of icon sizes to generate.
    let sizes = [16, 24, 32, 48, 64, 128, 256, 512];

    // Read and parse the SVG file.
    let svg_data = fs::read("public/icon.svg").expect("Failed to read SVG file");
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_data(&svg_data, &opt).expect("Failed to parse SVG");

    // Get the original SVG size.
    let original_size = rtree.size();

    for size in sizes.iter() {
        let size_str = format!("{}x{}", size, size);
        let pixmap_size = tiny_skia::Size::from_wh(*size as f32, *size as f32).unwrap();

        // Create a mutable Pixmap for rendering.
        let mut pixmap =
            tiny_skia::Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
                .expect("Failed to create pixmap");

        // Calculate the scaling transform to fit the SVG within the target size,
        // while maintaining the aspect ratio.
        let scale_x = *size as f32 / original_size.width();
        let scale_y = *size as f32 / original_size.height();
        let scale = scale_x.min(scale_y);
        let transform = tiny_skia::Transform::from_scale(scale, scale);

        // Render the SVG to the pixmap with the calculated transform.
        resvg::render(
            &rtree,
            transform,
            &mut pixmap.as_mut(),
        );

        // Save the PNG to OUT_DIR.
        let png_path = out_dir_path.join(format!("{}.png", size_str));
        pixmap.save_png(&png_path).expect("Failed to save PNG");
        println!("Generated {}", png_path.display());
    }
}
