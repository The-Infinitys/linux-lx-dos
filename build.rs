use slint_build::compile;
fn main() {
    compile("ui/main.slint").expect("Slint compile error");
}
