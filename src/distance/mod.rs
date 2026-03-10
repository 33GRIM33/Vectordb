mod euclidean;
mod cosine;
mod manhattan;

pub use euclidean::Euclidean;
pub use cosine::Cosine;
pub use manhattan::Manhattan;

pub trait DistanceMetric {
    fn distance(&self, a: &[f32], b: &[f32]) -> f32;
    
    fn similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        1.0 / (1.0 + self.distance(a, b))
    }
}