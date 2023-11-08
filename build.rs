extern crate cmake;
use cmake::Config;

fn main()
{
    // Build libhello for the target
    let dst = Config::new("libhello").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=hello");

    // Get the git details for this project at build time
    let mut options = built::Options::default();
    options.set_git(true);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(out_dir.as_str());
    let built_file = out_dir.join("built.rs");

    built::write_built_file_with_opts(&options, std::path::Path::new(env!("CARGO_MANIFEST_DIR")), &built_file)
        .expect("Could not retrieve build time info");
}