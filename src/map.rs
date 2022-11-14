use crate::util::{Float, FloatMap};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct VecMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.data.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub unsafe fn raw(&mut self) -> &mut Vec<(K, V)> {
        &mut self.data
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.iter().map(|(_, v)| v)
    }
}

impl<K, V> VecMap<K, V>
where
    K: Ord,
{
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self
            .data
            .binary_search_by(|(a, _)| a.borrow().partial_cmp(key).unwrap())
        {
            Ok(idx) => {
                let (_, v) = &self.data[idx];
                Some(v)
            }
            Err(_) => None,
        }
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self
            .data
            .binary_search_by(|(a, _)| a.borrow().partial_cmp(key).unwrap())
        {
            Ok(idx) => {
                let (k, v) = &self.data[idx];
                Some((k, v))
            }
            Err(_) => None,
        }
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self
            .data
            .binary_search_by(|(a, _)| a.borrow().partial_cmp(key).unwrap())
        {
            Ok(idx) => Some(&mut self.data[idx].1),
            Err(_) => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self
            .data
            .binary_search_by(|(a, _)| a.partial_cmp(&key).unwrap())
        {
            Ok(idx) => {
                let (_, v) = std::mem::replace(&mut self.data[idx], (key, value));
                Some(v)
            }
            Err(idx) => {
                self.data.insert(idx, (key, value));
                None
            }
        }
    }

    pub fn range<'a, R>(&'a self, range: R) -> impl Iterator<Item = &'a (K, V)>
    where
        R: RangeBounds<K> + 'a,
    {
        let mut start = None;

        for i in 0..self.data.len() {
            let (k, _) = &self.data[i];
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
            let (k, _) = &self.data[i];
            if range.contains(k) {
                return self.data[start..i + 1].iter();
            }
        }

        return self.data[start..].iter();
    }

    pub fn range_mut<'a, R>(&'a mut self, range: R) -> impl Iterator<Item = &'a mut (K, V)>
    where
        R: RangeBounds<K> + 'a,
    {
        let mut start = None;

        for i in 0..self.data.len() {
            let (k, _) = &self.data[i];
            if range.contains(k) {
                start = Some(i);
                break;
            }
        }

        let start = match start {
            Some(i) => i,
            None => return self.data[0..0].iter_mut(),
        };

        for i in (start..self.data.len()).rev() {
            let (k, _) = &self.data[i];
            if range.contains(k) {
                return self.data[start..i + 1].iter_mut();
            }
        }

        return self.data[start..].iter_mut();
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self.data.binary_search_by(|(a, _)| a.borrow().cmp(key)) {
            Ok(idx) => Some(self.data.remove(idx)),
            Err(_) => None,
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        K: Ord,
        F: FnMut(&K, &mut V) -> bool,
    {
        let mut deleted = 0;

        for i in 0..self.data.len() {
            let (k, v) = &mut self.data[i - deleted];

            if !f(k, v) {
                self.data.remove(i - deleted);
                deleted += 1;
            }
        }
    }
}

impl<K, V> FloatMap<K, V> for VecMap<K, V>
where
    K: Float + PartialOrd<K>,
{
    fn get(&self, key: &K) -> Option<&V> {
        match self.data.binary_search_by(|(a, _)| a.ord(key)) {
            Ok(idx) => {
                let (_, v) = &self.data[idx];
                Some(v)
            }
            Err(_) => None,
        }
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        match self.data.binary_search_by(|(a, _)| a.ord(key)) {
            Ok(idx) => {
                let (k, v) = &self.data[idx];
                Some((k, v))
            }
            Err(_) => None,
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.data.binary_search_by(|(a, _)| a.ord(key)) {
            Ok(idx) => Some(&mut self.data[idx].1),
            Err(_) => None,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.data.binary_search_by(|(a, _)| a.ord(&key)) {
            Ok(idx) => {
                let res = std::mem::replace(&mut self.data[idx], (key, value));
                Some(res.1)
            }
            Err(idx) => {
                self.data.insert(idx, (key, value));
                None
            }
        }
    }

    fn range<'a, R>(&'a self, range: R) -> Box<dyn Iterator<Item = &'a (K, V)> + 'a>
    where
        R: RangeBounds<K> + 'a,
    {
        let mut start = None;

        for i in 0..self.data.len() {
            let (k, _) = &self.data[i];
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
            let (k, _) = &self.data[i];
            if range.contains(k) {
                return Box::new(self.data[start..i + 1].iter());
            }
        }

        Box::new(self.data[start..].iter())
    }

    fn range_mut<'a, R>(&'a mut self, range: R) -> Box<dyn Iterator<Item = &'a mut (K, V)> + 'a>
    where
        R: RangeBounds<K> + 'a,
    {
        let mut start = None;

        for i in 0..self.data.len() {
            let (k, _) = &self.data[i];
            if range.contains(k) {
                start = Some(i);
                break;
            }
        }

        let start = match start {
            Some(i) => i,
            None => return Box::new(self.data[0..0].iter_mut()),
        };

        for i in (start..self.data.len()).rev() {
            let (k, _) = &self.data[i];
            if range.contains(k) {
                return Box::new(self.data[start..i + 1].iter_mut());
            }
        }

        Box::new(self.data[start..].iter_mut())
    }

    fn remove(&mut self, key: &K) -> Option<(K, V)> {
        match self.data.binary_search_by(|(a, _)| a.ord(key)) {
            Ok(idx) => Some(self.data.remove(idx)),
            Err(_) => None,
        }
    }

    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        let mut deleted = 0;

        for i in 0..self.data.len() {
            let (k, v) = &mut self.data[i - deleted];

            if !f(k, v) {
                self.data.remove(i - deleted);
                deleted += 1;
            }
        }
    }
}

impl<K, V> std::fmt::Debug for VecMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.data.iter().map(|(ref k, ref v)| (k, v)))
            .finish()
    }
}
