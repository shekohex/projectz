#[cfg(not(windows))]
fn main() {
  embed_build_data();
}

#[cfg(windows)]
fn main() {
  embed_build_data();
  println!("cargo:rerun-if-changed=assets/res/windows_icon.rc");
  // on windows, we will set our game icon as icon for the executable
  embed_resource::compile("assets/res/windows_icon.rc", embed_resource::NONE)
    .manifest_required()
    .unwrap();
}

fn embed_build_data() {
  build_data::set_GIT_BRANCH();
  build_data::set_GIT_COMMIT_SHORT();
  build_data::set_RUSTC_VERSION();
}
