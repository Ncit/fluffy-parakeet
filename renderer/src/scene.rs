use serde::Deserialize;

use crate::uniforms::NodeUniform;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Default for Easing {
    fn default() -> Self {
        Self::Linear
    }
}

impl Easing {
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    #[serde(default)]
    pub easing: Easing,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AnimatedValue {
    pub keyframes: Vec<Keyframe>,
}

impl AnimatedValue {
    pub fn constant(value: f32) -> Self {
        Self { keyframes: vec![Keyframe { time: 0.0, value, easing: Easing::Linear }] }
    }

    pub fn sample(&self, t: f32) -> f32 {
        if self.keyframes.is_empty() { return 0.0; }
        if t <= self.keyframes[0].time { return self.keyframes[0].value; }
        for pair in self.keyframes.windows(2) {
            let a = pair[0]; let b = pair[1];
            if t >= a.time && t <= b.time {
                let duration = b.time - a.time;
                if duration.abs() < f32::EPSILON { return b.value; }
                let eased_t = b.easing.apply((t - a.time) / duration);
                return a.value + (b.value - a.value) * eased_t;
            }
        }
        self.keyframes.last().unwrap().value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SampledNode { pub uniform: NodeUniform }

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub kind: String,
    pub layer: i32,
    pub start_time: f32,
    pub end_time: Option<f32>,
    pub color: [f32; 4],
    pub width: f32,
    pub height: f32,
    pub anchor_x: f32,
    pub anchor_y: f32,
    pub x: AnimatedValue,
    pub y: AnimatedValue,
    pub scale_x: AnimatedValue,
    pub scale_y: AnimatedValue,
    pub rotation: AnimatedValue,
    pub opacity: AnimatedValue,
}

impl SceneNode {
    pub fn is_active(&self, t: f32) -> bool {
        t >= self.start_time && self.end_time.map_or(true, |end_time| t <= end_time)
    }

    fn local_uniform(&self, t: f32) -> NodeUniform {
        let local_t = (t - self.start_time).max(0.0);
        let scale_x = self.width * self.scale_x.sample(local_t);
        let scale_y = self.height * self.scale_y.sample(local_t);
        NodeUniform {
            offset: [self.x.sample(local_t) + (0.5 - self.anchor_x) * scale_x, self.y.sample(local_t) + (0.5 - self.anchor_y) * scale_y],
            scale: [scale_x, scale_y],
            rotation: self.rotation.sample(local_t),
            opacity: self.opacity.sample(local_t),
            _padding: [0.0, 0.0],
            color: self.color,
        }
    }

    pub fn sample(&self, t: f32) -> SampledNode {
        SampledNode { uniform: self.local_uniform(t) }
    }
}

#[derive(Clone, Debug)]
pub struct Scene { pub duration: f32, pub nodes: Vec<SceneNode> }

#[derive(Debug, Deserialize)]
struct DslScene { duration: f32, nodes: Vec<DslNode> }

#[derive(Debug, Deserialize)]
struct DslNode {
    #[serde(default)] id: String,
    #[serde(default)] parent_id: Option<String>,
    #[serde(rename = "type")] kind: String,
    #[serde(default)] layer: i32,
    #[serde(default)] start_time: f32,
    #[serde(default)] end_time: Option<f32>,
    #[serde(default = "default_color")] color: [f32; 4],
    #[serde(default = "default_width")] width: f32,
    #[serde(default = "default_height")] height: f32,
    #[serde(default = "default_anchor")] anchor_x: f32,
    #[serde(default = "default_anchor")] anchor_y: f32,
    x: Vec<Keyframe>,
    y: Vec<Keyframe>,
    scale_x: Vec<Keyframe>,
    scale_y: Vec<Keyframe>,
    #[serde(default = "default_rotation")] rotation: Vec<Keyframe>,
    #[serde(default = "default_opacity")] opacity: Vec<Keyframe>,
}

fn default_color() -> [f32; 4] { [0.2, 0.8, 1.0, 1.0] }
fn default_width() -> f32 { 1.0 }
fn default_height() -> f32 { 1.0 }
fn default_anchor() -> f32 { 0.5 }
fn default_rotation() -> Vec<Keyframe> { vec![Keyframe { time: 0.0, value: 0.0, easing: Easing::Linear }] }
fn default_opacity() -> Vec<Keyframe> { vec![Keyframe { time: 0.0, value: 1.0, easing: Easing::Linear }] }

impl Scene {
    pub fn sample_node(&self, index: usize, t: f32) -> SampledNode {
        let mut uniform = self.nodes[index].local_uniform(t);
        if let Some(parent_id) = &self.nodes[index].parent_id {
            if let Some(parent_index) = self.nodes.iter().position(|node| &node.id == parent_id) {
                let parent = self.sample_node(parent_index, t).uniform;
                uniform.offset[0] += parent.offset[0];
                uniform.offset[1] += parent.offset[1];
                uniform.rotation += parent.rotation;
                uniform.opacity *= parent.opacity;
                uniform.color[3] *= parent.color[3];
            }
        }
        SampledNode { uniform }
    }

    pub fn from_dsl_json(json: &str) -> Result<Self, serde_json::Error> {
        let dsl: DslScene = serde_json::from_str(json)?;
        let mut nodes: Vec<SceneNode> = dsl.nodes.into_iter().map(|node| SceneNode {
            id: node.id,
            parent_id: node.parent_id,
            kind: node.kind,
            layer: node.layer,
            start_time: node.start_time,
            end_time: node.end_time,
            color: node.color,
            width: node.width,
            height: node.height,
            anchor_x: node.anchor_x,
            anchor_y: node.anchor_y,
            x: AnimatedValue { keyframes: node.x },
            y: AnimatedValue { keyframes: node.y },
            scale_x: AnimatedValue { keyframes: node.scale_x },
            scale_y: AnimatedValue { keyframes: node.scale_y },
            rotation: AnimatedValue { keyframes: node.rotation },
            opacity: AnimatedValue { keyframes: node.opacity },
        }).collect();
        nodes.sort_by_key(|node| node.layer);
        Ok(Self { duration: dsl.duration, nodes })
    }

    pub fn demo() -> Self {
        Self::from_dsl_json(include_str!("../../ai/example_scene.json")).expect("demo DSL scene should parse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_parent_child_offsets() {
        let scene = Scene::from_dsl_json(r#"
        {
            "duration": 1.0,
            "nodes": [
                {"id":"parent","type":"rect","x":[{"time":0,"value":0.25}],"y":[{"time":0,"value":0.25}],"scale_x":[{"time":0,"value":1}],"scale_y":[{"time":0,"value":1}]},
                {"id":"child","parent_id":"parent","type":"rect","x":[{"time":0,"value":0.1}],"y":[{"time":0,"value":0.2}],"scale_x":[{"time":0,"value":1}],"scale_y":[{"time":0,"value":1}]}
            ]
        }
        "#).unwrap();
        let child = scene.sample_node(1, 0.0).uniform;
        assert_eq!(child.offset, [0.35, 0.45]);
    }
}
