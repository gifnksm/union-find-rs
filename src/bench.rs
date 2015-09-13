extern crate test;

use std::io::{BufRead, BufReader};
use std::fs::File;
use ::{UnionFind, UfValue};
use self::test::Bencher;

lazy_static! {
    pub static ref TINY: Input = Input::from_file("etc/tinyUF.txt");
    pub static ref MEDIUM: Input = Input::from_file("etc/mediumUF.txt");
    pub static ref LARGE: Input = Input::from_file("etc/largeUF.txt");
}

#[derive(Clone, Debug)]
pub struct Input {
    size: usize,
    conn: Vec<(usize, usize)>
}

impl Input {
    fn from_file(name: &str) -> Input {
        let mut reader = BufReader::new(File::open(name).unwrap());
        let mut buf = String::new();

        let _ = reader.read_line(&mut buf).unwrap();
        let size = buf.trim().parse::<usize>().unwrap();
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

        Input { size: size, conn: conn }
    }

    fn union<T, V>(&self, uf: &mut T)
        where T: UnionFind<V>, V: UfValue
    {
        for &(p, q) in &self.conn {
            uf.union(p, q);
        }
    }
    fn find_all<T, V>(&self, uf: &mut T)
        where T: UnionFind<V>, V: UfValue
    {
        for i in 0..uf.size() {
            let _ = uf.find(i);
        }
    }

    pub fn bench_union1<T, V>(&self, bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let uf = T::new(self.size);
        bencher.bytes = self.conn.len() as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            self.union(&mut uf);
            uf
        });
    }
    pub fn bench_union2<T, V>(&self, bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(self.size);
        self.union(&mut uf);
        bencher.bytes = self.conn.len() as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            self.union(&mut uf);
            uf
        });
    }
    pub fn bench_union3<T, V>(&self, bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(self.size);
        self.union(&mut uf);
        self.union(&mut uf);
        bencher.bytes = self.conn.len() as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            self.union(&mut uf);
            uf
        });
    }

    pub fn bench_find1<T, V>(&self, bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(self.size);
        self.union(&mut uf);
        bencher.bytes = self.size as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            self.find_all(&mut uf);
            uf
        });
    }
    pub fn bench_find2<T, V>(&self, bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(self.size);
        self.union(&mut uf);
        self.find_all(&mut uf);
        bencher.bytes = self.size as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            self.find_all(&mut uf);
            uf
        });
    }
    pub fn bench_find3<T, V>(&self, bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(self.size);
        self.union(&mut uf);
        self.find_all(&mut uf);
        self.find_all(&mut uf);
        bencher.bytes = self.size as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            self.find_all(&mut uf);
            uf
        });
    }
}

macro_rules! bench_fns_for_type_with_input {
    ($ty:ty, $input:path) => {
        #[bench]
        fn union1(bencher: &mut ::bench::test::Bencher) {
            $input.bench_union1::<$ty, _>(bencher);
        }
        #[bench]
        fn union2(bencher: &mut ::bench::test::Bencher) {
            $input.bench_union2::<$ty, _>(bencher);
        }
        #[bench]
        fn union3(bencher: &mut ::bench::test::Bencher) {
            $input.bench_union3::<$ty, _>(bencher);
        }
        #[bench]
        fn find1(bencher: &mut ::bench::test::Bencher) {
            $input.bench_find1::<$ty, _>(bencher);
        }
        #[bench]
        fn find2(bencher: &mut ::bench::test::Bencher) {
            $input.bench_find2::<$ty, _>(bencher);
        }
        #[bench]
        fn find3(bencher: &mut ::bench::test::Bencher) {
            $input.bench_find3::<$ty, _>(bencher);
        }
    }
}

macro_rules! bench_fns_for_type {
    ($ty:ty) => {
        mod tiny { bench_fns_for_type_with_input!($ty, ::bench::TINY); }
        mod medium { bench_fns_for_type_with_input!($ty, ::bench::MEDIUM); }
        mod large { bench_fns_for_type_with_input!($ty, ::bench::LARGE); }
    }
}

mod quick_union { bench_fns_for_type!(::QuickUnionUf<::Size>); }
mod quick_find {
    // bench_fns_for_type!(::QuickFindUf<::Size>);
    mod tiny { bench_fns_for_type_with_input!(::QuickFindUf<::Size>, ::bench::TINY); }
    mod medium { bench_fns_for_type_with_input!(::QuickFindUf<::Size>, ::bench::MEDIUM); }
    // large is too large to execute
    // mod large { bench_fns_for_type_with_input!(::QuickFindUf<::Size>, ::bench::LARGE); }
}
