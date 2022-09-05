use crate::dummy::Dummy;
use std::collections::HashSet;
use std::collections::HashMap;

impl<T: Dummy> Dummy for Vec<T> {
    fn dummy() -> Self {
        let mut result = Vec::<T>::new();
        let size = u32::dummy() % 10 + 2;
        (0..size).for_each(|_|{
            result.push(T::dummy());
        });
        result
    }
}

impl<T: Dummy + std::cmp::Eq + std::hash::Hash> Dummy for HashSet<T> {
    fn dummy() -> Self {
        let mut result = HashSet::<T>::new();
        let size = u32::dummy() % 10 + 2;
        (0..size).for_each(|_|{
            result.insert(T::dummy());
        });
        result
    }
}


impl<K: Dummy + std::cmp::Eq + std::hash::Hash, V: Dummy> Dummy for HashMap<K, V> {
    fn dummy() -> Self {
        let mut result = HashMap::<K,V>::new();
        let size = u32::dummy() % 10 + 2;
        (0..size).for_each(|_|{
            result.insert(K::dummy(), V::dummy());
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use itertools::zip;

    #[test]
    fn test_vec() {
        assert!((0..10)
                .map(|_| Vec::<i32>::dummy().len())
                .unique()
                .count() > 1);
        assert!((0..10)
                .map(|_| Vec::<i32>::dummy())
                .unique()
                .count() > 1);
    }

    #[test]
    fn test_hashset() {
        let a = HashSet::<String>::dummy();
        let b = HashSet::<String>::dummy();
        let mut found_a_difference = false;
        for (a_, b_) in zip(a, b) {
            if a_ != b_ {
                found_a_difference = true;
                break;
            }
        }
        assert!(found_a_difference);

        assert!((0..10)
                .map(|_| HashSet::<i32>::dummy().len())
                .unique()
                .count() > 1);
    }

    #[test]
    fn test_hashmap() {
        let a = HashMap::<String, i32>::dummy();
        let b = HashMap::<String, i32>::dummy();
        for (a_pair, b_pair) in zip(a, b) {
           assert_ne!(a_pair.0, b_pair.0);
           assert_ne!(a_pair.1, b_pair.1);
        }

        assert!((0..10)
                .map(|_| HashMap::<String, i32>::dummy().len())
                .unique()
                .count() > 1);
    }
}
