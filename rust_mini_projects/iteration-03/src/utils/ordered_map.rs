use std::collections::btree_set::{IntoIter, Iter};
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;
use std::rc::Rc;

/// A custom data structure created for the timing board
/// It allows storing ordered items using BTreeSet while at the same time
/// key lookups are still possible with the associated HashMap.
pub struct OrderedMap<K, V>
where
    K: Eq + Hash,
    V: Ord,
{
    ord: BTreeSet<Rc<V>>,
    map: HashMap<K, Rc<V>>,
}

impl<K, V> Default for OrderedMap<K, V>
where
    K: Eq + Hash,
    V: Ord,
{
    fn default() -> Self {
        Self {
            ord: BTreeSet::default(),
            map: HashMap::default(),
        }
    }
}

impl<K, V> OrderedMap<K, V>
where
    K: Eq + Hash,
    V: Ord,
{
    pub fn insert(&mut self, key: K, value: V) {
        let rc = Rc::new(value);
        self.map.insert(key, rc.clone());
        self.ord.insert(rc);
    }
    pub fn get(&self, key: &K) -> Option<Rc<V>> {
        self.map.get(key).cloned()
    }

    pub fn get_first(&self) -> Option<Rc<V>> {
        self.ord.first().cloned()
    }

    pub fn replace(&mut self, key: K, value: V) {
        self.map.remove(&key).map(|v| self.ord.remove(&v));
        self.insert(key, value);
    }

    pub fn new() -> Self {
        Self {
            ord: BTreeSet::new(),
            map: HashMap::new(),
        }
    }
    pub fn iter(&self) -> OrderedMapIter<'_, V> {
        OrderedMapIter {
            inner_iter: self.ord.iter(),
        }
    }
}

pub struct OrderedMapIter<'a, V>
where
    V: Ord,
{
    inner_iter: Iter<'a, Rc<V>>,
}

impl<'a, V> Iterator for OrderedMapIter<'a, V>
where
    V: Ord,
{
    type Item = &'a Rc<V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

impl<K, V> IntoIterator for OrderedMap<K, V>
where
    K: Eq + Hash,
    V: Ord,
{
    type Item = Rc<V>;
    type IntoIter = IntoIter<Rc<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.ord.into_iter()
    }
}
