pub trait Interpolatable {
    fn lerp(a: Self, b: Self, t: f32) -> Self;
}