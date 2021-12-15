use std::convert::AsMut;

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    pub cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, initial_value: T) -> Grid<T> {
        Grid {
            width,
            height,
            cells: vec![initial_value; width * height],
        }
    }

    pub fn new_square(size: usize, initial_value: T) -> Grid<T> {
        Grid {
            width: size,
            height: size,
            cells: vec![initial_value; size * size],
        }
    }

    pub fn new_square_with_value_fn<F>(size: usize, cell_value_fn: F) -> Grid<T>
    where
        F: Fn(usize, usize) -> T,
    {
        let mut g = Grid {
            width: size,
            height: size,
            cells: Vec::with_capacity(size * size),
        };
        for y in 0..size {
            for x in 0..size {
                g.cells.push(cell_value_fn(x, y));
            }
        }
        g
    }

    pub fn new_square_with_rows(size: usize, rows: Vec<Vec<T>>) -> Grid<T> {
        let mut cells = Vec::new();
        for mut row in rows {
            cells.append(&mut row);
        }
        Grid {
            width: size,
            height: size,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.cells[self.cell_offset(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let offset = self.cell_offset(x, y);
        &mut self.cells[offset]
    }

    pub fn try_get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 { None }
        else {
            let x = x as usize;
            let y = y as usize;
            if x >= self.width || y >= self.height {
                None
            } else {
                Some(self.get(x, y))
            }
        }
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: &self,
            x: 0,
            y: 0,
        }
    }

    pub fn set_row(&mut self, y: usize, values: Vec<T>) {
        for (i, v) in values.into_iter().enumerate() {
            self[(i, y)] = v;
        }
    }

    pub fn get_row(&self, y: usize) -> Vec<T> {
        let start = self.cell_offset(0, y);
        let end = start + self.width;
        self.cells[start..end].to_vec()
    }

    pub fn get_col(&self, x: usize) -> Vec<&T> {
        let mut col = Vec::new();
        for y in 0..self.height {
            col.push(&self[(x, y)]);
        }
        col
    }

    fn cell_offset(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T: Clone> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.height {
            let (x, y, val) = (self.x, self.y, self.grid.get(self.x, self.y));
            self.x += 1;
            if self.x == self.grid.width {
                self.x = 0;
                self.y += 1;
            }
            Some((x, y, val))
        } else {
            None
        }
    }
}

impl<T: Clone> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &T {
        let (x, y) = coords;
        self.get(x, y)
    }
}

impl<T: Clone> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut T {
        let (x, y) = coords;
        self.get_mut(x, y)
    }
}

pub fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}
