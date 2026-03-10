use std::{collections::HashMap};
use crate::distance::DistanceMetric;
use crate::stored_vector::StoredVector;
pub struct VectorDB{
    storage : HashMap<String,StoredVector>
}

impl VectorDB {
    pub fn new()->Self{
        Self {
             storage: HashMap::new() 
            }
    }
    pub fn insert(&mut self,id:String,vector : Vec<f32>,metadata:Option<HashMap<String, String>>){
        let v = StoredVector::new(id.clone(),vector,metadata);
        self.storage.insert(id, v);

    }
    pub fn get(&self,id : &str)->Option<&StoredVector>{
        self.storage.get(id)
    }
    pub fn delete(&mut self,id : &str)->bool{
        self.storage.remove(id).is_some()
    }
    pub fn len(&self)->usize{
        self.storage.len()
    }
    pub fn list_ids(&self)->Vec<String>{
        self.storage
        .iter()
        .map(|(key, _)| key.clone())
        .collect()
    }
}