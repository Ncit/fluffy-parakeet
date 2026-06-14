use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ProjectManifest {
    pub version: u32,
    pub scene: String,
    pub assets_dir: String,
}

#[derive(Clone, Debug)]
pub struct Project {
    pub root: PathBuf,
    pub manifest: ProjectManifest,
}

impl Project {
    pub fn scene_path(&self) -> PathBuf {
        self.root.join(&self.manifest.scene)
    }

    pub fn assets_path(&self) -> PathBuf {
        self.root.join(&self.manifest.assets_dir)
    }
}

pub fn load_project(path: impl AsRef<Path>) -> Result<Project, String> {
    let root = path.as_ref().to_path_buf();
    let manifest_path = root.join("project.json");
    let json = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("failed to read project manifest: {error}"))?;
    let manifest: ProjectManifest = serde_json::from_str(&json)
        .map_err(|error| format!("failed to parse project manifest: {error}"))?;

    if manifest.version != 1 {
        return Err(format!("unsupported project version {}", manifest.version));
    }

    Ok(Project { root, manifest })
}
