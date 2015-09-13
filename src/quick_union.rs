use std::iter::FromIterator;
use std::mem;
use {UfValue, UnionFind, Merge};

/// Union-Find implementation with quick union operation.
#[derive(Clone, Debug)]
pub struct QuickUnionUf<V> {
    parent: Vec<usize>,
    data: Vec<Option<V>>
}

impl<V: UfValue> UnionFind<V> for QuickUnionUf<V> {
    /// Returns the size of `self`.
    #[inline]
    fn size(&self) -> usize { self.data.len() }

    /// Join two sets that contains given keys (Union operation).
    ///
    /// Returns `true` if these keys are belonged to different sets.
    #[inline]
    fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.find(key0);
        let k1 = self.find(key1);
        if k0 == k1 { return false; }

        // Temporary replace with dummy to move out the elements of the vector.
        let v0 = mem::replace(&mut self.data[k0], None).unwrap();
        let v1 = mem::replace(&mut self.data[k1], None).unwrap();

        let (parent, child, val) = match UfValue::merge(v0, v1) {
            Merge::Left(val) => (k0, k1, val),
            Merge::Right(val) => (k1, k0, val)
        };
        self.data[parent] = Some(val);
        self.parent[child] = parent;

        true
    }

    /// Returns the identifier of the set that the key belongs to.
    #[inline]
    fn find(&mut self, key: usize) -> usize {
        let p = self.parent[key];
        if key == p { return key }

        let mut cur_key = p;
        loop {
            let p = self.parent[cur_key];
            if p == cur_key { break }

            self.parent[cur_key] = self.parent[p];
            cur_key = p;
        }

        self.parent[key] = cur_key;
        cur_key
    }

    /// Returns the reference to the value of the set that the key belongs to.
    #[inline]
    fn get(&mut self, key: usize) -> &V {
        let root_key = self.find(key);
        self.data[root_key].as_ref().unwrap()
    }

    /// Returns the mutable reference to the value of the set that the key belongs to.
    #[inline]
    fn get_mut(&mut self, key: usize) -> &mut V {
        let root_key = self.find(key);
        self.data[root_key].as_mut().unwrap()
    }
}

impl<A: UfValue> FromIterator<A> for QuickUnionUf<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item=A>>(iterator: T) -> QuickUnionUf<A> {
        let data = iterator.into_iter().map(Some).collect::<Vec<_>>();
        QuickUnionUf {
            parent: (0..data.len()).collect(),
            data: data
        }
    }
}

#[cfg(test)]
mod tests {
    use ::{Size, QuickUnionUf};

    #[test] fn union_find() { ::tests::union_find::<QuickUnionUf<Size>>(); }
}

#[cfg(all(test, feature = "nightly"))]
mod bench {
    mod union {
        use ::bench::test::Bencher;
        use ::{Size, QuickUnionUf};

        #[bench]
        fn tiny(bencher: &mut Bencher) {
            ::bench::union::tiny::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn medium(bencher: &mut Bencher) {
            ::bench::union::medium::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn large(bencher: &mut Bencher) {
            ::bench::union::large::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn tiny2(bencher: &mut Bencher) {
            ::bench::union::tiny2::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn medium2(bencher: &mut Bencher) {
            ::bench::union::medium2::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn large2(bencher: &mut Bencher) {
            ::bench::union::large2::<QuickUnionUf<Size>, _>(bencher);
        }
    }
    mod find {
        use ::bench::test::Bencher;
        use ::{Size, QuickUnionUf};

        #[bench]
        fn tiny(bencher: &mut Bencher) {
            ::bench::find::tiny::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn medium(bencher: &mut Bencher) {
            ::bench::find::medium::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn large(bencher: &mut Bencher) {
            ::bench::find::large::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn tiny2(bencher: &mut Bencher) {
            ::bench::find::tiny2::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn medium2(bencher: &mut Bencher) {
            ::bench::find::medium2::<QuickUnionUf<Size>, _>(bencher);
        }
        #[bench]
        fn large2(bencher: &mut Bencher) {
            ::bench::find::large2::<QuickUnionUf<Size>, _>(bencher);
        }
    }
}
