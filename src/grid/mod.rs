//! グリッド
pub mod to_graph;

/// グリッド上の位置を表す。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
    h: usize,
    w: usize,
}

impl Position {
    /// `(0, 0) ~ (h - 1, w - 1)`の長方形内の位置`(x, y)`を表す`Position`を返す。
    pub fn new(x: usize, y: usize, h: usize, w: usize) -> Self {
        Self { x, y, h, w }
    }

    /// x方向の位置
    pub fn x(self) -> usize {
        self.x
    }

    /// y方向の位置
    pub fn y(self) -> usize {
        self.y
    }

    /// `(self.x + d.dx, self.y + d.dy)`に相当する`Position`を`Some`に包んで返す。
    ///
    /// 移動先が`(0, 0)`から`(h - 1, w - 1)`の外部にある場合、`None`を返す。
    pub fn mov(self, d: Dir) -> Option<Self> {
        let x = self.x.checked_add_signed(d.dx)?;
        let y = self.y.checked_add_signed(d.dy)?;

        (x < self.h && y < self.w).then_some(Self { x, y, ..self })
    }
}

/// 移動方向
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dir {
    /// x方向の移動量
    pub dx: isize,
    /// y方向の移動量
    pub dy: isize,
}

impl Dir {
    /// 移動量`(dx, dy)`の`Dir`を返す。
    pub fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }

    /// 左方向への1マス分の移動
    pub const L: Self = Self { dx: 0, dy: -1 };
    /// 右方向への1マス分の移動
    pub const R: Self = Self { dx: 0, dy: 1 };
    /// 上方向への1マス分の移動
    pub const U: Self = Self { dx: -1, dy: 0 };
    /// 下方向への1マス分の移動
    pub const D: Self = Self { dx: 1, dy: 0 };
    /// 上下左右4方向への1マス分の移動を格納した配列
    pub const DIR_4: [Self; 4] = [Self::L, Self::R, Self::U, Self::D];
}

impl std::ops::Add for Dir {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.dx + other.dx, self.dy + other.dy)
    }
}

/// 長方形型のマス目グリッドを扱う。
#[derive(Clone, Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    h: usize,
    w: usize,
}

impl<T> Grid<T> {
    /// [`Vec<Vec<T>>`]などから[`Grid`]を構築する。
    pub fn new(g: impl IntoIterator<Item = impl Into<Vec<T>>>) -> Option<Self> {
        let grid: Vec<_> = g.into_iter().map(|a| a.into()).collect();

        let w = grid.first().map_or(0, |a| a.len());

        for a in &grid {
            if a.len() != w {
                return None;
            }
        }
        let h = grid.len();

        Some(Self { grid, h, w })
    }

    /// 位置`p`の要素への参照を返す。
    pub fn get(&self, p: Position) -> &T {
        &self.grid[p.x][p.y]
    }

    /// 位置`p`の要素への可変参照を返す。
    pub fn get_mut(&mut self, p: Position) -> &mut T {
        &mut self.grid[p.x][p.y]
    }

    /// グリッド上の位置と要素の参照のタプルへのイテレータを返す。
    pub fn iter(&self) -> impl Iterator<Item = (Position, &T)> {
        let (h, w) = (self.h, self.w);
        self.grid.iter().enumerate().flat_map(move |(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(j, x)| (Position::new(i, j, h, w), x))
        })
    }

    /// グリッド上の位置と要素の可変参照のタプルへのイテレータを返す。
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        let (h, w) = (self.h, self.w);
        self.grid.iter_mut().enumerate().flat_map(move |(i, v)| {
            v.iter_mut()
                .enumerate()
                .map(move |(j, x)| (Position::new(i, j, h, w), x))
        })
    }
}

impl<T> From<Grid<T>> for Vec<Vec<T>> {
    fn from(value: Grid<T>) -> Self {
        value.grid
    }
}

impl<T> AsRef<[Vec<T>]> for Grid<T> {
    fn as_ref(&self) -> &[Vec<T>] {
        &self.grid
    }
}
