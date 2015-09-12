//! Struct and methods for union-find operation.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![cfg_attr(all(test, feature = "nightly"), feature(test))]

use std::mem;
use std::default::Default;
use std::iter::{IntoIterator, FromIterator};

/// The value that can be contained with `UFValue`.
pub trait UFValue: Sized {
    /// Merge two value into one.
    ///
    /// This is used by `UnionFind::union` operation.
    fn merge(lval: Self, rval: Self) -> Merge<Self>;
}

/// Return value of the `UFValue::merege`
#[allow(missing_docs)]
pub enum Merge<T> {
    Left(T), Right(T)
}

/// Reperesents the size of the group.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Size(usize);

impl UFValue for Size {
    fn merge(Size(lval): Size, Size(rval): Size) -> Merge<Size> {
        if lval >= rval {
            Merge::Left(Size(lval + rval))
        } else {
            Merge::Right(Size(lval + rval))
        }
    }
}

impl Default for Size {
    fn default() -> Size { Size(1) }
}

/// Struct for union-find operation.
#[derive(Clone, Debug)]
pub struct UnionFind<V = Size> {
    parent: Vec<usize>,
    data: Vec<Option<V>>
}

impl<T: UFValue = Size> UnionFind<T> {
    /// Creates empty `UnionFind` struct.
    #[inline]
    pub fn new(len: usize) -> UnionFind<T> where T: Default {
        Self::from_iter((0..len).map(|_| Default::default()))
    }

    /// Returns the size of `self`.
    #[inline]
    pub fn size(&self) -> usize { self.data.len() }

    /// Join two sets that contains given keys (Union operation).
    ///
    /// Returns `true` if these keys are belonged to different sets.
    #[inline]
    pub fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.find(key0);
        let k1 = self.find(key1);
        if k0 == k1 { return false; }

        // Temporary replace with dummy to move out the elements of the vector.
        let v0 = mem::replace(&mut self.data[k0], None).unwrap();
        let v1 = mem::replace(&mut self.data[k1], None).unwrap();

        let (parent, child, val) = match UFValue::merge(v0, v1) {
            Merge::Left(val) => (k0, k1, val),
            Merge::Right(val) => (k1, k0, val)
        };
        self.data[parent] = Some(val);
        self.parent[child] = parent;

        true
    }

    /// Returns the identifier of the set that the key belongs to.
    #[inline]
    pub fn find(&mut self, key: usize) -> usize {
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
    pub fn get(&mut self, key: usize) -> &T {
        let root_key = self.find(key);
        self.data[root_key].as_ref().unwrap()
    }

    /// Returns the mutable reference to the value of the set that the key belongs to.
    #[inline]
    pub fn get_mut(&mut self, key: usize) -> &mut T {
        let root_key = self.find(key);
        self.data[root_key].as_mut().unwrap()
    }
}

impl<A: UFValue> FromIterator<A> for UnionFind<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item=A>>(iterator: T) -> UnionFind<A> {
        let data = iterator.into_iter().map(Some).collect::<Vec<_>>();
        UnionFind {
            parent: (0..data.len()).collect(),
            data: data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UnionFind, Size};

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(100);
        assert_eq!(&Size(1), uf.get(0));
        assert_eq!(&Size(1), uf.get(1));
        assert!(uf.find(0) != uf.find(1));
        assert!(uf.find(1) != uf.find(2));
        assert!(uf.union(0, 1));
        assert!(uf.find(0) == uf.find(1));
        assert_eq!(&Size(2), uf.get(0));
        assert_eq!(&Size(2), uf.get(1));
        assert_eq!(&Size(1), uf.get(2));
        assert!(!uf.union(0, 1));
        assert_eq!(&Size(2), uf.get(0));
        assert_eq!(&Size(2), uf.get(1));
        assert_eq!(&Size(1), uf.get(2));
        assert!(uf.union(1, 2));
        assert_eq!(&Size(3), uf.get(0));
        assert_eq!(&Size(3), uf.get(1));
        assert_eq!(&Size(3), uf.get(2));
        assert!(uf.find(0) == uf.find(1));
        assert!(uf.find(2) == uf.find(1));
    }
}

