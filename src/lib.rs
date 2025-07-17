//! # RandSet
//!
//! A hash set that supports efficient random element access.
//!
//! `RandSet` combines the fast lookup of a hash set with the ability to
//! efficiently retrieve random elements, using the get_rand() method.
//! 
//! rest of the method work the same as a HashSet https://doc.rust-lang.org/std/collections/struct.HashSet.html
//!
//! ## Key Differences from Standard Sets
//!
//! Unlike a regular `HashSet`, elements stored in `RandSet` must implement the
//! `Clone` trait. 
//! All other methods
//! work like a regular set (insertion, removal, containment checking, etc.).
//!
//! ## Example
//!
//! ```
//! use rand_set::RandSet;
//!
//! let mut set = RandSet::new();
//! set.insert("hello");
//! set.insert("world");
//!
//! // Fast containment check (works like HashSet)
//! assert!(set.contains(&"hello"));
//!
//! // Unique feature: Get a random element in O(1)
//! if let Some(random_item) = set.get_rand() {
//!     println!("Random: {}", random_item);
//! }
//! 

use std::collections::HashMap;
use std::vec::Vec;
use std::hash::{Hash, BuildHasher, RandomState};

use std::fmt;

use rand::Rng;



#[derive(Debug, Clone, Default)]
pub struct RandSet<T, S = RandomState>
where
    T:Hash + Eq + Clone,
    S:BuildHasher,
{
    
    values_to_index: HashMap<T, usize, S>,
    items_vector: Vec<T>,

}

pub type RandSetDefault<T> = RandSet<T, RandomState>;




impl<T, S> RandSet<T, S>
where
    T:Hash + Eq + Clone,
    S: BuildHasher,
{

    /// Get a reference to an random element in the set, if it exists.
    pub fn get_rand(&self) -> Option<&T> {
        if self.items_vector.is_empty() {
            return None;
        }
        
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..self.items_vector.len());
        Some(&self.items_vector[random_index])
    }

   
    pub fn get(&self, value: &T) -> Option<&T> {
        if self.items_vector.is_empty() {
            return None;
        }

        match self.values_to_index.get_key_value(value){
            Some((key, _)) => Some(key),
            None => None,
        }
    }


    pub fn with_hasher(hash_builder: S) -> Self {
        RandSet {
            values_to_index: HashMap::with_hasher(hash_builder),
            items_vector: Vec::new(),
        }
    }


    pub fn hasher(&self) -> &S {
        self.values_to_index.hasher()
    }


    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        RandSet {
            values_to_index: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            items_vector: Vec::with_capacity(capacity),
        }
    }


    pub fn capacity(&self) -> usize{
        return self.values_to_index.capacity(); 
    }

    pub fn is_empty(&self) -> bool{
        return self.values_to_index.is_empty();
    }

    pub fn clear(&mut self) {
        self.values_to_index.clear();
        self.items_vector.clear(); 
    }

    pub fn remove(&mut self, value: &T) -> bool{  

        let matching_index =  match self.values_to_index.get(value) {
            Some(val) => val.clone(),
            None => return false,
        };

        let last_index = self.items_vector.len() - 1;

        if matching_index != last_index {
            self.items_vector.swap(matching_index, last_index);

            let swapped_value = &self.items_vector[matching_index];
            self.values_to_index.insert(swapped_value.clone(), matching_index);
        } 

        self.items_vector.pop();
        self.values_to_index.remove(value);

        true
    }

    pub fn len(&self) -> usize {
        self.items_vector.len()
    }



    pub fn insert(&mut self, value: T) -> bool{  
        if self.values_to_index.contains_key(&value){  
            return false;
        }
                
        self.values_to_index.insert(value.clone(), self.items_vector.len());  
        self.items_vector.push(value);
        true
    }


    pub fn contains(&self, value: &T) -> bool{  

        self.values_to_index.contains_key(value)
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.items_vector.iter()
    }
}

impl<T> RandSet<T, RandomState>
where
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        RandSet {
            values_to_index: HashMap::new(),
            items_vector: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        RandSet {
            values_to_index: HashMap::with_capacity(capacity),
            items_vector: Vec::with_capacity(capacity),
        }
    }
}


// PartialEq implementation that works with different hashers
impl<T, S1, S2> PartialEq<RandSet<T, S2>> for RandSet<T, S1>
where
    T: Hash + Eq + Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &RandSet<T, S2>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        
        // Check if all elements in self are in other
        self.iter().all(|item| other.contains(item))
    }
}

impl<T, S> Eq for RandSet<T, S>
where
    T: Hash + Eq + Clone,
    S: BuildHasher,
{}

impl<T, S> fmt::Display for RandSet<T, S>
where
    T: Hash + Eq + Clone + fmt::Display,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RandSet{{")?;
        for (i, item) in self.items_vector.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "}}")
    }
}

// FromIterator implementation
impl<T, S> FromIterator<T> for RandSet<T, S>
where
    T: Hash + Eq + Clone,
    S: BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = RandSet::with_hasher(S::default());
        for item in iter {
            set.insert(item);
        }
        set
    }
}

// IntoIterator implementations
impl<T, S> IntoIterator for RandSet<T, S>
where
    T: Hash + Eq + Clone,
    S: BuildHasher,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items_vector.into_iter()
    }
}

impl<'a, T, S> IntoIterator for &'a RandSet<T, S>
where
    T: Hash + Eq + Clone,
    S: BuildHasher,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items_vector.iter()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let mut rs = RandSet::<i32>::new();

        rs.insert(23);
        rs.insert(40);
        rs.insert(15);
        rs.insert(17);
        assert_eq!(rs.contains(&23),true);
        assert_eq!(rs.len(),4);

        rs.remove(&23);
        assert_eq!(rs.contains(&23),false);
        assert_eq!(rs.len(),3);
    }

    #[test]
    fn contains_fail() {
        let mut rs = RandSet::<i32>::new();

        rs.insert(23);
        rs.insert(40);
        rs.insert(15);
        rs.insert(17);
        assert_ne!(rs.contains(&23),false);
    }

    #[test]
    fn is_empty_false() {
        let mut rs = RandSet::<i32>::new();

        rs.insert(23);
        rs.insert(40);
        rs.insert(15);
        rs.insert(17);
        assert_eq!(rs.is_empty(),false);
    }

    #[test]
    fn is_empty_true() {
        let mut rs = RandSet::<i32>::new();
        assert_eq!(rs.is_empty(),true);

        rs.insert(23);
        rs.insert(40);
        assert_eq!(rs.is_empty(),false);

        rs.remove(&23);
        rs.remove(&40);
        assert_eq!(rs.is_empty(),true);
        assert_eq!(rs.len(),0);
    }

    #[test]
    fn get_rand() {
        let mut rs = RandSet::<i32>::new();
        assert_eq!(rs.get_rand().is_none(), true);

        rs.insert(23);
        rs.insert(40);
        assert_eq!(rs.get_rand().is_some(), true);
     
        rs.remove(&23);
        assert_eq!(rs.get_rand().is_some(), true);
        assert_eq!(*rs.get_rand().unwrap(), 40);
    }

    #[test]
    fn equal() {
        let mut rs = RandSet::<i32>::new();

        rs.insert(23);
        rs.insert(40);
        rs.insert(80);

        rs.remove(&80);
        rs.remove(&23);

        let mut equal_rs = RandSet::<i32>::new();

        equal_rs.insert(40);

        assert!(rs == equal_rs);

        equal_rs.insert(23);
        assert!(rs != equal_rs);

    }
}
