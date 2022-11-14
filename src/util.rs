use std::cmp::Ordering;
use std::ops::RangeBounds;

pub trait Float {
    fn ord(&self, other: &Self) -> Ordering;
}

impl Float for f32 {
    fn ord(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => match (self.is_nan(), other.is_nan()) {
                (true, true) => Ordering::Less,
                (true, _) => Ordering::Greater,
                (_, true) => Ordering::Less,
                (_, _) => unreachable!(),
            },
        }
    }
}

impl Float for f64 {
    fn ord(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => match (self.is_nan(), other.is_nan()) {
                (true, true) => Ordering::Less,
                (true, _) => Ordering::Greater,
                (_, true) => Ordering::Less,
                (_, _) => unreachable!(),
            },
        }
    }
}

pub trait FloatMap<K, V>
where
    K: Float + PartialOrd<K>,
{
    fn get(&self, key: &K) -> Option<&V>;

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn range<'a, R>(&'a self, range: R) -> Box<dyn Iterator<Item = &'a (K, V)> + 'a>
    where
        R: RangeBounds<K> + 'a;

    fn range_mut<'a, R>(&'a mut self, range: R) -> Box<dyn Iterator<Item = &'a mut (K, V)> + 'a>
    where
        R: RangeBounds<K> + 'a;

    fn remove(&mut self, key: &K) -> Option<(K, V)>;

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool;
}

pub trait FloatSet<K>
where
    K: Float,
{
    fn contains(&self, key: &K) -> bool;

    fn get(&self, key: &K) -> Option<&K>;

    fn insert(&mut self, key: K);

    fn range<'a, R>(&'a self, range: R) -> Box<dyn Iterator<Item = &'a K> + 'a>
    where
        R: RangeBounds<K> + 'a;

    fn remove(&mut self, key: &K) -> Option<K>;
}
