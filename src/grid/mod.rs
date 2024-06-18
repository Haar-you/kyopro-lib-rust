pub mod to_graph;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dir {
    pub dx: isize,
    pub dy: isize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn mov_strict(self, d: Dir, h: usize, w: usize) -> Option<Self> {
        let x = self.x.checked_add_signed(d.dx)?;
        let y = self.y.checked_add_signed(d.dy)?;

        if x >= h || y >= w {
            None
        } else {
            Some(Self::new(x, y))
        }
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
        &self.grid[p.x as usize][p.y as usize]
    }

    pub fn find_all<P: FnMut(&T) -> bool>(&self, mut p: P) -> Vec<Position> {
        let mut ret = vec![];
        self.for_each(|a, c| {
            if p(c) {
                ret.push(a);
            }
        });
        ret
    }

    pub fn for_each<F: FnMut(Position, &T)>(&self, mut f: F) {
        for (i, v) in self.grid.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                f(Position::new(i, j), c);
            }
        }
    }
}
