// Dependency-free native harness for comparing all paintings with read-only
// kytos source. Not linked into the WebAssembly island.
#[allow(dead_code)]
#[path = "../../src/paint.rs"]
mod paint;

fn main() {
    for (palette, seed) in paint::LANDSCAPES {
        println!("{}", paint::paint(palette, *seed));
    }
}
