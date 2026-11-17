use std::path::{Path, PathBuf};

use directories::{BaseDirs, ProjectDirs};

use crate::meta::NAME;

pub struct Directory {}

impl Directory {
    pub fn home_dir() -> Option<PathBuf> {
        BaseDirs::new().map(|d| PathBuf::from(d.home_dir()))
    }

    #[cfg(not(feature = "portable"))]
    pub fn project_dirs() -> Option<ProjectDirs> {
        ProjectDirs::from("dev", "antares", NAME)
    }

    #[cfg(not(feature = "portable"))]
    pub fn legacy_project_dirs() -> Option<ProjectDirs> {
        let legacy_name = NAME.replacen("Antares", "Lapce", 1);
        ProjectDirs::from("dev", "lapce", &legacy_name)
    }

    /// Return path adjacent to antares executable when built as portable
    #[cfg(feature = "portable")]
    pub fn project_dirs() -> Option<ProjectDirs> {
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                let antares_data = parent.join("antares-data");
                if !antares_data.exists() {
                    let lapce_data = parent.join("lapce-data");
                    if lapce_data.exists() {
                        let _ = Self::migrate_directory_content(&lapce_data, &antares_data);
                    }
                }
                return ProjectDirs::from_path(antares_data);
            }
            unreachable!("Couldn't obtain current process parent path");
        }
        unreachable!("Couldn't obtain current process path");
    }

    /// Recursively migrate contents from a legacy directory to an Antares directory.
    /// Non-destructive: copies files and subdirectories without deleting source files.
    /// Idempotent: existing files in the target directory are never overwritten.
    pub fn migrate_directory_content(source: &Path, target: &Path) -> std::io::Result<usize> {
        if !source.exists() || !source.is_dir() {
            return Ok(0);
        }

        std::fs::create_dir_all(target)?;
        let mut count = 0;

        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let target_path = target.join(entry.file_name());

            if source_path.is_dir() {
                count += Self::migrate_directory_content(&source_path, &target_path)?;
            } else if source_path.is_file() {
                if !target_path.exists() {
                    if let Err(err) = std::fs::copy(&source_path, &target_path) {
                        tracing::warn!(
                            "Failed to copy {:?} to {:?}: {:?}",
                            source_path,
                            target_path,
                            err
                        );
                    } else {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    // Get path of local data directory
    // Local data directory differs from data directory
    // on some platforms and is not transferred across
    // machines
    pub fn data_local_directory() -> Option<PathBuf> {
        match Self::project_dirs() {
            Some(dir) => {
                let local_dir = dir.data_local_dir();
                if !local_dir.exists() {
                    #[cfg(not(feature = "portable"))]
                    if let Some(legacy) = Self::legacy_project_dirs() {
                        let legacy_local = legacy.data_local_dir();
                        if legacy_local.exists() {
                            let _ = Self::migrate_directory_content(legacy_local, local_dir);
                        }
                    }
                    if let Err(err) = std::fs::create_dir_all(local_dir) {
                        tracing::error!("{:?}", err);
                    }
                }
                Some(local_dir.to_path_buf())
            }
            None => None,
        }
    }

    /// Get the path to logs directory
    /// Each log file is for individual application startup
    pub fn logs_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("logs");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }
            Some(dir)
        } else {
            None
        }
    }

    /// Get the path to cache directory
    pub fn cache_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("cache");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }
            Some(dir)
        } else {
            None
        }
    }

    /// Directory to store proxy executables used on local
    /// host as well, as ones uploaded to remote host when
    /// connecting
    pub fn proxy_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("proxy");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }
            Some(dir)
        } else {
            None
        }
    }
    /// Get the path to the themes folder
    /// Themes are stored within as individual toml files
    pub fn themes_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("themes");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }
            Some(dir)
        } else {
            None
        }
    }
    // Get the path to plugins directory
    // Each plugin has own directory that contains
    // metadata file and plugin wasm
    pub fn plugins_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("plugins");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }
            Some(dir)
        } else {
            None
        }
    }

    // Config directory contain only configuration files
    pub fn config_directory() -> Option<PathBuf> {
        match Self::project_dirs() {
            Some(dir) => {
                let config_dir = dir.config_dir();
                if !config_dir.exists() {
                    #[cfg(not(feature = "portable"))]
                    if let Some(legacy) = Self::legacy_project_dirs() {
                        let legacy_config = legacy.config_dir();
                        if legacy_config.exists() {
                            let _ = Self::migrate_directory_content(legacy_config, config_dir);
                        }
                    }
                    if let Err(err) = std::fs::create_dir_all(config_dir) {
                        tracing::error!("{:?}", err);
                    }
                }
                Some(config_dir.to_path_buf())
            }
            None => None,
        }
    }

    pub fn local_socket() -> Option<PathBuf> {
        Self::data_local_directory().map(|dir| dir.join("local.sock"))
    }

    pub fn updates_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("updates");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }
            Some(dir)
        } else {
            None
        }
    }

    pub fn queries_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::config_directory() {
            let dir = dir.join("queries");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }

            Some(dir)
        } else {
            None
        }
    }

    pub fn grammars_directory() -> Option<PathBuf> {
        if let Some(dir) = Self::data_local_directory() {
            let dir = dir.join("grammars");
            if !dir.exists() {
                if let Err(err) = std::fs::create_dir(&dir) {
                    tracing::error!("{:?}", err);
                }
            }

            Some(dir)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::Directory;

    #[test]
    fn test_migrate_directory_content_basic() {
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(12345);
        let temp = std::env::temp_dir().join(format!("antares_test_migrate_basic_{unique_id}"));
        let source = temp.join("lapce-legacy");
        let target = temp.join("antares-new");

        fs::create_dir_all(source.join("themes")).unwrap();
        fs::write(source.join("settings.toml"), "font-size = 14\n").unwrap();
        fs::write(source.join("themes").join("custom.toml"), "[color-theme]\nname = \"custom\"\n").unwrap();

        let migrated = Directory::migrate_directory_content(&source, &target).unwrap();
        assert_eq!(migrated, 2);

        // Verify target has the files
        assert!(target.join("settings.toml").exists());
        assert_eq!(fs::read_to_string(target.join("settings.toml")).unwrap(), "font-size = 14\n");
        assert!(target.join("themes").join("custom.toml").exists());

        // Verify non-destructive: source still has the files
        assert!(source.join("settings.toml").exists());
        assert!(source.join("themes").join("custom.toml").exists());

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_migrate_directory_content_idempotent() {
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(23456);
        let temp = std::env::temp_dir().join(format!("antares_test_migrate_idempotent_{unique_id}"));
        let source = temp.join("lapce-legacy");
        let target = temp.join("antares-new");

        fs::create_dir_all(&source).unwrap();
        fs::create_dir_all(&target).unwrap();

        fs::write(source.join("settings.toml"), "font-size = 12\n").unwrap();
        fs::write(target.join("settings.toml"), "font-size = 16\n").unwrap();

        let migrated = Directory::migrate_directory_content(&source, &target).unwrap();
        assert_eq!(migrated, 0);

        // Existing target file preserved
        assert_eq!(fs::read_to_string(target.join("settings.toml")).unwrap(), "font-size = 16\n");

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_migrate_directory_content_missing_source() {
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(34567);
        let temp = std::env::temp_dir().join(format!("antares_test_migrate_missing_{unique_id}"));
        let source = temp.join("does-not-exist");
        let target = temp.join("antares-new");

        let migrated = Directory::migrate_directory_content(&source, &target).unwrap();
        assert_eq!(migrated, 0);
    }
}
