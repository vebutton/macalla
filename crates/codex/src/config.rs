use std::path::{Path, PathBuf};

pub fn config_path() -> PathBuf {
    if let Some(codex_home) = std::env::var_os("CODEX_HOME").filter(|path| !path.is_empty()) {
        return PathBuf::from(codex_home).join("config.toml");
    }

    dirs::home_dir()
        .expect("could not determine home directory")
        .join(".codex")
        .join("config.toml")
}

pub fn read_config(path: &Path) -> Result<toml::Table, String> {
    match std::fs::read_to_string(path) {
        Ok(contents) => contents
            .parse::<toml::Table>()
            .map_err(|e| format!("failed to parse {}: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(toml::Table::new()),
        Err(e) => Err(format!("failed to read {}: {e}", path.display())),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::config_path;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn config_path_uses_codex_home_when_present() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let temp_dir = tempfile::tempdir().expect("tempdir");

        // SAFETY: tests serialize environment mutation with ENV_LOCK.
        unsafe {
            std::env::set_var("CODEX_HOME", temp_dir.path());
        }

        assert_eq!(config_path(), temp_dir.path().join("config.toml"));

        // SAFETY: tests serialize environment mutation with ENV_LOCK.
        unsafe {
            std::env::remove_var("CODEX_HOME");
        }
    }
}
