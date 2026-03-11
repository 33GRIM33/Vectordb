// This holds one search result (id + distance)

#[derive(Debug, Clone)]
pub struct SearchResult {
    id : String,
    distance: f32
}

impl SearchResult {
    pub fn new(id: String, distance: f32) -> Self {
        Self { id, distance }
    }
    pub fn distance(&self)->f32{
        self.distance
    }
    pub fn id(&self)->&String{
        &self.id
    }
}