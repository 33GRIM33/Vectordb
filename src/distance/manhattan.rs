use super::DistanceMetric;
// Manhattan distance: sum(|a[i] - b[i]|)
pub struct Manhattan;

impl DistanceMetric for Manhattan {
    fn distance(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter()
         .zip(b)
         .map(|(x, y)| (x - y).abs())
         .sum()
    }
}