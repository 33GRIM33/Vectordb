use super::DistanceMetric;

pub struct Euclidean;

impl DistanceMetric for Euclidean {
    fn distance(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f32>().sqrt()
        
    }
}