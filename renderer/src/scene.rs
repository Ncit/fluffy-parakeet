use serde::Deserialize;

use crate::uniforms::TransformUniform;

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
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform {
    pub fn to_uniform(self) -> TransformUniform {
        TransformUniform {
            offset: [self.x, self.y],
            scale: [self.scale_x, self.scale_y],
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub x: AnimatedValue,
    pub y: AnimatedValue,
    pub scale_x: AnimatedValue,
    pub scale_y: AnimatedValue,
}

impl SceneNode {
    pub fn sample(&self, t: f32) -> Transform {
        Transform {
            x: self.x.sample(t),
            y: self.y.sample(t),
            scale_x: self.scale_x.sample(t),
            scale_y: self.scale_y.sample(t),
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
    #[serde(rename = "type")]
    _kind: String,
    x: Vec<Keyframe>,
    y: Vec<Keyframe>,
    scale_x: Vec<Keyframe>,
    scale_y: Vec<Keyframe>,
}

impl Scene {
    pub fn from_dsl_json(json: &str) -> Result<Self, serde_json::Error> {
        let dsl: DslScene = serde_json::from_str(json)?;

        Ok(Self {
            duration: dsl.duration,
            nodes: dsl.nodes.into_iter().map(|node| SceneNode {
                x: AnimatedValue { keyframes: node.x },
                y: AnimatedValue { keyframes: node.y },
                scale_x: AnimatedValue { keyframes: node.scale_x },
                scale_y: AnimatedValue { keyframes: node.scale_y },
            }).collect(),
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
}
