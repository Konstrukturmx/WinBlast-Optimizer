//! Export/import user tweak configuration profiles (`.wb0` files).

use std::path::Path;
use serde::{Deserialize, Serialize};

/// A user's saved tweak profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakProfile {
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub winblast_version: String,
    pub selected_tweak_ids: Vec<String>,
}

impl TweakProfile {
    /// Create a new profile with the given metadata and tweak IDs.
    pub fn new(name: String, description: String, selected_ids: Vec<String>) -> Self {
        Self {
            name,
            description,
            created_at: chrono::Utc::now().to_rfc3339(),
            winblast_version: env!("CARGO_PKG_VERSION").to_string(),
            selected_tweak_ids: selected_ids,
        }
    }
}

/// Export the current tweak selection to a `.wb0` profile file.
pub fn export_profile(
    path: &Path,
    selected_ids: &[String],
) -> crate::core::error::Result<()> {
    let profile = TweakProfile::new(
        "WinBlast0 Custom Profile".to_string(),
        "Exported user tweak profile".to_string(),
        selected_ids.to_vec(),
    );
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(crate::core::error::WinBlastError::Registry)?;
        }
    }
    let json = serde_json::to_string_pretty(&profile)?;
    std::fs::write(path, json).map_err(crate::core::error::WinBlastError::Registry)?;
    tracing::info!("Profile exported to {:?}", path);
    Ok(())
}

/// Import a `.wb0` profile file and return the selected tweak IDs.
pub fn import_profile(
    path: &Path,
) -> crate::core::error::Result<Vec<String>> {
    let content = std::fs::read_to_string(path).map_err(crate::core::error::WinBlastError::Registry)?;
    let profile: TweakProfile = serde_json::from_str(&content)?;
    tracing::info!("Profile imported from {:?}: {}", path, profile.name);
    Ok(profile.selected_tweak_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_export_import() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(format!("test_profile_{}.wb0", uuid::Uuid::new_v4()));

        let tweaks = vec!["tweak_a".to_string(), "tweak_b".to_string()];
        export_profile(&path, &tweaks).expect("Failed to export profile");

        let imported = import_profile(&path).expect("Failed to import profile");
        assert_eq!(imported, tweaks);

        let nested_dir = temp_dir.join(format!("winblast_prof_dir_{}", uuid::Uuid::new_v4()));
        let nested_path = nested_dir.join("sub").join("nested.wb0");
        export_profile(&nested_path, &tweaks).expect("Failed to create parent dirs on export");
        assert_eq!(import_profile(&nested_path).unwrap(), tweaks);

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir_all(nested_dir);
    }
}
