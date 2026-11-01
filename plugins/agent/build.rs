const COMMANDS: &[&str] = &["health_check"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
