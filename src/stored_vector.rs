

use std::{collections::{HashMap, hash_map}, fs::Metadata};
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
    fn dimension(vector : &Vec<f32>)->usize{

        vector.len()
    }
}
