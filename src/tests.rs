use ::{UnionFind, UnionBySize};

pub fn union_find<T>()
    where T: UnionFind<UnionBySize>
{
    let mut uf = T::new(100);
    assert_eq!(1, uf.get(0).size());
    assert_eq!(1, uf.get(1).size());
    assert!(uf.find(0) != uf.find(1));
    assert!(uf.find(1) != uf.find(2));
    assert!(uf.union(0, 1));
    assert!(uf.find(0) == uf.find(1));
    assert_eq!(2, uf.get(0).size());
    assert_eq!(2, uf.get(1).size());
    assert_eq!(1, uf.get(2).size());
    assert!(!uf.union(0, 1));
    assert_eq!(2, uf.get(0).size());
    assert_eq!(2, uf.get(1).size());
    assert_eq!(1, uf.get(2).size());
    assert!(uf.union(1, 2));
    assert_eq!(3, uf.get(0).size());
    assert_eq!(3, uf.get(1).size());
    assert_eq!(3, uf.get(2).size());
    assert!(uf.find(0) == uf.find(1));
    assert!(uf.find(2) == uf.find(1));
}

mod quick_union {
    #[test]
    fn union_find() {
        super::union_find::<::QuickUnionUf<::UnionBySize>>();
    }
}
mod quick_find {
    #[test]
    fn union_find() {
        super::union_find::<::QuickFindUf<::UnionBySize>>();
    }
}
