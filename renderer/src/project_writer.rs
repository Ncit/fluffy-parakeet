use std::fs;
use std::path::Path;

pub fn create_project_skeleton(path: impl AsRef<Path>) -> Result<(), String> {
    let root = path.as_ref();
    fs::create_dir_all(root.join("assets")).map_err(|error| format!("failed to create assets dir: {error}"))?;
    fs::write(root.join("project.json"), "{\n  \"version\": 1,\n  \"scene\": \"scene.json\",\n  \"assets_dir\": \"assets\"\n}\n")
        .map_err(|error| format!("failed to write project manifest: {error}"))?;
    fs::write(root.join("scene.json"), "{\n  \"version\": 3,\n  \"duration\": 3.0,\n  \"nodes\": []\n}\n")
        .map_err(|error| format!("failed to write scene: {error}"))?;
    Ok(())
}