#[cfg(all(test, feature = "nightly"))]
mod bench {
    extern crate test;

    use std::io::{BufRead, BufReader};
    use std::fs::File;
    use ::{UnionFind, Size};

    fn read_file(name: &str) -> (usize, Vec<(usize, usize)>) {
        let mut reader = BufReader::new(File::open(name).unwrap());
        let mut buf = String::new();

        let _ = reader.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        let mut conn = vec![];

        while reader.read_line(&mut buf).unwrap() > 0 {
            {
                let mut sp = buf.trim().split_whitespace();
                let a = sp.next().unwrap().parse::<usize>().unwrap();
                let b = sp.next().unwrap().parse::<usize>().unwrap();
                conn.push((a, b));
            }

            buf.clear();
        }

        (num, conn)
    }

    fn union(uf: &mut UnionFind<Size>, conn: &[(usize, usize)]) {
        for &(p, q) in conn { uf.union(p, q); }
    }

    mod union {
        use ::bench::test::Bencher;
        use ::{UnionFind, Size};
        use std::mem;

        fn do_benchmark(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)]) {
            let uf = UnionFind::<Size>::new(size);
            bencher.bytes = (conn.len() * mem::size_of::<usize>()) as u64;
            bencher.iter(|| {
                let mut uf = uf.clone();
                super::union(&mut uf, conn);
                uf
            });
        }

        fn do_benchmark2(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)]) {
            let mut uf = UnionFind::<Size>::new(size);
            super::union(&mut uf, conn);
            bencher.bytes = (conn.len() * mem::size_of::<usize>()) as u64;
            bencher.iter(|| {
                let mut uf = uf.clone();
                super::union(&mut uf, conn);
                uf
            });
        }

        #[bench]
        fn tiny(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/tinyUF.txt");
            do_benchmark(bencher, size, &conn);
        }
        #[bench]
        fn medium(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/mediumUF.txt");
            do_benchmark(bencher, size, &conn);
        }
        #[bench]
        fn large(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/largeUF.txt");
            do_benchmark(bencher, size, &conn);
        }
        #[bench]
        fn tiny2(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/tinyUF.txt");
            do_benchmark2(bencher, size, &conn);
        }
        #[bench]
        fn medium2(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/mediumUF.txt");
            do_benchmark2(bencher, size, &conn);
        }
        #[bench]
        fn large2(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/largeUF.txt");
            do_benchmark2(bencher, size, &conn);
        }
    }

    mod find {
        use ::bench::test::Bencher;
        use ::{UnionFind, Size};
        use std::mem;

        fn do_benchmark(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)]) {
            let mut uf = UnionFind::<Size>::new(size);
            super::union(&mut uf, conn);
            bencher.bytes = (size * mem::size_of::<usize>()) as u64;
            bencher.iter(|| {
                let mut uf = uf.clone();
                for i in 0..uf.size() {
                    let _ = uf.find(i);
                }
                uf
            });
        }
        fn do_benchmark2(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)]) {
            let mut uf = UnionFind::<Size>::new(size);
            super::union(&mut uf, conn);
            for i in 0..uf.size() {
                let _ = uf.find(i);
            }
            bencher.bytes = (size * mem::size_of::<usize>()) as u64;
            bencher.iter(|| {
                let mut uf = uf.clone();
                for i in 0..uf.size() {
                    let _ = uf.find(i);
                }
                uf
            });
        }
        #[bench]
        fn tiny(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/tinyUF.txt");
            do_benchmark(bencher, size, &conn);
        }
        #[bench]
        fn medium(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/mediumUF.txt");
            do_benchmark(bencher, size, &conn);
        }
        #[bench]
        fn large(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/largeUF.txt");
            do_benchmark(bencher, size, &conn);
        }
        #[bench]
        fn tiny2(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/tinyUF.txt");
            do_benchmark2(bencher, size, &conn);
        }
        #[bench]
        fn medium2(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/mediumUF.txt");
            do_benchmark2(bencher, size, &conn);
        }
        #[bench]
        fn large2(bencher: &mut Bencher) {
            let (size, conn) = super::read_file("etc/largeUF.txt");
            do_benchmark2(bencher, size, &conn);
        }
    }
}
