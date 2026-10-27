use std::{
    fmt,
    hash::Hash,
    iter::Map,
    marker::PhantomData,
    ops::{Index, IndexMut},
    range::{Range, RangeIter},
};

use la_arena::{Idx, RawIdx};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

#[derive(Debug)]
pub(crate) struct LayeredArena<T: 'static> {
    base: &'static [T],
    local: Vec<T>,
}

impl<T> Default for LayeredArena<T> {
    fn default() -> Self {
        LayeredArena {
            base: &[],
            local: vec![],
        }
    }
}

impl<T> LayeredArena<T> {
    pub(crate) fn on_top_of(base: &'static LayeredArena<T>) -> Self {
        assert!(base.base.is_empty());
        LayeredArena {
            base: &base.local,
            local: vec![],
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.base.len() + self.local.len()
    }

    pub(crate) fn alloc(&mut self, value: T) -> Idx<T> {
        let idx = idx(self.len());
        self.local.push(value);
        idx
    }

    pub(crate) fn alloc_many(&mut self, values: impl IntoIterator<Item = T>) -> IdxRange<T> {
        let start = idx(self.len());
        self.local.extend(values);
        IdxRange::new(start, self.next_idx())
    }

    pub(crate) fn next_idx(&self) -> Idx<T> {
        idx(self.len())
    }

    pub(crate) fn range_from(&self, start: Idx<T>) -> IdxRange<T> {
        IdxRange::new(start, self.next_idx())
    }

    pub(crate) fn alloc_copies(&mut self, ids: impl IntoIterator<Item = Idx<T>>) -> IdxRange<T>
    where
        T: Copy,
    {
        self.alloc_mapped_copies(ids, |value| value)
    }

    pub(crate) fn alloc_mapped_copies(
        &mut self,
        ids: impl IntoIterator<Item = Idx<T>>,
        f: impl Fn(T) -> T,
    ) -> IdxRange<T>
    where
        T: Copy,
    {
        let start = idx(self.len());
        for id in ids {
            let value = f(self[id]);
            self.local.push(value);
        }
        IdxRange::new(start, self.next_idx())
    }

    pub(crate) fn is_local(&self, idx: Idx<T>) -> bool {
        idx.into_raw().into_u32() as usize >= self.base.len()
    }

    pub(crate) fn get_local_mut(&mut self, idx: Idx<T>) -> Option<&mut T> {
        let i = idx.into_raw().into_u32() as usize;
        self.local.get_mut(i.checked_sub(self.base.len())?)
    }

    pub(crate) fn iter(&self) -> impl DoubleEndedIterator<Item = (Idx<T>, &T)> {
        self.base
            .iter()
            .enumerate()
            .map(|(i, value)| (idx(i), value))
            .chain(self.iter_local())
    }

    pub(crate) fn iter_local(&self) -> impl DoubleEndedIterator<Item = (Idx<T>, &T)> {
        let offset = self.base.len();
        self.local
            .iter()
            .enumerate()
            .map(move |(i, value)| (idx(offset + i), value))
    }

    pub(crate) fn iter_local_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.local.iter_mut()
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.local.shrink_to_fit();
    }
}

impl<T> Index<Idx<T>> for LayeredArena<T> {
    type Output = T;

    fn index(&self, idx: Idx<T>) -> &T {
        let i = idx.into_raw().into_u32() as usize;
        match i.checked_sub(self.base.len()) {
            Some(local) => &self.local[local],
            None => &self.base[i],
        }
    }
}

impl<T> IndexMut<Idx<T>> for LayeredArena<T> {
    fn index_mut(&mut self, idx: Idx<T>) -> &mut T {
        self.get_local_mut(idx).expect("base layer is immutable")
    }
}

#[derive(Debug)]
pub(crate) struct LayeredMultimap<K: 'static, V: 'static> {
    base: Option<&'static FxHashMap<K, SmallVec<[V; 1]>>>,
    local: FxHashMap<K, SmallVec<[V; 1]>>,
}

impl<K, V> Default for LayeredMultimap<K, V> {
    fn default() -> Self {
        LayeredMultimap {
            base: None,
            local: FxHashMap::default(),
        }
    }
}

impl<K: Copy + Eq + Hash, V: Copy + Ord> LayeredMultimap<K, V> {
    pub(crate) fn on_top_of(base: &'static LayeredMultimap<K, V>) -> Self {
        LayeredMultimap {
            base: Some(&base.local),
            local: FxHashMap::default(),
        }
    }

    pub(crate) fn get(&self, key: K) -> impl DoubleEndedIterator<Item = V> + '_ {
        let local = self.local.get(&key).into_iter().flatten().rev();
        let base = self
            .base
            .and_then(|base| base.get(&key))
            .into_iter()
            .flatten()
            .rev();
        local.chain(base).copied()
    }

    pub(crate) fn insert(&mut self, key: K, id: V) {
        let ids = self.local.entry(key).or_default();
        let position = ids.partition_point(|existing| *existing < id);
        if ids.get(position) != Some(&id) {
            ids.insert(position, id);
        }
    }

    pub(crate) fn remove(&mut self, key: K, id: V) {
        if let Some(ids) = self.local.get_mut(&key) {
            ids.retain(|existing| *existing != id);
            if ids.is_empty() {
                self.local.remove(&key);
            }
        }
    }

    pub(crate) fn take(&mut self, key: K) -> SmallVec<[V; 1]> {
        self.local.remove(&key).unwrap_or_default()
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.local.shrink_to_fit();
    }
}

fn idx<T>(i: usize) -> Idx<T> {
    Idx::from_raw(RawIdx::from_u32(u32::try_from(i).unwrap()))
}

pub(crate) struct IdxRange<T> {
    // using std::Range is copy vs the crates.io version
    range: Range<u32>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> IdxRange<T> {
    pub(crate) fn new(start: Idx<T>, end: Idx<T>) -> Self {
        IdxRange {
            range: Range::from(start.into_raw().into_u32()..end.into_raw().into_u32()),
            _ty: PhantomData,
        }
    }

    pub(crate) fn single(idx: Idx<T>) -> Self {
        let start = idx.into_raw().into_u32();
        IdxRange {
            range: Range::from(start..start + 1),
            _ty: PhantomData,
        }
    }

    pub(crate) fn start(self) -> Idx<T> {
        Idx::from_raw(RawIdx::from_u32(self.range.start))
    }

    pub(crate) fn len(self) -> usize {
        (self.range.end - self.range.start) as usize
    }

    pub(crate) fn is_empty(self) -> bool {
        self.range.is_empty()
    }

    pub(crate) fn iter(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<T> IntoIterator for IdxRange<T> {
    type Item = Idx<T>;
    type IntoIter = Map<RangeIter<u32>, fn(u32) -> Idx<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.range
            .into_iter()
            .map(|raw| Idx::from_raw(RawIdx::from_u32(raw)))
    }
}

impl<T> Clone for IdxRange<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for IdxRange<T> {}

impl<T> PartialEq for IdxRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl<T> Eq for IdxRange<T> {}

impl<T> fmt::Debug for IdxRange<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IdxRange::<{}>({:?})",
            std::any::type_name::<T>(),
            self.range
        )
    }
}
