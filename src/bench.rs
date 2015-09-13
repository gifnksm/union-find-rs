extern crate test;

use std::io::{BufRead, BufReader};
use std::fs::File;
use ::{UnionFind, UfValue};

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

fn union<T, V> (uf: &mut T, conn: &[(usize, usize)])
    where T: UnionFind<V>, V: UfValue
{
    for &(p, q) in conn { uf.union(p, q); }
}

pub mod union {
    use ::bench::test::Bencher;
    use ::{UfValue, UnionFind};
    use std::mem;

    fn do_benchmark<T, V>(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)])
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let uf = T::new(size);
        bencher.bytes = (conn.len() * mem::size_of::<usize>()) as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            super::union(&mut uf, conn);
            uf
        });
    }

    fn do_benchmark2<T, V>(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)])
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(size);
        super::union(&mut uf, conn);
        bencher.bytes = (conn.len() * mem::size_of::<usize>()) as u64;
        bencher.iter(|| {
            let mut uf = uf.clone();
            super::union(&mut uf, conn);
            uf
        });
    }

    pub fn tiny<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/tinyUF.txt");
        do_benchmark::<T, V>(bencher, size, &conn);
    }
    pub fn medium<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/mediumUF.txt");
        do_benchmark::<T, V>(bencher, size, &conn);
    }
    pub fn large<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/largeUF.txt");
        do_benchmark::<T, V>(bencher, size, &conn);
    }
    pub fn tiny2<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/tinyUF.txt");
        do_benchmark2::<T, V>(bencher, size, &conn);
    }
    pub fn medium2<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/mediumUF.txt");
        do_benchmark2::<T, V>(bencher, size, &conn);
    }
    pub fn large2<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/largeUF.txt");
        do_benchmark2::<T, V>(bencher, size, &conn);
    }
}

pub mod find {
    use ::bench::test::Bencher;
    use ::{UnionFind, UfValue};
    use std::mem;

    fn do_benchmark<T, V>(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)])
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(size);
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
    fn do_benchmark2<T, V>(bencher: &mut Bencher, size: usize, conn: &[(usize, usize)])
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let mut uf = T::new(size);
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
    pub fn tiny<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/tinyUF.txt");
        do_benchmark::<T, V>(bencher, size, &conn);
    }
    pub fn medium<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/mediumUF.txt");
        do_benchmark::<T, V>(bencher, size, &conn);
    }
    pub fn large<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/largeUF.txt");
        do_benchmark::<T, V>(bencher, size, &conn);
    }
    pub fn tiny2<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/tinyUF.txt");
        do_benchmark2::<T, V>(bencher, size, &conn);
    }
    pub fn medium2<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/mediumUF.txt");
        do_benchmark2::<T, V>(bencher, size, &conn);
    }
    pub fn large2<T, V>(bencher: &mut Bencher)
        where T: UnionFind<V> + Clone, V: UfValue
    {
        let (size, conn) = super::read_file("etc/largeUF.txt");
        do_benchmark2::<T, V>(bencher, size, &conn);
    }
}
