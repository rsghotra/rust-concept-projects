use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K, V> LruCache<K, V> 
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn put(&mut self, key: K, value: V) {
        //if key already present, update and mark recent
        if self.map.contains_key(&key) { //  to avoid two hash lookups -- get mutable reference at first time
            self.map.insert(key.clone(), value);
            self.order.retain(|k| k != &key); //we can use double linked list to make it more efficient
            self.order.push_back(key);
            return;
        }

        //if cache is full, remove last recently used
        if self.map.len() == self.capacity {
            if let Some(old_key) = self.order.pop_front() {
                self.map.remove(&old_key);
            }
        }

        self.map.insert(key.clone(), value);
        self.order.push_back(key);
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.get(key) {
            //mark key as recently used
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());

            Some(value.clone())
        } else {
            None
        }
    }
}


fn main() {
    let mut cache = LruCache::new(2);

    cache.put(1, "one");
    cache.put(2, "two");


    println!("{:?}", cache.get(&1)); //Some("one")

    cache.put(3, "three"); //removes key 2


    println!("{:?}", cache.get(&2)); //None
    println!("{:?}", cache.get(&3)); //Some("three")
}