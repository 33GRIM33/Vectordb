use std::{collections::HashMap};
use crate::stored_vector::{self, StoredVector};

// TODO: Define VectorDB struct
// Should have:
//   - storage: HashMap<String, StoredVector>  (maps id -> vector)

struct VectorDB{
    storage : HashMap<String,StoredVector>
}


impl VectorDB {
    // TODO: Implement new() - creates empty database
    fn new()->Self{
        Self {
             storage: HashMap::new() 
            }
    }
    fn insert(&mut self,id:String,vector : Vec<f32>,metadata:Option<HashMap<String, String>>){
        let v = StoredVector::new(id.clone(),vector,metadata);
        self.storage.insert(id, v);

    }
    fn get(id : &str){

    }
    
    // TODO: Implement get()
    // Takes: id (&str)
    // Returns: Option<&StoredVector>
    // Hint: Use self.storage.get(id)
    
    
    // TODO: Implement delete()
    // Takes: id (&str)
    // Returns: bool (true if existed and was deleted, false if didn't exist)
    // Hint: Use self.storage.remove(id).is_some()
    
    
    // TODO: Implement len()
    // Returns: usize (number of vectors in database)
    
    
    // TODO: Implement list_ids()
    // Returns: Vec<String> (all vector IDs)
    // Hint: Collect keys from storage HashMap
    
}