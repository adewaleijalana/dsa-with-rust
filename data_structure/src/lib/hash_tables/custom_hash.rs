// pub struct CustomHash<K, V> {
//     data: Vec<Option<(K, V)>>,
//     length: usize,
// }

// impl<K: Eq + std::hash::Hash, V> CustomHash<K, V> {
//     pub fn new(size: usize) -> Self {
//         CustomHash {
//             data: vec![None; size],
//             length: 0,
//         }
//     }

//     pub fn add(&mut self, key: K, value: V) {
//         let index = self.hash(&key);
//         self.data[index] = Some((key, value));
//         self.length += 1;
//     }

//     pub fn get(&self, key: &K) -> Option<&V> {
//         let index = self.hash(key);
//         if let Some((_, value)) = &self.data[index] {
//             Some(value)
//         } else {
//             None
//         }
//     }

//     fn hash(&self, key: &K) -> usize {
//       use std::hash::{Hash, Hasher};
//       use std::collections::hash_map::DefaultHasher;

//       let mut hasher = DefaultHasher::new();
//       key.hash(&mut hasher);
//       (hasher.finish() as usize) % self.data.len()
//   }
//   }
