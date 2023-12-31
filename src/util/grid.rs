#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    data: Box<[T]>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Box<[T]>, width: usize, height: usize) -> Grid<T> {
        Grid {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.data[y * self.height + x])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&mut self.data[y * self.height + x])
    }

    pub fn get_signed(&self, x: isize, y: isize) -> Option<&T> {
        if x as usize >= self.width || y as usize >= self.height {
            return None;
        }

        Some(&self.data[y as usize * self.height + x as usize])
    }

    pub fn get_mut_signed(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x as usize >= self.width || y as usize >= self.height {
            return None;
        }

        Some(&mut self.data[y as usize * self.height + x as usize])
    }

    pub fn column<'grid>(&'grid self, column: usize) -> Option<Column<'grid, T>> {
        if column >= self.width {
            return None;
        }

        Some(Column { grid: self, column })
    }

    pub fn row<'grid>(&'grid self, row: usize) -> Option<Row<'grid, T>> {
        if row >= self.height {
            return None;
        }

        Some(Row { grid: self, row })
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column<'grid, T> {
    grid: &'grid Grid<T>,
    column: usize,
}

impl<'grid, T> Column<'grid, T> {
    pub fn get(&self, y: usize) -> Option<&'grid T> {
        if y >= self.grid.height {
            return None;
        }

        Some(&self.grid.data[y * self.grid.height + self.column])
    }

    pub fn iter(self) -> ColumnIter<'grid, T> {
        ColumnIter {
            grid: self.grid,
            row: 0,
            column: self.column,
        }
    }
}

pub struct ColumnIter<'grid, T> {
    grid: &'grid Grid<T>,
    row: usize,
    column: usize,
}

impl<'grid, T> Iterator for ColumnIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.column, self.row);

        if result.is_some() {
            self.row += 1;
        }

        result
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row<'grid, T> {
    grid: &'grid Grid<T>,
    row: usize,
}

impl<'grid, T> Row<'grid, T> {
    pub fn get(&self, x: usize) -> Option<&'grid T> {
        if x >= self.grid.width {
            return None;
        }

        Some(&self.grid.data[self.row * self.grid.height + x])
    }

    pub fn iter(self) -> RowIter<'grid, T> {
        RowIter {
            grid: self.grid,
            row: self.row,
            column: 0,
        }
    }
}

pub struct RowIter<'grid, T> {
    grid: &'grid Grid<T>,
    row: usize,
    column: usize,
}

impl<'grid, T> Iterator for RowIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.column, self.row);

        if result.is_some() {
            self.column += 1;
        }

        result
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub const fn offset(&self) -> (isize, isize) {
        match self {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        }
    }

    pub const fn opposite(&self) -> CardinalDirection {
        self.clockwise().clockwise()
    }

    pub const fn clockwise(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    pub const fn counterclockwise(&self) -> CardinalDirection {
        self.clockwise().clockwise().clockwise()
    }
}

pub const CARDINAL_DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::North,
    CardinalDirection::East,
    CardinalDirection::South,
    CardinalDirection::West,
];
