use std::fs;
use std::path::Path;

use crate::scene::{Scene, SceneLoadError};

pub fn load_scene_from_path(path: impl AsRef<Path>) -> Result<Scene, SceneLoadError> {
    let json = fs::read_to_string(path).map_err(|error| {
        SceneLoadError::Validation(vec![format!("failed to read scene file: {error}")])
    })?;
    Scene::from_dsl_json(&json)
}
