use Vector_Db::{VectorDB, distance::{Euclidean, Cosine, Manhattan}};

fn main() {
    println!("=== VectorDB Phase 2: Distance Metrics ===\n");

    // Create databases with different metrics
    let mut db_euclidean = VectorDB::new(Box::new(Euclidean));
    let mut db_cosine = VectorDB::new(Box::new(Cosine));
    let mut db_manhattan = VectorDB::new(Box::new(Manhattan));

    // Test vectors
    let vec1 = vec![1.0, 0.0, 0.0];
    let vec2 = vec![0.0, 1.0, 0.0];
    let vec3 = vec![0.5, 0.5, 0.0];

    // Insert vectors into all DBs
    db_euclidean.insert("vec1".to_string(), vec1.clone(), None);
    db_euclidean.insert("vec2".to_string(), vec2.clone(), None);
    db_euclidean.insert("vec3".to_string(), vec3.clone(), None);

    db_cosine.insert("vec1".to_string(), vec1.clone(), None);
    db_cosine.insert("vec2".to_string(), vec2.clone(), None);
    db_cosine.insert("vec3".to_string(), vec3.clone(), None);

    db_manhattan.insert("vec1".to_string(), vec1.clone(), None);
    db_manhattan.insert("vec2".to_string(), vec2.clone(), None);
    db_manhattan.insert("vec3".to_string(), vec3.clone(), None);

    println!("--- Euclidean Distance ---");

    match db_euclidean.calculate_distance("vec1", "vec2") {
        Some(dist) => println!("vec1 -> vec2 = {}", dist),
        None => println!("Vector not found"),
    }

    match db_euclidean.calculate_distance("vec1", "vec3") {
        Some(dist) => println!("vec1 -> vec3 = {}", dist),
        None => println!("Vector not found"),
    }

    println!("\n--- Cosine Distance ---");

    match db_cosine.calculate_distance("vec1", "vec2") {
        Some(dist) => println!("vec1 -> vec2 = {}", dist),
        None => println!("Vector not found"),
    }

    match db_cosine.calculate_distance("vec1", "vec3") {
        Some(dist) => println!("vec1 -> vec3 = {}", dist),
        None => println!("Vector not found"),
    }

    println!("\n--- Manhattan Distance ---");

    match db_manhattan.calculate_distance("vec1", "vec2") {
        Some(dist) => println!("vec1 -> vec2 = {}", dist),
        None => println!("Vector not found"),
    }

    match db_manhattan.calculate_distance("vec1", "vec3") {
        Some(dist) => println!("vec1 -> vec3 = {}", dist),
        None => println!("Vector not found"),
    }

    println!("\n=== Phase 2 Complete! ===");
}