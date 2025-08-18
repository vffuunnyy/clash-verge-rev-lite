fn main() {
    tauri_build::build()
}

pub fn is_dev() -> bool {
    std::env::var("DEP_TAURI_DEV").expect("missing `cargo:dev` instruction, please update tauri to latest")
        == "true"
}