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
        Self {
            keyframes: vec![Keyframe {
                time: 0.0,
                value,
                easing: Easing::Linear,
            }],
        }
    }

    pub fn sample(&self, t: f32) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        if t <= self.keyframes[0].time {
            return self.keyframes[0].value;
        }

        for pair in self.keyframes.windows(2) {
            let a = pair[0];
            let b = pair[1];
            if t >= a.time && t <= b.time {
                let duration = b.time - a.time;
                if duration.abs() < f32::EPSILON {
                    return b.value;
                }
                let local_t = (t - a.time) / duration;
                let eased_t = b.easing.apply(local_t);
                return a.value + (b.value - a.value) * eased_t;
            }
        }

        self.keyframes.last().unwrap().value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SampledNode {
    pub uniform: NodeUniform,
}

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub id: String,
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

    pub fn sample(&self, t: f32) -> SampledNode {
        let local_t = (t - self.start_time).max(0.0);
        let scale_x = self.width * self.scale_x.sample(local_t);
        let scale_y = self.height * self.scale_y.sample(local_t);

        SampledNode {
            uniform: NodeUniform {
                offset: [
                    self.x.sample(local_t) + (0.5 - self.anchor_x) * scale_x,
                    self.y.sample(local_t) + (0.5 - self.anchor_y) * scale_y,
                ],
                scale: [scale_x, scale_y],
                rotation: self.rotation.sample(local_t),
                opacity: self.opacity.sample(local_t),
                _padding: [0.0, 0.0],
                color: self.color,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub duration: f32,
    pub nodes: Vec<SceneNode>,
}

#[derive(Debug, Deserialize)]
struct DslScene {
    duration: f32,
    nodes: Vec<DslNode>,
}

#[derive(Debug, Deserialize)]
struct DslNode {
    #[serde(default)]
    id: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    layer: i32,
    #[serde(default)]
    start_time: f32,
    #[serde(default)]
    end_time: Option<f32>,
    #[serde(default = "default_color")]
    color: [f32; 4],
    #[serde(default = "default_width")]
    width: f32,
    #[serde(default = "default_height")]
    height: f32,
    #[serde(default = "default_anchor")]
    anchor_x: f32,
    #[serde(default = "default_anchor")]
    anchor_y: f32,
    x: Vec<Keyframe>,
    y: Vec<Keyframe>,
    scale_x: Vec<Keyframe>,
    scale_y: Vec<Keyframe>,
    #[serde(default = "default_rotation")]
    rotation: Vec<Keyframe>,
    #[serde(default = "default_opacity")]
    opacity: Vec<Keyframe>,
}

fn default_color() -> [f32; 4] {
    [0.2, 0.8, 1.0, 1.0]
}

fn default_width() -> f32 {
    1.0
}

fn default_height() -> f32 {
    1.0
}

fn default_anchor() -> f32 {
    0.5
}

fn default_rotation() -> Vec<Keyframe> {
    vec![Keyframe {
        time: 0.0,
        value: 0.0,
        easing: Easing::Linear,
    }]
}

fn default_opacity() -> Vec<Keyframe> {
    vec![Keyframe {
        time: 0.0,
        value: 1.0,
        easing: Easing::Linear,
    }]
}

impl Scene {
    pub fn from_dsl_json(json: &str) -> Result<Self, serde_json::Error> {
        let dsl: DslScene = serde_json::from_str(json)?;

        let mut nodes: Vec<SceneNode> = dsl.nodes.into_iter().map(|node| SceneNode {
            id: node.id,
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

        Ok(Self {
            duration: dsl.duration,
            nodes,
        })
    }

    pub fn demo() -> Self {
        Self::from_dsl_json(include_str!("../../ai/example_scene.json"))
            .expect("demo DSL scene should parse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_between_keyframes() {
        let value = AnimatedValue {
            keyframes: vec![
                Keyframe { time: 0.0, value: 0.0, easing: Easing::Linear },
                Keyframe { time: 1.0, value: 10.0, easing: Easing::Linear },
            ],
        };

        assert_eq!(value.sample(0.5), 5.0);
    }

    #[test]
    fn supports_ease_in_keyframes() {
        let value = AnimatedValue {
            keyframes: vec![
                Keyframe { time: 0.0, value: 0.0, easing: Easing::Linear },
                Keyframe { time: 1.0, value: 10.0, easing: Easing::EaseIn },
            ],
        };

        assert_eq!(value.sample(0.5), 2.5);
    }

    #[test]
    fn parses_demo_scene() {
        let scene = Scene::demo();
        assert_eq!(scene.nodes.len(), 2);
        assert!(scene.duration > 0.0);
    }

    #[test]
    fn sorts_nodes_by_layer() {
        let scene = Scene::demo();
        assert!(scene.nodes[0].layer <= scene.nodes[1].layer);
    }

    #[test]
    fn samples_visual_uniforms() {
        let scene = Scene::demo();
        let sampled = scene.nodes[0].sample(0.0);
        assert!(sampled.uniform.opacity >= 0.0);
        assert!(sampled.uniform.color[3] > 0.0);
    }

    #[test]
    fn applies_width_and_height_to_scale() {
        let scene = Scene::demo();
        let sampled = scene.nodes[0].sample(0.0);
        assert!(sampled.uniform.scale[0] > 0.0);
        assert!(sampled.uniform.scale[1] > 0.0);
    }

    #[test]
    fn defaults_anchor_to_center() {
        let scene = Scene::demo();
        assert_eq!(scene.nodes[0].anchor_x, 0.5);
        assert_eq!(scene.nodes[0].anchor_y, 0.5);
    }

    #[test]
    fn supports_top_left_anchor_offset() {
        let scene = Scene::from_dsl_json(r#"
        {
            "duration": 1.0,
            "nodes": [{
                "type": "rect",
                "width": 1.0,
                "height": 1.0,
                "anchor_x": 0.0,
                "anchor_y": 0.0,
                "x": [{"time": 0.0, "value": 0.0}],
                "y": [{"time": 0.0, "value": 0.0}],
                "scale_x": [{"time": 0.0, "value": 1.0}],
                "scale_y": [{"time": 0.0, "value": 1.0}]
            }]
        }
        "#).unwrap();

        let sampled = scene.nodes[0].sample(0.0);
        assert_eq!(sampled.uniform.offset, [0.5, 0.5]);
    }

    #[test]
    fn respects_node_lifecycle() {
        let scene = Scene::from_dsl_json(r#"
        {
            "duration": 3.0,
            "nodes": [{
                "type": "rect",
                "start_time": 1.0,
                "end_time": 2.0,
                "x": [{"time": 0.0, "value": 0.0}],
                "y": [{"time": 0.0, "value": 0.0}],
                "scale_x": [{"time": 0.0, "value": 1.0}],
                "scale_y": [{"time": 0.0, "value": 1.0}]
            }]
        }
        "#).unwrap();

        assert!(!scene.nodes[0].is_active(0.5));
        assert!(scene.nodes[0].is_active(1.5));
        assert!(!scene.nodes[0].is_active(2.5));
    }
}
