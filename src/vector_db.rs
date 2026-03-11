use crate::distance::DistanceMetric;
use crate::search_result::{self, SearchResult};
use crate::stored_vector::{self, StoredVector};
use std::collections::HashMap;
pub struct VectorDB {
    storage: HashMap<String, StoredVector>,
    metric: Box<dyn DistanceMetric>,
}

impl VectorDB {
    pub fn new(metric: Box<dyn DistanceMetric>) -> Self {
        Self {
            storage: HashMap::new(),
            metric,
        }
    }
    pub fn insert(
        &mut self,
        id: String,
        vector: Vec<f32>,
        metadata: Option<HashMap<String, String>>,
    ) {
        let v = StoredVector::new(id.clone(), vector, metadata);
        self.storage.insert(id, v);
    }
    pub fn get(&self, id: &str) -> Option<&StoredVector> {
        self.storage.get(id)
    }
    pub fn delete(&mut self, id: &str) -> bool {
        self.storage.remove(id).is_some()
    }
    pub fn len(&self) -> usize {
        self.storage.len()
    }
    pub fn list_ids(&self) -> Vec<String> {
        self.storage.iter().map(|(key, _)| key.clone()).collect()
    }
    pub fn calculate_distance(&self, id1: &str, id2: &str) -> Option<f32> {
        let v1 = self.get(id1)?;
        let v2 = self.get(id2)?;
        Some(self.metric.distance(v1.vector(), v2.vector()))
    }
    pub fn search(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
        let mut res: Vec<SearchResult> = Vec::new();
        for (id, stored_vector) in &self.storage {
            let dist = self.metric.distance(query, &stored_vector.vector());
            res.push(SearchResult::new(id.clone(), dist));
        }
        res.sort_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap());
        res.truncate(k);
        res
    }
    pub fn search_filtered<F>(&self, query: &[f32], k: usize, filter: F) -> Vec<SearchResult>
    where
        F: Fn(&HashMap<String, String>) -> bool,
    {
        let mut res = Vec::new();

        for (id, stored_vec) in &self.storage {
            if let Some(metadata) = stored_vec.metadata() {
                if !filter(metadata) {
                    continue;
                }
            }

            let dist = self.metric.distance(query, stored_vec.vector());

            res.push(SearchResult::new(id.clone(), dist));
        }

        res.sort_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap());

        res.truncate(k);

        res
    }
}
