use crate::uniforms::TransformUniform;

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

#[derive(Clone, Copy, Debug)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
}

#[derive(Clone, Debug)]
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
                let local_t = (t - a.time) / (b.time - a.time);
                return a.value + (b.value - a.value) * local_t;
            }
        }

        self.keyframes.last().unwrap().value
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
    pub nodes: Vec<SceneNode>,
}

impl Scene {
    pub fn demo() -> Self {
        Self {
            nodes: vec![
                SceneNode {
                    x: AnimatedValue { keyframes: vec![
                        Keyframe { time: 0.0, value: -0.6 },
                        Keyframe { time: 1.5, value: 0.6 },
                        Keyframe { time: 3.0, value: -0.6 },
                    ]},
                    y: AnimatedValue { keyframes: vec![Keyframe { time: 0.0, value: 0.25 }] },
                    scale_x: AnimatedValue { keyframes: vec![Keyframe { time: 0.0, value: 0.35 }] },
                    scale_y: AnimatedValue { keyframes: vec![Keyframe { time: 0.0, value: 0.35 }] },
                },
                SceneNode {
                    x: AnimatedValue { keyframes: vec![Keyframe { time: 0.0, value: 0.0 }] },
                    y: AnimatedValue { keyframes: vec![
                        Keyframe { time: 0.0, value: -0.45 },
                        Keyframe { time: 1.5, value: 0.05 },
                        Keyframe { time: 3.0, value: -0.45 },
                    ]},
                    scale_x: AnimatedValue { keyframes: vec![Keyframe { time: 0.0, value: 0.25 }] },
                    scale_y: AnimatedValue { keyframes: vec![Keyframe { time: 0.0, value: 0.25 }] },
                },
            ],
        }
    }
}