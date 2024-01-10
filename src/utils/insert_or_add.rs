use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

pub trait InsertOrAdd<T, V>{
    fn insert_or_add(&mut self, key: T, value: V);
}

impl<T: Hash + Eq, V: Add<Output=V> + Clone + Copy> InsertOrAdd<T, V> for HashMap<T, V>{
    fn insert_or_add(&mut self, key: T, value: V) {
        self.entry(key).and_modify(|v| *v = *v + value).or_insert(value);
    }
}