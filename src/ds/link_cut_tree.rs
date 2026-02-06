//! Link-Cut Tree
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/dynamic_tree_vertex_add_path_sum>
//! - <https://judge.yosupo.jp/problem/dynamic_tree_vertex_set_path_composite> (非可換なモノイド)

use std::{mem::swap, ptr};

use crate::algebra::traits::*;

struct Manager<M: Monoid> {
    monoid: M,
    nodes: Vec<Node<M>>,
}

struct Node<M: Monoid> {
    value: M::Element,
    acc: M::Element,
    rev_acc: Option<M::Element>,
    size: usize,
    rev: bool,
    lc: *mut Self,
    rc: *mut Self,
    par: *mut Self,
}

impl<M: Monoid + Commutative> Manager<M> {
    fn create_commutative(&mut self) {
        self.nodes.push(Node {
            value: self.monoid.id(),
            acc: self.monoid.id(),
            rev_acc: None,
            size: 1,
            rev: false,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        })
    }
}

impl<M: Monoid> Manager<M> {
    fn new(monoid: M) -> Self {
        Self {
            monoid,
            nodes: vec![],
        }
    }

    fn create(&mut self) {
        self.nodes.push(Node {
            value: self.monoid.id(),
            acc: self.monoid.id(),
            rev_acc: Some(self.monoid.id()),
            size: 1,
            rev: false,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        })
    }

    fn ptr(&self, k: usize) -> *mut Node<M> {
        &self.nodes[k] as *const _ as *mut _
    }
}

