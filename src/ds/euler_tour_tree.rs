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

struct Manager<M> {
    monoid: M,
}

struct Node<M: Monoid> {
    value: M::Element,
    sum: M::Element,
    size: usize,
    lc: *mut Self,
    rc: *mut Self,
    par: *mut Self,
}

impl<M: Monoid> Manager<M>
where
    M::Element: Clone,
{
    fn new(monoid: M) -> Self {
        Self { monoid }
    }

    fn create(&self, value: M::Element) -> Node<M> {
        Node {
            value,
            sum: self.monoid.id(),
            size: 1,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn get_sum(&self, this: *mut Node<M>) -> M::Element {
        if this.is_null() {
            self.monoid.id()
        } else {
            unsafe { (*this).sum.clone() }
        }
    }

    fn set_value(&self, this: *mut Node<M>, value: M::Element) {
        assert!(!this.is_null());
        unsafe {
            (*this).value = value;
        }
    }

    fn update_value(&self, this: *mut Node<M>, value: M::Element) {
        assert!(!this.is_null());
        let this = unsafe { &mut *this };
        this.value = self.monoid.op(this.value.clone(), value);
    }

    fn rotate(&self, this: *mut Node<M>) {
        let p = self.par_of(this).unwrap();
        let pp = self.par_of(p).unwrap();

        if self.left_of(p).unwrap() == this {
            let c = self.right_of(this).unwrap();
            self.set_left(p, c);
            self.set_right(this, p);
        } else {
            let c = self.left_of(this).unwrap();
            self.set_right(p, c);
            self.set_left(this, p);
        }

        if !pp.is_null() {
            let pp = unsafe { &mut *pp };
            if pp.lc == p {
                pp.lc = this;
            }
            if pp.rc == p {
                pp.rc = this;
            }
        }

        assert!(!this.is_null());
        let this = unsafe { &mut *this };
        this.par = pp;

        self.update(p);
        self.update(this);
    }

    fn status(&self, this: *mut Node<M>) -> i32 {
        let par = self.par_of(this).unwrap();
        if par.is_null() {
            return 0;
        }
        let par = unsafe { &*par };
        if par.lc == this {
            return 1;
        }
        if par.rc == this {
            return -1;
        }

        unreachable!()
    }

    fn pushdown(&self, this: *mut Node<M>) {
        if !this.is_null() {
            self.update(this);
        }
    }

    fn update(&self, this: *mut Node<M>) {
        assert!(!this.is_null());
        let this = unsafe { &mut *this };

        this.size = 1 + self.size_of(this.lc) + self.size_of(this.rc);

        this.sum = this.value.clone();
        if !this.lc.is_null() {
            this.sum = self.monoid.op(this.sum.clone(), self.get_sum(this.lc));
        }
        if !this.rc.is_null() {
            this.sum = self.monoid.op(this.sum.clone(), self.get_sum(this.rc));
        }
    }

    fn splay(&self, this: *mut Node<M>) {
        while self.status(this) != 0 {
            let par = self.par_of(this).unwrap();

            if self.status(par) == 0 {
                self.rotate(this);
            } else if self.status(this) == self.status(par) {
                self.rotate(par);
                self.rotate(this);
            } else {
                self.rotate(this);
                self.rotate(this);
            }
        }
    }

    fn get_first(&self, root: *mut Node<M>) -> *mut Node<M> {
        if root.is_null() {
            return root;
        }

        let mut cur = root;

        loop {
            self.pushdown(cur);

            let left = self.left_of(cur).unwrap();

            if left.is_null() {
                self.splay(cur);
                return cur;
            }
            cur = left;
        }
    }

    fn get_last(&self, root: *mut Node<M>) -> *mut Node<M> {
        if root.is_null() {
            return root;
        }

        let mut cur = root;

        loop {
            self.pushdown(cur);

            let right = self.right_of(cur).unwrap();

            if right.is_null() {
                self.splay(cur);
                return cur;
            }
            cur = right;
        }
    }

    fn merge(&self, left: *mut Node<M>, right: *mut Node<M>) -> *mut Node<M> {
        if left.is_null() {
            return right;
        }
        if right.is_null() {
            return left;
        }

        let cur = self.get_last(left);

        self.set_right(cur, right);
        self.update(right);
        self.update(cur);

        cur
    }

    fn split_left(&self, root: *mut Node<M>) -> (*mut Node<M>, *mut Node<M>) {
        if root.is_null() {
            return (ptr::null_mut(), ptr::null_mut());
        }

        let cur = root;
        let left = self.left_of(cur).unwrap();

        if !left.is_null() {
            self.set_par(left, ptr::null_mut());
            self.update(left);
        }
        self.set_left(cur, ptr::null_mut());
        self.update(cur);

        (left, cur)
    }

    fn split_right(&self, root: *mut Node<M>) -> (*mut Node<M>, *mut Node<M>) {
        if root.is_null() {
            return (ptr::null_mut(), ptr::null_mut());
        }

        let cur = root;
        let right = self.right_of(cur).unwrap();

        if !right.is_null() {
            self.set_par(right, ptr::null_mut());
            self.update(right);
        }
        self.set_right(cur, ptr::null_mut());
        self.update(cur);

        (cur, right)
    }

    fn set_left(&self, this: *mut Node<M>, left: *mut Node<M>) {
        assert!(!this.is_null());
        unsafe { (*this).lc = left };
        if !left.is_null() {
            unsafe { (*left).par = this };
        }
    }

    fn set_right(&self, this: *mut Node<M>, right: *mut Node<M>) {
        assert!(!this.is_null());
        unsafe { (*this).rc = right };
        if !right.is_null() {
            unsafe { (*right).par = this };
        }
    }

    fn set_par(&self, this: *mut Node<M>, par: *mut Node<M>) {
        assert!(!this.is_null());
        unsafe { (*this).par = par };
    }

    fn size_of(&self, this: *mut Node<M>) -> usize {
        if this.is_null() {
            0
        } else {
            unsafe { (*this).size }
        }
    }

    fn left_of(&self, this: *mut Node<M>) -> Option<*mut Node<M>> {
        (!this.is_null()).then(|| unsafe { (*this).lc })
    }

    fn right_of(&self, this: *mut Node<M>) -> Option<*mut Node<M>> {
        (!this.is_null()).then(|| unsafe { (*this).rc })
    }

    fn par_of(&self, this: *mut Node<M>) -> Option<*mut Node<M>> {
        (!this.is_null()).then(|| unsafe { (*this).par })
    }
}

/// Euler tour tree
pub struct EulerTourTree<M: Monoid> {
    man: Manager<M>,
    vertices: Vec<*mut Node<M>>,
    edges: Vec<HashMap<usize, *mut Node<M>>>,
}

impl<M: Monoid + Clone> EulerTourTree<M>
where
    M::Element: Clone,
{
    /// `n`個の頂点のみからなる森を構築する。
    pub fn new(monoid: M, n: usize) -> Self {
        let man = Manager::new(monoid);
        let vertices = std::iter::repeat_with(|| {
            let p = Box::new(man.create(man.monoid.id()));
            Box::into_raw(p)
        })
        .take(n)
        .collect::<Vec<_>>();

        let edges = (0..n).map(|i| HashMap::from([(i, vertices[i])])).collect();

        Self {
            man,
            vertices,
            edges,
        }
    }

    /// 頂点`r`をそれの属する木の根にする。
    pub fn reroot(&mut self, r: usize) {
        let p = self.vertices[r];

        self.man.splay(p);
        let (l, r) = self.man.split_left(p);
        self.man.merge(r, l);
    }

    /// 2つの頂点が同一の木に属するかどうかを判定する。
    pub fn is_same_tree(&self, i: usize, j: usize) -> bool {
        if i == j {
            return true;
        }

        let pi = self.vertices[i];
        let pj = self.vertices[j];

        self.man.splay(pi);
        let ri = self.man.get_first(pi);

        self.man.splay(pj);
        let rj = self.man.get_first(pj);

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

        self.man.splay(pi);
        self.man.splay(pj);

        let eij = Box::into_raw(Box::new(self.man.create(self.man.monoid.id())));
        self.edges[i].insert(j, eij);

        let eji = Box::into_raw(Box::new(self.man.create(self.man.monoid.id())));
        self.edges[j].insert(i, eji);

        let t = self.man.merge(pi, eij);
        let t = self.man.merge(t, pj);
        self.man.merge(t, eji);

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

                self.man.splay(eij);
                let (s, a) = self.man.split_left(eij);
                self.man.split_right(a);

                self.man.splay(eji);
                let (_, a) = self.man.split_left(eji);
                let (_, u) = self.man.split_right(a);

                self.man.merge(s, u);

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
    pub fn set(&mut self, i: usize, value: M::Element) {
        let p = self.vertices[i];
        self.man.splay(p);
        self.man.set_value(p, value);
        self.man.pushdown(p);
    }

    /// 頂点`i`の値をモノイドの演算と値`value`で更新する。
    pub fn update(&mut self, i: usize, value: M::Element) {
        let p = self.vertices[i];
        self.man.splay(p);
        self.man.update_value(p, value);
        self.man.pushdown(p);
    }

    /// 頂点`p`を親とする頂点`v`について、`v`を根とする部分木の値を集積して返す。
    pub fn subtree_sum(&mut self, v: usize, p: usize) -> Result<M::Element, &'static str> {
        self.cut(v, p)?;

        let rv = self.vertices[v];
        self.man.splay(rv);
        let ret = self.man.get_sum(rv);

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
        let mut ett = EulerTourTree::new(Trivial, 10);

        ett.link(1, 2).unwrap();
        ett.link(3, 5).unwrap();
        ett.link(1, 5).unwrap();
        ett.reroot(2);

        ett.cut(1, 2).unwrap();
    }
}
