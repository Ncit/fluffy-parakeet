#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFit {
    Contain,
    Cover,
    Stretch,
}

impl ImageFit {
    pub fn scale(self, source: [f32; 2], target: [f32; 2]) -> [f32; 2] {
        if source[0] <= 0.0 || source[1] <= 0.0 || target[0] <= 0.0 || target[1] <= 0.0 {
            return [1.0, 1.0];
        }
        let sx = target[0] / source[0];
        let sy = target[1] / source[1];
        match self {
            Self::Stretch => [sx, sy],
            Self::Contain => {
                let scale = sx.min(sy);
                [scale, scale]
            }
            Self::Cover => {
                let scale = sx.max(sy);
                [scale, scale]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_contain_scale() {
        assert_eq!(ImageFit::Contain.scale([100.0, 50.0], [50.0, 50.0]), [0.5, 0.5]);
    }
}
