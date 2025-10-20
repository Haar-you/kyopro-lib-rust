//! Stern-Brocot木
//!
//! # References
//! - <https://miscalc.hatenablog.com/entry/2023/12/22/213007>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/stern_brocot_tree>
//! - <https://judge.yosupo.jp/problem/rational_approximation>

use crate::math::continued_fraction::*;

/// 分数$\frac{a}{b}$を表す。
#[derive(Clone, Copy, Debug)]
pub struct Frac(pub u64, pub u64);

impl PartialEq for Frac {
    fn eq(&self, Self(c, d): &Self) -> bool {
        let Self(a, b) = self;
        a * d == b * c
    }
}
impl Eq for Frac {}
impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let Self(a, b) = self;
        let Self(c, d) = other;
        (a * d).partial_cmp(&(b * c))
    }
}

/// Stern-Brocot木上の頂点を表す。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SBNode(u64, u64, u64, u64);

impl SBNode {
    /// Stern-Brocot木の根を返す。
    pub fn root() -> Self {
        Self(0, 1, 1, 0)
    }

    /// ノードの内部の4つ組の数を返す。
    pub fn quadruplet(self) -> (u64, u64, u64, u64) {
        (self.0, self.1, self.2, self.3)
    }

    /// 左側の子を`n`回辿った先のノードを返す。
    pub fn left_child(self, n: u64) -> Self {
        let Self(p, q, r, s) = self;
        Self(p, q, p * n + r, q * n + s)
    }

    /// 右側の子を`n`回辿った先のノードを返す。
    pub fn right_child(self, n: u64) -> Self {
        let Self(p, q, r, s) = self;
        Self(p + r * n, q + s * n, r, s)
    }

    /// ノードの表す値を[`Frac`]型に変換する。
    pub fn as_frac(self) -> Frac {
        let Self(p, q, r, s) = self;
        Frac(p + r, q + s)
    }

    /// ノードの表す値を[`f64`]型に変換する。
    pub fn as_f64(self) -> f64 {
        let Self(p, q, r, s) = self;
        ((p + r) as f64) / ((q + s) as f64)
    }

    /// ノードの子孫ノードの表す値の範囲を返す。
    pub fn range(self) -> (Frac, Frac) {
        let Self(p, q, r, s) = self;
        (Frac(p, q), Frac(r, s))
    }

    /// [`Frac`]から[`SBNode`]に変換する。
    pub fn from_frac(a: Frac) -> Option<Self> {
        Some(SBPath::decode(SBPath::encode(a)?))
    }

    /// Stern-Brocot木上でのノード`a`とノード`b`のLCAを返す。
    pub fn lca(a: impl Into<Frac>, b: impl Into<Frac>) -> Option<Self> {
        let pa = SBPath::encode(a)?;
        let pb = SBPath::encode(b)?;

        let mut pc = vec![];
        for (a, b) in std::iter::zip(pa.0, pb.0) {
            if a == b {
                pc.push(a);
            } else {
                match (a, b) {
                    (SBMove::L(a), SBMove::L(b)) => pc.push(SBMove::L(a.min(b))),
                    (SBMove::R(a), SBMove::R(b)) => pc.push(SBMove::R(a.min(b))),
                    _ => {}
                }
                break;
            }
        }

        Some(SBPath::decode(SBPath(pc)))
    }

    /// Stern-Brocot木の根からノード`a`へのパスで、根から深さ`d`のノードを返す。
    pub fn ancestor(a: impl Into<Frac>, mut d: u64) -> Option<Self> {
        let path = SBPath::encode(a)?;

        let mut path_d = vec![];
        for m in path.0 {
            match m {
                SBMove::L(n) => {
                    path_d.push(SBMove::L(n.min(d)));
                    d = d.saturating_sub(n);
                }
                SBMove::R(n) => {
                    path_d.push(SBMove::R(n.min(d)));
                    d = d.saturating_sub(n);
                }
            }
            if d == 0 {
                break;
            }
        }
        if d > 0 {
            return None;
        }

        Some(SBPath::decode(SBPath(path_d)))
    }
}

impl From<SBNode> for Frac {
    fn from(value: SBNode) -> Self {
        value.as_frac()
    }
}

/// 子ノードへの移動を表す。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SBMove {
    /// 左の子を`n`回辿る操作を表す。
    L(u64),
    /// 右の子を`n`回辿る操作を表す。
    R(u64),
}
/// Stern-Brocot木の根からノードへのパスを表す。
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SBPath(pub Vec<SBMove>);

impl SBPath {
    /// [`Frac`]から[`SBPath`]への変換をする。
    ///
    /// `f`の表す分数が、$\frac{n}{0}$または$\frac{0}{n}$であるとき、`None`を返す。
    pub fn encode(f: impl Into<Frac>) -> Option<Self> {
        let Frac(p, q) = f.into();
        if p == 0 || q == 0 {
            return None;
        }

        let mut cfe = continued_fraction(p, q)?;
        *cfe.last_mut().unwrap() -= 1;

        let mut ret = vec![];
        if let Some(&m) = cfe.first() {
            if m != 0 {
                ret.push(SBMove::R(m));
            }
        }

        for (i, m) in cfe.into_iter().skip(1).enumerate() {
            if i % 2 == 0 {
                ret.push(SBMove::L(m));
            } else {
                ret.push(SBMove::R(m));
            }
        }

        Some(Self(ret))
    }

    /// [`SBPath`]から[`SBNode`]への変換をする。
    pub fn decode(self) -> SBNode {
        let mut ret = SBNode::root();
        for m in self.0 {
            match m {
                SBMove::L(n) => ret = ret.left_child(n),
                SBMove::R(n) => ret = ret.right_child(n),
            }
        }
        ret
    }
}
