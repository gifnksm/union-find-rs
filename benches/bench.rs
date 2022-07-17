// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate criterion;
extern crate lazy_static;
extern crate union_find;

use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader};
use union_find::{
    QuickFindUf, QuickUnionUf, Union, UnionByRank, UnionByRankSize, UnionBySize, UnionBySizeRank,
    UnionFind,
};

lazy_static! {
    static ref TINY: Input = Input::from_file("tiny", "etc/tinyUF.txt");
    static ref MEDIUM: Input = Input::from_file("medium", "etc/mediumUF.txt");
    static ref LARGE: Input = Input::from_file("large", "etc/largeUF.txt");
}

struct Cache<'a, T> {
    input: &'a Input,
    init: Option<T>,
    union1: Option<T>,
    union2: Option<T>,
    find1: Option<T>,
    find2: Option<T>,
}

impl<'a, T> Cache<'a, T> {
    fn new(input: &'a Input) -> Cache<'a, T> {
        Cache {
            input,
            init: None,
            union1: None,
            union2: None,
            find1: None,
            find2: None,
        }
    }

    fn init<V>(&mut self) -> &T
    where
        T: UnionFind<V>,
        V: Union + Default,
    {
        if self.init.is_none() {
            self.init = Some(self.input.init());
        }
        self.init.as_ref().unwrap()
    }

    fn union1<V>(&mut self) -> &T
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        if self.union1.is_none() {
            let mut uf = self.init().clone();
            self.input.union(&mut uf);
            self.union1 = Some(uf);
        }
        self.union1.as_ref().unwrap()
    }

    fn union2<V>(&mut self) -> &T
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        if self.union2.is_none() {
            let mut uf = self.union1().clone();
            self.input.union(&mut uf);
            self.union2 = Some(uf);
        }
        self.union2.as_ref().unwrap()
    }

    fn find1<V>(&mut self) -> &T
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        if self.find1.is_none() {
            let mut uf = self.union1().clone();
            self.input.find_all(&mut uf);
            self.find1 = Some(uf);
        }
        self.find1.as_ref().unwrap()
    }

    fn find2<V>(&mut self) -> &T
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        if self.find2.is_none() {
            let mut uf = self.union1().clone();
            self.input.find_all(&mut uf);
            self.find2 = Some(uf);
        }
        self.find2.as_ref().unwrap()
    }
}

#[derive(Clone, Debug)]
struct Input {
    name: &'static str,
    size: usize,
    conn: Vec<(usize, usize)>,
}

impl Input {
    fn from_file(name: &'static str, file_name: &str) -> Input {
        let mut reader = BufReader::new(File::open(file_name).unwrap());
        let mut buf = String::new();

        let _ = reader.read_line(&mut buf).unwrap();
        let size = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        let mut conn = vec![];

        while reader.read_line(&mut buf).unwrap() > 0 {
            {
                let mut sp = buf.split_whitespace();
                let a = sp.next().unwrap().parse::<usize>().unwrap();
                let b = sp.next().unwrap().parse::<usize>().unwrap();
                conn.push((a, b));
            }

            buf.clear();
        }

        Input { name, size, conn }
    }

    fn init<T, V>(&self) -> T
    where
        T: UnionFind<V>,
        V: Union + Default,
    {
        T::new(self.size)
    }

    fn union<T, V>(&self, uf: &mut T)
    where
        T: UnionFind<V>,
        V: Union,
    {
        for &(p, q) in &self.conn {
            uf.union(p, q);
        }
    }
    fn find_all<T, V>(&self, uf: &mut T)
    where
        T: UnionFind<V>,
        V: Union,
    {
        for i in 0..uf.size() {
            let _ = uf.find(i);
        }
    }

    fn bench_clone_from<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "clone_from");
        let base = cache.init();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
            })
        });
    }

    fn bench_union1<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "union1");
        let base = cache.init();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
                self.union(&mut uf);
            })
        });
    }
    fn bench_union2<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "union2");
        let base = cache.union1();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
                self.union(&mut uf);
            })
        });
    }
    fn bench_union3<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "union3");
        let base = cache.union2();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
                self.union(&mut uf);
            })
        });
    }

    fn bench_find1<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "find1");
        let base = cache.union1();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
                self.find_all(&mut uf);
            })
        });
    }
    fn bench_find2<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "find2");
        let base = cache.find1();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
                self.find_all(&mut uf);
            })
        });
    }
    fn bench_find3<T, V>(&self, c: &mut Criterion, category: &str, cache: &mut Cache<T>)
    where
        T: UnionFind<V> + Clone,
        V: Union + Default,
    {
        let id = format!("{}::{}::{}", category, self.name, "find3");
        let base = cache.find2();
        let mut uf = base.clone();
        c.bench_function(&id, |b| {
            b.iter(|| {
                uf.clone_from(base);
                self.find_all(&mut uf);
            })
        });
    }
}

fn bench_for_type<T, V>(c: &mut Criterion, category: &str, inputs: &[Input])
where
    T: UnionFind<V> + Clone,
    V: Union + Default,
{
    for input in inputs {
        let mut cache = Cache::<T>::new(input);
        input.bench_clone_from(c, category, &mut cache);
        input.bench_union1(c, category, &mut cache);
        input.bench_union2(c, category, &mut cache);
        input.bench_union3(c, category, &mut cache);
        input.bench_find1(c, category, &mut cache);
        input.bench_find2(c, category, &mut cache);
        input.bench_find3(c, category, &mut cache);
    }
}

fn bench(c: &mut Criterion) {
    let inputs = &[
        Input::from_file("tiny", "etc/tinyUF.txt"),
        Input::from_file("medium", "etc/mediumUF.txt"),
        Input::from_file("large", "etc/largeUF.txt"),
    ];

    bench_for_type::<QuickUnionUf<UnionBySize>, _>(c, "quick_union::by_size", inputs);
    bench_for_type::<QuickUnionUf<UnionByRank>, _>(c, "quick_union::by_rank", inputs);
    bench_for_type::<QuickUnionUf<UnionBySizeRank>, _>(c, "quick_union::by_size_rank", inputs);
    bench_for_type::<QuickUnionUf<UnionByRankSize>, _>(c, "quick_union::by_rank_size", inputs);
    bench_for_type::<QuickFindUf<UnionBySize>, _>(c, "quick_find::by_size", inputs);
    bench_for_type::<QuickFindUf<UnionByRank>, _>(c, "quick_find::by_rank", inputs);
    bench_for_type::<QuickFindUf<UnionBySizeRank>, _>(c, "quick_find::by_size_rank", inputs);
    bench_for_type::<QuickFindUf<UnionByRankSize>, _>(c, "quick_find::by_rank_size", inputs);
}

criterion_group!(benches, bench);
criterion_main!(benches);
