use crate::{Float, FloatSet};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct VecSet<K> {
    data: Vec<K>,
}

impl<K> VecSet<K> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub unsafe fn raw(&mut self) -> &mut Vec<K> {
        &mut self.data
    }
}

impl<K> VecSet<K>
where
    K: Ord,
{
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.data
            .binary_search_by(|a| a.borrow().partial_cmp(key).unwrap())
            .is_ok()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self
            .data
            .binary_search_by(|a| a.borrow().partial_cmp(key).unwrap())
        {
            Ok(idx) => {
                let k = &self.data[idx];
                Some(k)
            }
            Err(_) => None,
        }
    }

    pub fn insert(&mut self, key: K) {
        match self.data.binary_search_by(|a| a.partial_cmp(&key).unwrap()) {
            Ok(_) => {}
            Err(idx) => {
                self.data.insert(idx, key);
            }
        }
    }

    pub fn range<'a, R>(&'a self, range: R) -> impl Iterator<Item = &'a K>
    where
        R: RangeBounds<K> + 'a,
    {
        let mut start = None;

        for i in 0..self.data.len() {
            let k = &self.data[i];
            if range.contains(k) {
                start = Some(i);
                break;
            }
        }

        let start = match start {
            Some(i) => i,
            None => return self.data[0..0].iter(),
        };

        for i in (start..self.data.len()).rev() {
            if range.contains(&self.data[i]) {
                return self.data[start..i + 1].iter();
            }
        }

        self.data[start..].iter()
    }

    pub fn remove(&mut self, key: &K) -> Option<K> {
        match self.data.binary_search_by(|a| a.partial_cmp(key).unwrap()) {
            Ok(idx) => Some(self.data.remove(idx)),
            Err(_) => None,
        }
    }
}

impl<K> FloatSet<K> for VecSet<K>
where
    K: Float + PartialOrd<K>,
{
    fn contains(&self, key: &K) -> bool {
        self.data.binary_search_by(|a| a.ord(key)).is_ok()
    }

    fn get(&self, key: &K) -> Option<&K> {
        match self.data.binary_search_by(|a| a.ord(key)) {
            Ok(idx) => {
                let k = &self.data[idx];
                Some(k)
            }
            Err(_) => None,
        }
    }

    fn insert(&mut self, key: K) {
        match self.data.binary_search_by(|a| a.ord(&key)) {
            Ok(_) => {}
            Err(idx) => {
                self.data.insert(idx, key);
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<K> {
        match self.data.binary_search_by(|a| a.ord(key)) {
            Ok(idx) => Some(self.data.remove(idx)),
            Err(_) => None,
        }
    }

    fn range<'a, R>(&'a self, range: R) -> Box<dyn Iterator<Item = &'a K> + 'a>
    where
        R: RangeBounds<K> + 'a,
    {
        let mut start = None;

        for i in 0..self.data.len() {
            let k = &self.data[i];
            if range.contains(k) {
                start = Some(i);
                break;
            }
        }

        let start = match start {
            Some(i) => i,
            None => return Box::new(self.data[0..0].iter()),
        };

        for i in (start..self.data.len()).rev() {
            if range.contains(&self.data[i]) {
                return Box::new(self.data[start..i + 1].iter());
            }
        }

        Box::new(self.data[start..].iter())
    }
}

impl<K> std::fmt::Debug for VecSet<K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.data.iter()).finish()
    }
}
