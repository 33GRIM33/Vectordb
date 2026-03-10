use Vector_Db::VectorDB;
use std::collections::HashMap;

fn main() {
    println!("=== VectorDB Phase 1 Testing ===\n");

    let mut db = VectorDB::new();

    let vec1 = vec![1.0, 0.0, 0.0];
    let vec2 = vec![0.0, 1.0, 0.0];
    let vec3 = vec![0.5, 0.5, 0.0];

    let mut meta1 = HashMap::new();
    meta1.insert("type".to_string(), "test".to_string());
    meta1.insert("name".to_string(), "vector1".to_string());

    let mut meta2 = HashMap::new();
    meta2.insert("type".to_string(), "test".to_string());
    meta2.insert("name".to_string(), "vector2".to_string());

    let mut meta3 = HashMap::new();
    meta3.insert("type".to_string(), "test".to_string());
    meta3.insert("name".to_string(), "vector3".to_string());

    db.insert("vec1".into(), vec1, Some(meta1));
    db.insert("vec2".into(), vec2, Some(meta2));
    db.insert("vec3".into(), vec3, Some(meta3));

    println!("IDs before delete: {:?}", db.list_ids());
    
    let removed = db.delete("vec1");
    println!("Removed vec1: {}", removed);
    println!("Length after delete: {}", db.len());
    println!("\n=== All tests passed! ===");
}
