//! Link-Cut Tree
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/dynamic_tree_vertex_add_path_sum>

use std::{mem::swap, ptr};

use crate::algebra::traits::Monoid;

struct Node<M: Monoid> {
    value: M::Element,
    result: M::Element,
    monoid: M,
    subsize: usize,
    _index: usize,
    rev: bool,
    lc: *mut Node<M>,
    rc: *mut Node<M>,
    par: *mut Node<M>,
}

impl<M: Monoid + Copy> Node<M>
where
    M::Element: Clone,
{
    fn new(monoid: M, _index: usize) -> Self {
        Self {
            value: monoid.id(),
            result: monoid.id(),
            monoid,
            subsize: 1,
            _index,
            rev: false,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn is_root(this: *mut Self) -> bool {
        assert!(!this.is_null());
        unsafe {
            let par = (*this).par;
            par.is_null() || ((*par).lc != this && (*par).rc != this)
        }
    }

    fn update_node_status(this: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).subsize = 1;
            (*this).result = (*this).value.clone();

            let left = (*this).lc;
            if !left.is_null() {
                Self::pushdown(left);
                (*this).subsize += (*left).subsize;
                (*this).result = (*this)
                    .monoid
                    .op((*this).result.clone(), (*left).result.clone());
            }

            let right = (*this).rc;
            if !right.is_null() {
                Self::pushdown(right);
                (*this).subsize += (*right).subsize;
                (*this).result = (*this)
                    .monoid
                    .op((*this).result.clone(), (*right).result.clone());
            }
        }
    }

    fn reverse(this: *mut Self) {
        if !this.is_null() {
            unsafe { (*this).rev ^= true };
        }
    }

    fn pushdown(this: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            if (*this).rev {
                swap(&mut (*this).lc, &mut (*this).rc);
                Self::reverse((*this).lc);
                Self::reverse((*this).rc);
                (*this).rev = false;
            }
        }
    }

    fn left_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).lc })
    }

    fn right_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).rc })
    }

    fn par_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).par })
    }

    fn set_left(this: *mut Self, left: *mut Self) {
        assert!(!this.is_null());
        unsafe { (*this).lc = left };
        if !left.is_null() {
            unsafe { (*left).par = this };
        }
    }

    fn set_right(this: *mut Self, right: *mut Self) {
        assert!(!this.is_null());
        unsafe { (*this).rc = right };
        if !right.is_null() {
            unsafe { (*right).par = this };
        }
    }

    fn set_par(this: *mut Self, par: *mut Self) {
        assert!(!this.is_null());
        unsafe { (*this).par = par };
    }

    fn rot(this: *mut Self, dir_right: bool) {
        let p = Self::par_of(this).unwrap();
        let g = Self::par_of(p).unwrap();

        if dir_right {
            let c = Self::right_of(this).unwrap();
            Self::set_left(p, c);
            Self::set_right(this, p);
        } else {
            let c = Self::left_of(this).unwrap();
            Self::set_right(p, c);
            Self::set_left(this, p);
        }

        Self::update_node_status(p);
        Self::update_node_status(this);

        Self::set_par(this, g);
        if !g.is_null() {
            unsafe {
                if (*g).lc == p {
                    (*g).lc = this;
                }
                if (*g).rc == p {
                    (*g).rc = this;
                }
            }
            Self::update_node_status(g);
        }
    }

    fn splay(this: *mut Self) {
        assert!(!this.is_null());
        while !Self::is_root(this) {
            let p = Self::par_of(this).unwrap();
            let gp = Self::par_of(p).unwrap();

            if Self::is_root(p) {
                Self::pushdown(p);
                Self::pushdown(this);
                Self::rot(this, this == Self::left_of(p).unwrap());
            } else {
                Self::pushdown(gp);
                Self::pushdown(p);
                Self::pushdown(this);
                let flag = this == Self::left_of(p).unwrap();

                if (this == Self::left_of(p).unwrap()) == (p == Self::left_of(gp).unwrap()) {
                    Self::rot(p, flag);
                    Self::rot(this, flag);
                } else {
                    Self::rot(this, flag);
                    Self::rot(this, !flag);
                }
            }
        }
        Self::pushdown(this);
    }

    fn expose(u: *mut Self) {
        let mut rp = ptr::null_mut();
        let mut p = u;

        while !p.is_null() {
            Self::splay(p);
            unsafe { (*p).rc = rp };
            Self::update_node_status(p);
            rp = p;
            p = Self::par_of(p).unwrap();
        }

        Self::splay(u);
    }

    fn root_of(mut this: *mut Self) -> *mut Self {
        assert!(!this.is_null());
        loop {
            let p = Self::par_of(this).unwrap();
            if p.is_null() {
                return this;
            }
            this = p;
        }
    }

    fn same_group(u: *mut Self, v: *mut Self) -> bool {
        Self::root_of(u) == Self::root_of(v)
    }

    fn is_conencted(u: *mut Self, v: *mut Self) -> bool {
        assert!(!u.is_null());
        assert!(!v.is_null());

        Self::par_of(u).unwrap() == v || Self::par_of(v).unwrap() == u
    }

    fn evert(u: *mut Self) {
        Self::expose(u);
        Self::reverse(u);
        Self::pushdown(u);
    }
}

