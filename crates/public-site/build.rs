use std::{env, fs, path::Path};

fn collect(root: &Path, dir: &Path, output: &mut Vec<(String, String)>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .expect("read asset directory")
        .map(|entry| entry.expect("read asset entry").path())
        .collect();
    entries.sort();
    for path in entries {
        assert!(
            !path.is_symlink(),
            "asset symlinks are not supported: {}",
            path.display()
        );
        if path.is_dir() {
            collect(root, &path, output);
        } else if path.is_file() {
            let name = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .expect("UTF-8 asset path")
                .replace('\\', "/");
            output.push((
                format!("/{name}"),
                path.canonicalize().unwrap().to_str().unwrap().to_owned(),
            ));
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=assets");
    let root = Path::new("assets");
    let mut assets = Vec::new();
    collect(root, root, &mut assets);
    let mut generated = String::from("pub const EMBEDDED_ASSETS: &[(&str, &[u8])] = &[\n");
    for (url, path) in assets {
        generated.push_str(&format!("({url:?}, include_bytes!({path:?})),\n"));
    }
    generated.push_str("];\n");
    fs::write(
        Path::new(&env::var_os("OUT_DIR").unwrap()).join("embedded_assets.rs"),
        generated,
    )
    .unwrap();
}
