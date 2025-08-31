//! Euler tour tree
//!
//! # References
//! - <https://qiita.com/hotman78/items/78cd3aa50b05a57738d4>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/dynamic_tree_vertex_add_subtree_sum>

use std::collections::HashMap;
use std::ptr;

use crate::algebra::traits::Monoid;

struct Node<M> {
    value: M,
    sum: M,
    size: usize,
    lc: *mut Node<M>,
    rc: *mut Node<M>,
    par: *mut Node<M>,
}

impl<M: Monoid + Clone> Node<M> {
    fn new(value: M) -> Self {
        Self {
            value,
            sum: M::id(),
            size: 1,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn get_sum(this: *mut Self) -> M {
        assert!(!this.is_null());
        unsafe { (*this).sum.clone() }
    }

    fn set_value(this: *mut Self, value: M) {
        assert!(!this.is_null());
        unsafe {
            (*this).value = value;
        }
    }

    fn update_value(this: *mut Self, value: M) {
        assert!(!this.is_null());
        unsafe {
            (*this).value = (*this).value.clone().op(value);
        }
    }

    fn rotate(this: *mut Self) {
        let p = Self::get_par(this).unwrap();
        let pp = Self::get_par(p).unwrap();

        if Self::left_of(p).unwrap() == this {
            let c = Self::right_of(this).unwrap();
            Self::set_left(p, c);
            Self::set_right(this, p);
        } else {
            let c = Self::left_of(this).unwrap();
            Self::set_right(p, c);
            Self::set_left(this, p);
        }

        unsafe {
            if !pp.is_null() {
                if (*pp).lc == p {
                    (*pp).lc = this;
                }
                if (*pp).rc == p {
                    (*pp).rc = this;
                }
            }

            assert!(!this.is_null());
            (*this).par = pp;
        }

        Self::update(p);
        Self::update(this);
    }

    fn status(this: *mut Self) -> i32 {
        let par = Self::get_par(this).unwrap();

        if par.is_null() {
            return 0;
        }
        if unsafe { (*par).lc } == this {
            return 1;
        }
        if unsafe { (*par).rc } == this {
            return -1;
        }

        unreachable!()
    }

    fn pushdown(this: *mut Self) {
        if !this.is_null() {
            Self::update(this);
        }
    }

    fn update(this: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).size = 1 + Self::size_of((*this).lc) + Self::size_of((*this).rc);

            (*this).sum = (*this).value.clone();
            if !(*this).lc.is_null() {
                (*this).sum = M::op(Self::get_sum(this), Self::get_sum((*this).lc));
            }
            if !(*this).rc.is_null() {
                (*this).sum = M::op(Self::get_sum(this), Self::get_sum((*this).rc));
            }
        }
    }

    fn splay(this: *mut Self) {
        while Self::status(this) != 0 {
            let par = Self::get_par(this).unwrap();

            if Self::status(par) == 0 {
                Self::rotate(this);
            } else if Self::status(this) == Self::status(par) {
                Self::rotate(par);
                Self::rotate(this);
            } else {
                Self::rotate(this);
                Self::rotate(this);
            }
        }
    }

    fn get_first(root: *mut Self) -> *mut Self {
        if root.is_null() {
            return root;
        }

        let mut cur = root;

        loop {
            Self::pushdown(cur);

            let left = Self::left_of(cur).unwrap();

            if left.is_null() {
                Self::splay(cur);
                return cur;
            }
            cur = left;
        }
    }

    fn get_last(root: *mut Self) -> *mut Self {
        if root.is_null() {
            return root;
        }

        let mut cur = root;

        loop {
            Self::pushdown(cur);

            let right = Self::right_of(cur).unwrap();

            if right.is_null() {
                Self::splay(cur);
                return cur;
            }
            cur = right;
        }
    }

    fn merge(left: *mut Self, right: *mut Self) -> *mut Self {
        if left.is_null() {
            return right;
        }
        if right.is_null() {
            return left;
        }

        let cur = Self::get_last(left);

        Self::set_right(cur, right);
        Self::update(right);
        Self::update(cur);

        cur
    }

    fn split_left(root: *mut Self) -> (*mut Self, *mut Self) {
        if root.is_null() {
            return (ptr::null_mut(), ptr::null_mut());
        }

        let cur = root;
        let left = Self::left_of(cur).unwrap();

        if !left.is_null() {
            unsafe {
                (*left).par = ptr::null_mut();
            }
            Self::update(left);
        }
        assert!(!cur.is_null());
        unsafe {
            (*cur).lc = ptr::null_mut();
        }
        Self::update(cur);

        (left, cur)
    }

    fn split_right(root: *mut Self) -> (*mut Self, *mut Self) {
        if root.is_null() {
            return (ptr::null_mut(), ptr::null_mut());
        }

        let cur = root;
        let right = Self::right_of(cur).unwrap();

        if !right.is_null() {
            unsafe {
                (*right).par = ptr::null_mut();
            }
            Self::update(right);
        }
        assert!(!cur.is_null());
        unsafe {
            (*cur).rc = ptr::null_mut();
        }
        Self::update(cur);

        (cur, right)
    }