impl<M: Monoid> Manager<M>
where
    M::Element: Clone,
{
    fn is_root(&self, this: *mut Node<M>) -> bool {
        assert!(!this.is_null());
        let par = unsafe { (*this).par };
        par.is_null() || unsafe { (*par).lc != this && (*par).rc != this }
    }

    fn update(&self, this: *mut Node<M>) {
        assert!(!this.is_null());
        let this = unsafe { &mut *this };
        let monoid = &self.monoid;
        this.size = 1;
        this.acc = this.value.clone();
        if let Some(rev_acc) = this.rev_acc.as_mut() {
            *rev_acc = this.value.clone();
        }

        let left = this.lc;
        if !left.is_null() {
            let left = unsafe { &mut *left };
            self.pushdown(left);
            this.size += left.size;
            this.acc = monoid.op(left.acc.clone(), this.acc.clone());

            if let Some(rev_acc) = this.rev_acc.as_mut() {
                *rev_acc = monoid.op(rev_acc.clone(), left.rev_acc.clone().unwrap());
            }
        }

        let right = this.rc;
        if !right.is_null() {
            let right = unsafe { &mut *right };
            self.pushdown(right);
            this.size += right.size;
            this.acc = monoid.op(this.acc.clone(), right.acc.clone());

            if let Some(rev_acc) = this.rev_acc.as_mut() {
                *rev_acc = monoid.op(right.rev_acc.clone().unwrap(), rev_acc.clone());
            }
        }
    }

    fn reverse(&self, this: *mut Node<M>) {
        if !this.is_null() {
            unsafe { (*this).rev ^= true };
        }
    }

    fn pushdown(&self, this: *mut Node<M>) {
        assert!(!this.is_null());
        let this = unsafe { &mut *this };
        if this.rev {
            swap(&mut this.lc, &mut this.rc);
            self.reverse(this.lc);
            self.reverse(this.rc);
            this.rev = false;

            if let Some(rev_acc) = this.rev_acc.as_mut() {
                swap(&mut this.acc, rev_acc);
            }
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

    fn rot(&self, this: *mut Node<M>, dir_right: bool) {
        let p = self.par_of(this).unwrap();
        let g = self.par_of(p).unwrap();

        if dir_right {
            let c = self.right_of(this).unwrap();
            self.set_left(p, c);
            self.set_right(this, p);
        } else {
            let c = self.left_of(this).unwrap();
            self.set_right(p, c);
            self.set_left(this, p);
        }

        self.update(p);
        self.update(this);

        self.set_par(this, g);
        if !g.is_null() {
            let g = unsafe { &mut *g };
            if g.lc == p {
                g.lc = this;
            }
            if g.rc == p {
                g.rc = this;
            }
            self.update(g);
        }
    }

    fn splay(&self, this: *mut Node<M>) {
        assert!(!this.is_null());
        while !self.is_root(this) {
            let p = self.par_of(this).unwrap();
            let gp = self.par_of(p).unwrap();

            if self.is_root(p) {
                self.pushdown(p);
                self.pushdown(this);
                self.rot(this, this == self.left_of(p).unwrap());
            } else {
                self.pushdown(gp);
                self.pushdown(p);
                self.pushdown(this);
                let flag = this == self.left_of(p).unwrap();

                if (this == self.left_of(p).unwrap()) == (p == self.left_of(gp).unwrap()) {
                    self.rot(p, flag);
                    self.rot(this, flag);
                } else {
                    self.rot(this, flag);
                    self.rot(this, !flag);
                }
            }
        }
        self.pushdown(this);
    }

    fn expose(&self, u: *mut Node<M>) {
        let mut rp = ptr::null_mut();
        let mut p = u;

        while !p.is_null() {
            self.splay(p);
            unsafe { (*p).rc = rp };
            self.update(p);
            rp = p;
            p = self.par_of(p).unwrap();
        }

        self.splay(u);
        assert!(self.right_of(u).unwrap().is_null());
    }

    fn root_of(&self, mut this: *mut Node<M>) -> *mut Node<M> {
        assert!(!this.is_null());
        loop {
            let p = self.par_of(this).unwrap();
            if p.is_null() {
                return this;
            }
            this = p;
        }
    }

    fn same_group(&self, u: *mut Node<M>, v: *mut Node<M>) -> bool {
        self.root_of(u) == self.root_of(v)
    }

    fn evert(&self, u: *mut Node<M>) {
        self.expose(u);
        self.reverse(u);
        self.pushdown(u);
    }
}

/// Link-Cut Tree
///
/// 動的に木の辺を追加・削除可能
pub struct LinkCutTree<M: Monoid> {
    man: Manager<M>,
}

impl<M: Monoid + Commutative + Clone> LinkCutTree<M> {
    /// 可換モノイドを持てる`LinkCutTree<M>`を生成する。
    pub fn new_commutative(monoid: M, n: usize) -> Self {
        let mut man = Manager::new(monoid);
        for _ in 0..n {
            man.create_commutative();
        }
        Self { man }
    }
}

impl<M: Monoid + Clone> LinkCutTree<M> {
    /// 非可換なモノイドを持てる`LinkCutTree<M>`を生成する。
    pub fn new_non_commutative(monoid: M, n: usize) -> Self {
        let mut man = Manager::new(monoid);
        for _ in 0..n {
            man.create();
        }
        Self { man }
    }
}

impl<M: Monoid + Clone> LinkCutTree<M>
where
    M::Element: Clone,
{
    #[allow(missing_docs)]
    pub fn expose(&mut self, k: usize) {
        let k = self.man.ptr(k);
        self.man.expose(k);
    }

    /// 頂点`i`と頂点`j`の間にある辺を削除する。
    ///
    /// # Panics
    ///
    /// 頂点`i`と頂点`j`が隣接していないときパニックする。
    pub fn cut(&mut self, i: usize, j: usize) {
        assert_ne!(i, j);

        let u = self.man.ptr(i);
        let v = self.man.ptr(j);

        self.man.evert(u);
        self.man.expose(v);

        assert!(self.man.left_of(u).unwrap().is_null());
        assert!(self.man.is_root(v));

        assert!(
            self.man.left_of(v).unwrap() == u && self.man.par_of(u).unwrap() == v,
            "`cut`操作では頂点`{i}`と`{j}`は隣接して連結されねばならない。",
        );

        self.man.set_par(u, ptr::null_mut());
        self.man.set_left(v, ptr::null_mut());
        self.man.update(v);
    }

    /// 頂点`i`と頂点`j`の間に辺を張る。
    ///
    /// # Panics
    ///
    /// 頂点`i`と頂点`j`が同一の木に属するときパニックする。
    pub fn link(&mut self, i: usize, j: usize) {
        assert_ne!(i, j);
        let u = self.man.ptr(i);
        let v = self.man.ptr(j);

        self.man.expose(u);
        self.man.evert(v);
        let v = self.man.root_of(v);

        assert!(
            !self.man.same_group(u, v),
            "`link`操作では頂点`{i}`と`{j}`は同一の木に属してはならない。",
        );

        assert!(self.man.is_root(u));
        self.man.set_right(u, v);
        self.man.update(u);
    }

    #[allow(missing_docs)]
    pub fn evert(&mut self, k: usize) {
        let k = self.man.ptr(k);
        self.man.evert(k);
    }

    /// 頂点`k`の値を`x`に変更する。
    pub fn set(&mut self, k: usize, x: M::Element) {
        let u = self.man.ptr(k);
        self.man.expose(u);
        self.man.nodes[k].value = x;
        self.man.update(u);
    }

    /// 頂点`k`の値をモノイドの演算と値`x`で更新する。
    pub fn update(&mut self, k: usize, x: M::Element) {
        self.set(k, self.man.monoid.op(self.get(k), x));
    }

    /// 頂点`k`の値を返す。
    pub fn get(&self, k: usize) -> M::Element {
        self.man.nodes[k].value.clone()
    }

    /// 頂点`i`,`j`間のパス上でのモノイドの演算の結果を返す。
    ///
    /// # Panics
    ///
    /// 頂点`i`と`j`が同一の木に属していないときパニックする。
    pub fn fold(&self, i: usize, j: usize) -> M::Element {
        let u = self.man.ptr(i);
        let v = self.man.ptr(j);

        self.man.evert(u);
        self.man.expose(v);

        assert!(
            self.man.same_group(u, v),
            "頂点`{i}`と`{j}`は同一の木に属していなければならない。",
        );

        self.man.nodes[j].acc.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algebra::trivial::*;

    #[test]
    fn test() {
        let n = 10;

        let mut lct = LinkCutTree::new_commutative(Trivial, n);

        //        lct.cut(0, 1); // Runtime error

        lct.link(0, 1);
        lct.link(1, 2);

        //        lct.link(0, 2); // Runtime error
    }
}
