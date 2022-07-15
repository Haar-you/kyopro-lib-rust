pub mod to_graph;

#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub const L: Position = Position { x: 0, y: -1 };
    pub const R: Position = Position { x: 0, y: 1 };
    pub const U: Position = Position { x: -1, y: 0 };
    pub const D: Position = Position { x: 1, y: 0 };
    pub const DIR_4: [Position; 4] = [Self::L, Self::R, Self::U, Self::D];
}

impl std::ops::Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
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
                f(Position::new(i as isize, j as isize), c);
            }
        }
    }
}
