pub mod to_graph;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// 移動方向
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dir {
    /// x方向の移動量
    pub dx: isize,
    /// y方向の移動量
    pub dy: isize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn mov_strict(self, d: Dir, h: usize, w: usize) -> Option<Self> {
        let x = self.x.checked_add_signed(d.dx)?;
        let y = self.y.checked_add_signed(d.dy)?;

        (x < h && y < w).then_some(Self::new(x, y))
    }
}

impl Dir {
    pub fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }

    pub const L: Dir = Dir { dx: 0, dy: -1 };
    pub const R: Dir = Dir { dx: 0, dy: 1 };
    pub const U: Dir = Dir { dx: -1, dy: 0 };
    pub const D: Dir = Dir { dx: 1, dy: 0 };
    pub const DIR_4: [Dir; 4] = [Self::L, Self::R, Self::U, Self::D];
}

impl std::ops::Add for Dir {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.dx + other.dx, self.dy + other.dy)
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(g: impl IntoIterator<Item = impl Into<Vec<T>>>) -> Self {
        Self {
            grid: g.into_iter().map(|a| a.into()).collect(),
        }
    }

    pub fn get(&self, p: Position) -> &T {
        &self.grid[p.x][p.y]
    }

    pub fn get_mut(&mut self, p: Position) -> &mut T {
        &mut self.grid[p.x][p.y]
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, &T)> {
        self.grid.iter().enumerate().flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(j, x)| (Position::new(i, j), x))
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        self.grid.iter_mut().enumerate().flat_map(|(i, v)| {
            v.iter_mut()
                .enumerate()
                .map(move |(j, x)| (Position::new(i, j), x))
        })
    }
}
