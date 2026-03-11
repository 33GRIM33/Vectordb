use super::DistanceMetric;
pub struct Cosine;

impl Cosine {
    fn dot_product(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x,y)|x*y).sum()
    }
    
    fn magnitude(v: &[f32]) -> f32 {
        v.iter().map(|x|x*x).sum::<f32>().sqrt()
    }
}

impl DistanceMetric for Cosine {
    fn distance(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot = Self::dot_product(a,b);
        let d = Self::magnitude(a)* Self::magnitude(b);
        if d == 0.0{
            return 1.0
        }
        let c = dot / d;
        
        1.0 - c
    }
}