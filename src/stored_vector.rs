use std::collections::HashMap;
#[derive(Debug)]
pub struct StoredVector{
    id:String,
    vector:Vec<f32>,
    metadata : Option<HashMap<String,String>>,
}

impl StoredVector {
    pub fn new(id : String, vector : Vec<f32>,metadata:Option<HashMap<String,String>>)->Self {
        Self {
            id,
            vector,
            metadata
        }
    }
    pub fn dimension(&self)->usize{
        self.vector.len()
    }
}
