use ::{UnionFind, Size};

pub fn union_find<T: UnionFind<Size>>() {
    let mut uf = T::new(100);
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

mod quick_union {
    #[test]
    fn union_find() {
        super::union_find::<::QuickUnionUf<::Size>>();
    }
}
mod quick_find {
    #[test]
    fn union_find() {
        super::union_find::<::QuickFindUf<::Size>>();
    }
}
