//! データ構造
//!
//! # セグメント木系の比較
//!
//! | データ構造 | 新規作成 | 区間更新 | 点更新 | 区間取得 | 点取得 |
//! | ---- | ---- | ---- | ---- | ---- | ---- |
//! | [`Segtree`](segtree::Segtree) | `new(n, M)` | | `assign(i, X)`, `update(i, X)` | `fold(l..r)` | |
//! | [`DualSegtree`](dual_segtree::DualSegtree) | `new(n, M)` | `update(l..r, X)` | | | `get(i)` |
//! | [`LazySegtree`](lazy_segtree::LazySegtree) | `new(n, A)` | `update(l..r, X)` | | `fold(l..r)` | |
//! | [`DynamicSegtree`](dynamic_segtree::DynamicSegtree) | `new(M)` | | `assign(i, X)` | `fold(l..r)` | |
//! | [`DynamicDualSegtree`](dynamic_dual_segtree::DynamicDualSegtree) | `new(M)` | `update(l..r, X)` | | | `get(i)` |
//! | [`DynamicLazySegtree`](dynamic_lazy_segtree::DynamicLazySegtree) | `new(A)` | `update(l..r, X)` | | `fold(l..r)` | |
//! | [`PersistentSegtree`](persistent_segtree::PersistentSegtree) | `new(n, M)` | | `assign(i, X)` | `fold(l..r)` | |
//! | [`SparseTable`](sparse_table::SparseTable) | `new(s, A)` | | | `fold(l..r)` | |
//! | [`DisjointSparseTable`](disjoint_sparse_table::DisjointSparseTable) | `new(s, S)` | | | `fold(l..r)` | |
//! | [`FenwickTree`](fenwick::FenwickTree) | `new(n, G)` | | `update(i, X)` | `fold_to(..r)`, `fold(l..r)` | |
//! | [`SegtreeBeats`](segtree_beats::SegtreeBeats) | `new(n)` | `chmin(l..r, X)`, `chmax(l..r, X)`, `add(l..r, X)` | | `sum(l..r, X)` | |
//! | [`StarrySkyTree`](starry_sky_tree::StarrySkyTree) | `new(n)` | `update(l..r, X)` | | `fold(l..r)` | |

// pub mod traits;

pub mod partially_persistent_unionfind;
pub mod persistent_unionfind;
pub mod potential_unionfind;
pub mod rollbackable_unionfind;
pub mod unionfind;

pub mod dual_segtree;
pub mod dynamic_dual_segtree;
pub mod dynamic_lazy_segtree;
pub mod dynamic_segtree;
pub mod fenwick;
pub mod fenwick_add;
pub mod lazy_segtree;
pub mod lazy_segtree_coeff;
pub mod persistent_segtree;
pub mod segtree;
pub mod segtree_2d;
pub mod segtree_beats;
pub mod segtree_bidir;
pub mod segtree_linear_add;
pub mod segtree_linear_add_range_sum;
pub mod starry_sky_tree;
pub mod starry_sky_tree_count;

pub mod fenwick_on_fenwick;
pub mod segtree_on_segtree;

pub mod cumulative_sum_1d;
pub mod cumulative_sum_2d;

pub mod persistent_array;
pub mod rollbackable_vector;

pub mod range_search_tree;

pub mod foldable_deque;
pub mod persistent_queue;

pub mod disjoint_sparse_table;
pub mod sparse_table;
pub mod sparse_table_2d;

pub mod interval_heap;
pub mod lazy_skew_heap;
pub mod skew_heap;

pub mod persistent_stack;

pub mod cht;
pub mod li_chao;

pub mod binary_trie;

pub mod succinct_bitvec;
pub mod wavelet_matrix;

pub mod multiset;

pub mod qword_tree;

pub mod aho_corasick;
pub mod palindromic_tree;
pub mod trie;

pub mod bitset;

pub mod merge_sort_tree;

pub mod linked_list;

pub mod lazy_splay_tree;
pub mod link_cut_tree;
pub mod splay_tree;

pub mod integer_set;

pub mod usize_set;

pub mod ordered_map;
pub mod ordered_set;

pub mod euler_tour_tree;