    fn set_left(this: *mut Self, left: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).lc = left;
            if !left.is_null() {
                (*left).par = this;
            }
        }
    }

    fn set_right(this: *mut Self, right: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).rc = right;
            if !right.is_null() {
                (*right).par = this;
            }
        }
    }

    fn size_of(this: *mut Self) -> usize {
        if this.is_null() {
            0
        } else {
            unsafe { (*this).size }
        }
    }

    fn left_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).lc })
    }

    fn right_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).rc })
    }

    fn get_par(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).par })
    }
}

/// Euler tour tree
pub struct EulerTourTree<M> {
    vertices: Vec<*mut Node<M>>,
    edges: Vec<HashMap<usize, *mut Node<M>>>,
}

impl<M: Monoid + Clone> EulerTourTree<M> {
    /// `n`個の頂点のみからなる森を構築する。
    pub fn new(n: usize) -> Self {
        let vertices = (0..n)
            .map(|_| {
                let p = Box::new(Node::new(M::id()));
                Box::into_raw(p)
            })
            .collect::<Vec<_>>();

        let edges = (0..n).map(|i| HashMap::from([(i, vertices[i])])).collect();

        Self { vertices, edges }
    }

    /// 頂点`r`をそれの属する木の根にする。
    pub fn reroot(&mut self, r: usize) {
        let p = self.vertices[r];

        Node::splay(p);
        let (l, r) = Node::split_left(p);
        Node::merge(r, l);
    }

    /// 2つの頂点が同一の木に属するかどうかを判定する。
    pub fn is_same_tree(&self, i: usize, j: usize) -> bool {
        if i == j {
            return true;
        }

        let pi = self.vertices[i];
        let pj = self.vertices[j];

        Node::splay(pi);
        let ri = Node::get_first(pi);

        Node::splay(pj);
        let rj = Node::get_first(pj);

        ptr::eq(ri, rj)
    }

    /// 異なる木にそれぞれ属する2頂点間に辺を張る。
    pub fn link(&mut self, i: usize, j: usize) -> Result<(), &'static str> {
        if self.is_same_tree(i, j) {
            return Err("既に同一の木に属している。");
        }

        self.reroot(i);
        self.reroot(j);

        let pi = self.vertices[i];
        let pj = self.vertices[j];

        Node::splay(pi);
        Node::splay(pj);

        let eij = Box::into_raw(Box::new(Node::new(M::id())));
        self.edges[i].insert(j, eij);

        let eji = Box::into_raw(Box::new(Node::new(M::id())));
        self.edges[j].insert(i, eji);

        let t = Node::merge(pi, eij);
        let t = Node::merge(t, pj);
        Node::merge(t, eji);

        Ok(())
    }

    /// 2頂点間を張る辺を削除する。
    pub fn cut(&mut self, i: usize, j: usize) -> Result<(), &'static str> {
        if i == j {
            return Err("同一頂点で`cut`は不可。");
        }
        match (self.edges[i].get(&j), self.edges[j].get(&i)) {
            (Some(&eij), Some(&eji)) => {
                self.reroot(i);

                Node::splay(eij);
                let (s, a) = Node::split_left(eij);
                Node::split_right(a);

                Node::splay(eji);
                let (_, a) = Node::split_left(eji);
                let (_, u) = Node::split_right(a);

                Node::merge(s, u);

                self.edges[i].remove(&j);
                self.edges[j].remove(&i);

                unsafe {
                    let _ = Box::from_raw(eij);
                    let _ = Box::from_raw(eji);
                }
            }
            _ => return Err("2頂点をつなぐ辺が存在しない。"),
        }

        Ok(())
    }

    /// 頂点`i`の値を`value`に設定する。
    pub fn set(&mut self, i: usize, value: M) {
        let p = self.vertices[i];
        Node::splay(p);
        Node::set_value(p, value);
        Node::pushdown(p);
    }

    /// 頂点`i`の値をモノイドの演算と値`value`で更新する。
    pub fn update(&mut self, i: usize, value: M) {
        let p = self.vertices[i];
        Node::splay(p);
        Node::update_value(p, value);
        Node::pushdown(p);
    }

    /// 頂点`p`を親とする頂点`v`について、`v`を根とする部分木の値を集積して返す。
    pub fn subtree_sum(&mut self, v: usize, p: usize) -> Result<M, &'static str> {
        self.cut(v, p)?;

        let rv = self.vertices[v];
        Node::splay(rv);
        let ret = Node::get_sum(rv);

        self.link(v, p)?;

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::trivial::Trivial;

    use super::*;

    #[test]
    fn test() {
        let mut ett = EulerTourTree::<Trivial>::new(10);

        ett.link(1, 2).unwrap();
        ett.link(3, 5).unwrap();
        ett.link(1, 5).unwrap();
        ett.reroot(2);

        ett.cut(1, 2).unwrap();
    }
}
