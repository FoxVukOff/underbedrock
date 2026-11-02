//! `modbuild <mod folder> <out.hmod>`: run Horizon's own content checks over a
//! content mod's folder and pack it as a `.hmod`, exactly as
//! `cargo horizon-mod build` does (crates/cargo-horizon-mod/src/main.rs).
use horizon_mods::{Content, Manifest, Package, manifest::MANIFEST};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (root, out) = match (args.first(), args.get(1)) {
        (Some(a), Some(b)) => (Path::new(a), b.clone()),
        _ => {
            eprintln!("usage: modbuild <mod folder> <out.hmod>");
            std::process::exit(2);
        }
    };
    let manifest_text = match std::fs::read_to_string(root.join(MANIFEST)) {
        Ok(t) => t,
        Err(e) => fail(format!("{MANIFEST}: {e}")),
    };
    let manifest = match Manifest::parse(&manifest_text) {
        Ok(m) => m,
        Err(e) => fail(format!("manifest: {e}")),
    };
    // The same checks as the game's when it reads the content.
    let content = match Content::read(root, &manifest) {
        Ok(c) => c,
        Err(e) => fail(format!("content: {e}")),
    };
    let mut reader = match content.reader() {
        Ok(r) => r,
        Err(e) => fail(format!("reader: {e}")),
    };
    let mut files = Vec::with_capacity(content.files.len());
    for name in &content.files {
        match reader.read(name) {
            Ok(bytes) => files.push((name.clone(), bytes)),
            Err(e) => fail(format!("{name}: {e}")),
        }
    }
    let bytes: Vec<(&str, &[u8])> = files.iter().map(|(n, b)| (n.as_str(), b.as_slice())).collect();
    let packed = match Package::write_files(&manifest_text, None, &bytes) {
        Ok(p) => p,
        Err(e) => fail(format!("pack: {e}")),
    };
    if let Err(e) = std::fs::write(&out, &packed) {
        fail(format!("{out}: {e}"));
    }
    println!("ok: {} content files, {} blocks, {} items, {} tabs",
        content.files.len(), content.blocks.len(), content.items.len(), content.creative_tabs.len());
    println!("namespaces: {:?}", content.namespaces);
    println!("wrote {out} ({} bytes)", packed.len());
}

fn fail(message: String) -> ! {
    eprintln!("error: {message}");
    std::process::exit(1);
}
