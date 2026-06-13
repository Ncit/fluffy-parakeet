use serde::Deserialize;

use crate::uniforms::NodeUniform;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AnimatedValue {
    pub keyframes: Vec<Keyframe>,
}

impl AnimatedValue {
    pub fn constant(value: f32) -> Self {
        Self {
            keyframes: vec![Keyframe { time: 0.0, value }],
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
                return a.value + (b.value - a.value) * local_t;
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
    pub color: [f32; 4],
    pub x: AnimatedValue,
    pub y: AnimatedValue,
    pub scale_x: AnimatedValue,
    pub scale_y: AnimatedValue,
    pub rotation: AnimatedValue,
    pub opacity: AnimatedValue,
}

impl SceneNode {
    pub fn sample(&self, t: f32) -> SampledNode {
        SampledNode {
            uniform: NodeUniform {
                offset: [self.x.sample(t), self.y.sample(t)],
                scale: [self.scale_x.sample(t), self.scale_y.sample(t)],
                rotation: self.rotation.sample(t),
                opacity: self.opacity.sample(t),
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
    #[serde(default = "default_color")]
    color: [f32; 4],
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

fn default_rotation() -> Vec<Keyframe> {
    vec![Keyframe { time: 0.0, value: 0.0 }]
}

fn default_opacity() -> Vec<Keyframe> {
    vec![Keyframe { time: 0.0, value: 1.0 }]
}

impl Scene {
    pub fn from_dsl_json(json: &str) -> Result<Self, serde_json::Error> {
        let dsl: DslScene = serde_json::from_str(json)?;

        let mut nodes: Vec<SceneNode> = dsl.nodes.into_iter().map(|node| SceneNode {
            id: node.id,
            kind: node.kind,
            layer: node.layer,
            color: node.color,
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
                Keyframe { time: 0.0, value: 0.0 },
                Keyframe { time: 1.0, value: 10.0 },
            ],
        };

        assert_eq!(value.sample(0.5), 5.0);
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
}
