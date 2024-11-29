#[cfg(not(windows))]
fn main() {
  embed_build_data();
}

#[cfg(windows)]
fn main() {
  embed_build_data();
  println!("cargo:rerun-if-changed=assets/res/windows_icon.rc");
  // on windows, will set game icon as icon for the executable
  embed_resource::compile("assets/res/windows_icon.rc", embed_resource::NONE)
    .manifest_required()
    .unwrap();
}

fn embed_build_data() {
  build_data::set_GIT_BRANCH();
  build_data::set_GIT_COMMIT_SHORT();
  build_data::set_RUSTC_VERSION();
  build_data::set_GIT_COMMIT_SHORT();
  build_data::set_ASSETS_DIR();
}

#[allow(non_snake_case)]
mod build_data {
  pub use ::build_data::*;
  /// Set the `ASSETS_DIR` environment variable to the path of the asset directory.
  pub fn set_ASSETS_DIR() {
    // Set the assets directory
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = std::path::Path::new(&cargo_manifest_dir);
    let assets_dir = cargo_manifest_dir.join("assets");
    std::fs::create_dir_all(&assets_dir).unwrap();
    let assets_dir = assets_dir.canonicalize().unwrap().display().to_string();
    println!("cargo:rustc-env=ASSETS_DIR={assets_dir}");
  }
}
