use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::Deserialize;

use crate::uniforms::NodeUniform;

pub const SUPPORTED_DSL_VERSION: u32 = 3;

#[derive(Debug)]
pub enum SceneLoadError { Parse(serde_json::Error), Validation(Vec<String>) }
impl fmt::Display for SceneLoadError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { match self { Self::Parse(error) => write!(f, "failed to parse scene DSL: {error}"), Self::Validation(errors) => write!(f, "scene validation failed: {}", errors.join("; ")) } } }
impl std::error::Error for SceneLoadError {}
impl From<serde_json::Error> for SceneLoadError { fn from(error: serde_json::Error) -> Self { Self::Parse(error) } }

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Easing { Linear, EaseIn, EaseOut, EaseInOut }
impl Default for Easing { fn default() -> Self { Self::Linear } }
impl Easing { pub fn apply(self, t: f32) -> f32 { let t = t.clamp(0.0, 1.0); match self { Self::Linear => t, Self::EaseIn => t * t, Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t), Self::EaseInOut => if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 } } } }

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Keyframe { pub time: f32, pub value: f32, #[serde(default)] pub easing: Easing }
#[derive(Clone, Debug, Deserialize)]
pub struct AnimatedValue { pub keyframes: Vec<Keyframe> }
impl AnimatedValue { pub fn constant(value: f32) -> Self { Self { keyframes: vec![Keyframe { time: 0.0, value, easing: Easing::Linear }] } } pub fn sample(&self, t: f32) -> f32 { if self.keyframes.is_empty() { return 0.0; } if t <= self.keyframes[0].time { return self.keyframes[0].value; } for pair in self.keyframes.windows(2) { let a = pair[0]; let b = pair[1]; if t >= a.time && t <= b.time { let duration = b.time - a.time; if duration.abs() < f32::EPSILON { return b.value; } return a.value + (b.value - a.value) * b.easing.apply((t - a.time) / duration); } } self.keyframes.last().unwrap().value } }

#[derive(Clone, Debug, Default, Deserialize)]
struct AnimationPreset { x: Option<Vec<Keyframe>>, y: Option<Vec<Keyframe>>, scale_x: Option<Vec<Keyframe>>, scale_y: Option<Vec<Keyframe>>, rotation: Option<Vec<Keyframe>>, opacity: Option<Vec<Keyframe>> }
impl AnimationPreset { fn apply_to(&self, node: &mut DslNode) { if let Some(value) = &self.x { node.x = value.clone(); } if let Some(value) = &self.y { node.y = value.clone(); } if let Some(value) = &self.scale_x { node.scale_x = value.clone(); } if let Some(value) = &self.scale_y { node.scale_y = value.clone(); } if let Some(value) = &self.rotation { node.rotation = value.clone(); } if let Some(value) = &self.opacity { node.opacity = value.clone(); } } }

#[derive(Clone, Debug, Default, Deserialize)]
struct Style { color: Option<[f32; 4]>, width: Option<f32>, height: Option<f32>, anchor_x: Option<f32>, anchor_y: Option<f32> }
impl Style { fn apply_to(&self, node: &mut DslNode) { if let Some(value) = self.color { node.color = value; } if let Some(value) = self.width { node.width = value; } if let Some(value) = self.height { node.height = value; } if let Some(value) = self.anchor_x { node.anchor_x = value; } if let Some(value) = self.anchor_y { node.anchor_y = value; } } }

#[derive(Clone, Debug, Deserialize)]
pub struct Asset { #[serde(rename = "type")] pub kind: String, pub src: String }
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign { Left, Center, Right }
impl Default for TextAlign { fn default() -> Self { Self::Left } }
#[derive(Clone, Debug)]
pub struct TextProperties { pub text: String, pub font_size: f32, pub font_family: String, pub font_weight: u32, pub align: TextAlign, pub line_height: f32 }

#[derive(Clone, Copy, Debug)]
pub struct SampledNode { pub uniform: NodeUniform }
#[derive(Clone, Debug, Deserialize)]
pub struct SelectionMetadata { #[serde(default)] pub name: String, #[serde(default)] pub locked: bool, #[serde(default = "default_selectable")] pub selectable: bool, #[serde(default)] pub tags: Vec<String> }
impl Default for SelectionMetadata { fn default() -> Self { Self { name: String::new(), locked: false, selectable: true, tags: Vec::new() } } }

#[derive(Clone, Debug)]
pub struct SceneNode { pub id: String, pub parent_id: Option<String>, pub kind: String, pub asset: Option<String>, pub text: Option<TextProperties>, pub layer: i32, pub start_time: f32, pub end_time: Option<f32>, pub selection: SelectionMetadata, pub color: [f32; 4], pub width: f32, pub height: f32, pub anchor_x: f32, pub anchor_y: f32, pub x: AnimatedValue, pub y: AnimatedValue, pub scale_x: AnimatedValue, pub scale_y: AnimatedValue, pub rotation: AnimatedValue, pub opacity: AnimatedValue }
impl SceneNode { pub fn is_active(&self, t: f32) -> bool { t >= self.start_time && self.end_time.map_or(true, |end_time| t <= end_time) } pub fn display_name(&self) -> &str { if self.selection.name.is_empty() { &self.id } else { &self.selection.name } } fn local_uniform(&self, t: f32) -> NodeUniform { let local_t = (t - self.start_time).max(0.0); let scale_x = self.width * self.scale_x.sample(local_t); let scale_y = self.height * self.scale_y.sample(local_t); NodeUniform { offset: [self.x.sample(local_t) + (0.5 - self.anchor_x) * scale_x, self.y.sample(local_t) + (0.5 - self.anchor_y) * scale_y], scale: [scale_x, scale_y], rotation: self.rotation.sample(local_t), opacity: self.opacity.sample(local_t), _padding: [0.0, 0.0], color: self.color } } pub fn sample(&self, t: f32) -> SampledNode { SampledNode { uniform: self.local_uniform(t) } } }

#[derive(Clone, Debug)]
pub struct Scene { pub version: u32, pub duration: f32, pub fps: f32, pub width: u32, pub height: u32, pub background: [f32; 4], pub assets: HashMap<String, Asset>, pub nodes: Vec<SceneNode> }

#[derive(Debug, Deserialize)]
struct DslScene { version: u32, duration: f32, #[serde(default = "default_fps")] fps: f32, #[serde(default = "default_project_width")] width: u32, #[serde(default = "default_project_height")] height: u32, #[serde(default = "default_background")] background: [f32; 4], #[serde(default)] assets: HashMap<String, Asset>, #[serde(default)] styles: HashMap<String, Style>, #[serde(default)] presets: HashMap<String, AnimationPreset>, nodes: Vec<DslNode> }

#[derive(Debug, Deserialize)]
struct DslNode { id: String, #[serde(default)] parent_id: Option<String>, #[serde(rename = "type")] kind: String, #[serde(default)] asset: Option<String>, #[serde(default)] style: Option<String>, #[serde(default)] preset: Option<String>, #[serde(default)] text: String, #[serde(default = "default_font_size")] font_size: f32, #[serde(default = "default_font_family")] font_family: String, #[serde(default = "default_font_weight")] font_weight: u32, #[serde(default)] align: TextAlign, #[serde(default = "default_line_height")] line_height: f32, #[serde(default)] layer: i32, #[serde(default)] start_time: f32, #[serde(default)] end_time: Option<f32>, #[serde(default)] selection: SelectionMetadata, #[serde(default = "default_color")] color: [f32; 4], #[serde(default = "default_width")] width: f32, #[serde(default = "default_height")] height: f32, #[serde(default = "default_anchor")] anchor_x: f32, #[serde(default = "default_anchor")] anchor_y: f32, #[serde(default = "default_zero_keyframes")] x: Vec<Keyframe>, #[serde(default = "default_zero_keyframes")] y: Vec<Keyframe>, #[serde(default = "default_one_keyframes")] scale_x: Vec<Keyframe>, #[serde(default = "default_one_keyframes")] scale_y: Vec<Keyframe>, #[serde(default = "default_rotation")] rotation: Vec<Keyframe>, #[serde(default = "default_opacity")] opacity: Vec<Keyframe> }

fn default_color() -> [f32; 4] { [0.2, 0.8, 1.0, 1.0] }
fn default_width() -> f32 { 1.0 }
fn default_height() -> f32 { 1.0 }
fn default_anchor() -> f32 { 0.5 }
fn default_selectable() -> bool { true }
fn default_fps() -> f32 { 60.0 }
fn default_project_width() -> u32 { 1920 }
fn default_project_height() -> u32 { 1080 }
fn default_background() -> [f32; 4] { [0.1, 0.2, 0.3, 1.0] }
fn default_font_size() -> f32 { 48.0 }
fn default_font_family() -> String { "Inter".to_string() }
fn default_font_weight() -> u32 { 400 }
fn default_line_height() -> f32 { 1.2 }
fn default_zero_keyframes() -> Vec<Keyframe> { vec![Keyframe { time: 0.0, value: 0.0, easing: Easing::Linear }] }
fn default_one_keyframes() -> Vec<Keyframe> { vec![Keyframe { time: 0.0, value: 1.0, easing: Easing::Linear }] }
fn default_rotation() -> Vec<Keyframe> { default_zero_keyframes() }
fn default_opacity() -> Vec<Keyframe> { default_one_keyframes() }

impl Scene { pub fn node_by_id(&self, id: &str) -> Option<&SceneNode> { self.nodes.iter().find(|node| node.id == id) } pub fn selectable_nodes(&self) -> impl Iterator<Item = &SceneNode> { self.nodes.iter().filter(|node| node.selection.selectable && !node.selection.locked) } pub fn asset_by_id(&self, id: &str) -> Option<&Asset> { self.assets.get(id) } pub fn sample_node(&self, index: usize, t: f32) -> SampledNode { let mut uniform = self.nodes[index].local_uniform(t); if let Some(parent_id) = &self.nodes[index].parent_id { if let Some(parent_index) = self.nodes.iter().position(|node| &node.id == parent_id) { let parent = self.sample_node(parent_index, t).uniform; uniform.offset[0] += parent.offset[0]; uniform.offset[1] += parent.offset[1]; uniform.rotation += parent.rotation; uniform.opacity *= parent.opacity; uniform.color[3] *= parent.color[3]; } } SampledNode { uniform } }

    pub fn from_dsl_json(json: &str) -> Result<Self, SceneLoadError> { let dsl: DslScene = serde_json::from_str(json)?; validate_references(&dsl)?; let mut nodes: Vec<SceneNode> = dsl.nodes.into_iter().map(|mut node| { if let Some(style_name) = &node.style { if let Some(style) = dsl.styles.get(style_name) { style.apply_to(&mut node); } } if let Some(preset_name) = &node.preset { if let Some(preset) = dsl.presets.get(preset_name) { preset.apply_to(&mut node); } } let text = if node.kind == "text" { Some(TextProperties { text: node.text, font_size: node.font_size, font_family: node.font_family, font_weight: node.font_weight, align: node.align, line_height: node.line_height }) } else { None }; SceneNode { id: node.id, parent_id: node.parent_id, kind: node.kind, asset: node.asset, text, layer: node.layer, start_time: node.start_time, end_time: node.end_time, selection: node.selection, color: node.color, width: node.width, height: node.height, anchor_x: node.anchor_x, anchor_y: node.anchor_y, x: AnimatedValue { keyframes: node.x }, y: AnimatedValue { keyframes: node.y }, scale_x: AnimatedValue { keyframes: node.scale_x }, scale_y: AnimatedValue { keyframes: node.scale_y }, rotation: AnimatedValue { keyframes: node.rotation }, opacity: AnimatedValue { keyframes: node.opacity } } }).collect(); validate_scene(dsl.version, dsl.duration, dsl.fps, dsl.width, dsl.height, &nodes)?; nodes.sort_by_key(|node| node.layer); Ok(Self { version: dsl.version, duration: dsl.duration, fps: dsl.fps, width: dsl.width, height: dsl.height, background: dsl.background, assets: dsl.assets, nodes }) }
    pub fn demo() -> Self { Self::from_dsl_json(include_str!("../../ai/example_scene.json")).expect("demo DSL scene should parse") }
}

fn validate_references(dsl: &DslScene) -> Result<(), SceneLoadError> { let mut errors = Vec::new(); for node in &dsl.nodes { if let Some(preset_name) = &node.preset { if !dsl.presets.contains_key(preset_name) { errors.push(format!("node '{}' references missing preset '{}'", node.id, preset_name)); } } if let Some(style_name) = &node.style { if !dsl.styles.contains_key(style_name) { errors.push(format!("node '{}' references missing style '{}'", node.id, style_name)); } } if let Some(asset_name) = &node.asset { if !dsl.assets.contains_key(asset_name) { errors.push(format!("node '{}' references missing asset '{}'", node.id, asset_name)); } } } if errors.is_empty() { Ok(()) } else { Err(SceneLoadError::Validation(errors)) } }
fn validate_scene(version: u32, duration: f32, fps: f32, width: u32, height: u32, nodes: &[SceneNode]) -> Result<(), SceneLoadError> { let mut errors = Vec::new(); if version != SUPPORTED_DSL_VERSION { errors.push(format!("unsupported DSL version {version}; expected {SUPPORTED_DSL_VERSION}")); } if duration <= 0.0 { errors.push("duration must be greater than 0".to_string()); } if fps <= 0.0 { errors.push("fps must be greater than 0".to_string()); } if width == 0 || height == 0 { errors.push("project width and height must be positive".to_string()); } let mut ids = HashSet::new(); for (index, node) in nodes.iter().enumerate() { if node.id.trim().is_empty() { errors.push(format!("node {index} is missing id")); } if node.id.chars().any(char::is_whitespace) { errors.push(format!("node '{}' id cannot contain whitespace", node.id)); } if !matches!(node.kind.as_str(), "rect" | "text" | "image") { errors.push(format!("node '{}' has unsupported type '{}'", node.id, node.kind)); } if node.kind == "text" && node.text.as_ref().map_or(true, |text| text.text.is_empty()) { errors.push(format!("text node '{}' must include text", node.id)); } if node.width <= 0.0 || node.height <= 0.0 { errors.push(format!("node '{}' width and height must be positive", node.id)); } if let Some(end_time) = node.end_time { if end_time < node.start_time { errors.push(format!("node '{}' end_time must be >= start_time", node.id)); } } if !ids.insert(node.id.clone()) { errors.push(format!("duplicate node id '{}'", node.id)); } } for node in nodes { if let Some(parent_id) = &node.parent_id { if !ids.contains(parent_id) { errors.push(format!("node '{}' references missing parent_id '{}'", node.id, parent_id)); } if parent_id == &node.id { errors.push(format!("node '{}' cannot parent itself", node.id)); } } } if errors.is_empty() { Ok(()) } else { Err(SceneLoadError::Validation(errors)) } }

#[cfg(test)]
mod tests { use super::*; #[test] fn parses_text_node_fields() { let scene = Scene::from_dsl_json(r#"{"version":3,"duration":1,"nodes":[{"id":"title","type":"text","text":"Hello","font_size":64,"font_family":"Inter","font_weight":700,"align":"center","line_height":1.1}]}"#).unwrap(); let text = scene.nodes[0].text.as_ref().unwrap(); assert_eq!(text.text, "Hello"); assert_eq!(text.font_size, 64.0); assert_eq!(text.align, TextAlign::Center); } }