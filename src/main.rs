use Vector_Db::{VectorDB, distance::{Euclidean, Cosine, Manhattan}};

fn main() {

    let mut db_euclidean = VectorDB::new(Box::new(Euclidean));
    let mut db_cosine = VectorDB::new(Box::new(Cosine));
    let mut db_manhattan = VectorDB::new(Box::new(Manhattan));

    let vec1 = vec![1.0, 0.0, 0.0];
    let vec2 = vec![0.0, 1.0, 0.0];
    let vec3 = vec![0.5, 0.5, 0.0];

    for db in [&mut db_euclidean, &mut db_cosine, &mut db_manhattan] {
        db.insert("vec1".to_string(), vec1.clone(), None);
        db.insert("vec2".to_string(), vec2.clone(), None);
        db.insert("vec3".to_string(), vec3.clone(), None);
    }

    let query = vec![0.6, 0.4, 0.0];

    println!("Query Vector: {:?}\n", query);

    println!("--- Euclidean Search ---");
    let results = db_euclidean.search(&query, 2);

    for r in results {
        println!("id: {}, distance: {}", r.id(), r.distance());
    }

    println!("\n--- Cosine Search ---");
    let results = db_cosine.search(&query, 2);

    for r in results {
        println!("id: {}, distance: {}", r.id(), r.distance());
    }

    println!("\n--- Manhattan Search ---");
    let results = db_manhattan.search(&query, 2);

    for r in results {
        println!("id: {}, distance: {}", r.id(), r.distance());
    }

}