/// Link-Cut Tree
///
/// 動的に木の辺を追加・削除可能
pub struct LinkCutTree<M: Monoid> {
    monoid: M,
    nodes: Vec<Node<M>>,
}

impl<M: Monoid + Copy> LinkCutTree<M>
where
    M::Element: Clone,
{
    /// `LinkCutTree<M>`を生成する。
    pub fn new(monoid: M, n: usize) -> Self {
        Self {
            monoid,
            nodes: (0..n).map(|i| Node::new(monoid, i)).collect(),
        }
    }

    #[allow(missing_docs)]
    pub fn expose(&mut self, k: usize) {
        Node::expose(&mut self.nodes[k]);
    }

    /// 頂点`i`と頂点`j`の間にある辺を削除する。
    ///
    /// # Panics
    ///
    /// 頂点`i`と頂点`j`が隣接していないときパニックする。
    pub fn cut(&mut self, i: usize, j: usize) {
        let u = &mut self.nodes[i] as *mut _;
        let v = &mut self.nodes[j] as *mut _;

        Node::expose(u);
        Node::expose(v);

        assert!(
            Node::is_conencted(u, v),
            "`cut`操作では頂点`{i}`と`{j}`は隣接して連結されねばならない。",
        );

        if Node::is_root(u) {
            Node::set_par(u, ptr::null_mut());
        } else {
            Node::set_par(Node::left_of(v).unwrap(), ptr::null_mut());
            Node::set_left(v, ptr::null_mut());
            Node::update_node_status(v);
        }
    }

    /// 頂点`i`と頂点`j`の間に辺を張る。
    ///
    /// # Panics
    ///
    /// 頂点`i`と頂点`j`が同一の木に属するときパニックする。
    pub fn link(&mut self, i: usize, j: usize) {
        assert_ne!(i, j);
        let u = &mut self.nodes[i] as *mut _;
        let v = &mut self.nodes[j] as *mut _;

        Node::evert(u);

        assert!(
            !Node::same_group(u, v),
            "`link`操作では頂点`{i}`と`{j}`は同一の木に属してはならない。",
        );

        Node::set_par(u, v);
    }

    #[allow(missing_docs)]
    pub fn evert(&mut self, k: usize) {
        Node::evert(&mut self.nodes[k]);
    }

    /// 頂点`k`の値を`x`に変更する。
    pub fn set(&mut self, k: usize, x: M::Element) {
        Node::evert(&mut self.nodes[k]);
        self.nodes[k].value = x;
        Node::pushdown(&mut self.nodes[k]);
    }

    /// 頂点`k`の値をモノイドの演算と値`x`で更新する。
    pub fn update(&mut self, k: usize, x: M::Element) {
        self.set(k, self.monoid.op(self.get(k), x));
    }

    /// 頂点`k`の値を返す。
    pub fn get(&self, k: usize) -> M::Element {
        self.nodes[k].value.clone()
    }

    /// 頂点`i`,`j`間のパス上でのモノイドの演算の結果を返す。
    ///
    /// # Panics
    ///
    /// 頂点`i`と`j`が同一の木に属していないときパニックする。
    pub fn fold(&self, i: usize, j: usize) -> M::Element {
        let u = &self.nodes[i] as *const _ as *mut Node<M>;
        let v = &self.nodes[j] as *const _ as *mut Node<M>;

        assert!(
            Node::same_group(u, v),
            "頂点`{i}`と`{j}`は同一の木に属していなければならない。",
        );

        Node::evert(u);
        Node::expose(v);
        self.nodes[j].result.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algebra::trivial::*;

    #[test]
    fn test() {
        let monoid = Trivial;
        let n = 10;

        let mut lct = LinkCutTree::new(monoid, n);

        //        lct.cut(0, 1); // Runtime error

        lct.link(0, 1);
        lct.link(1, 2);

        //        lct.link(0, 2); // Runtime error
    }
}
