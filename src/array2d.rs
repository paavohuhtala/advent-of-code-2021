use itertools::Itertools;

pub struct Array2D<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Array2D<T>
where
    T: Copy + 'static,
{
    pub fn from_data_and_rows(data: Vec<T>, rows: usize) -> Self {
        let width = data.len() / rows;
        Self {
            data,
            width,
            height: rows,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> T {
        self.data[(y as usize) * self.width + (x as usize)]
    }

    pub fn try_get_with_i(&self, x: i32, y: i32) -> Option<(usize, T)> {
        if x >= 0 && y >= 0 && x < (self.width as i32) && y < (self.height as i32) {
            Some(((y as usize) * self.width + (x as usize), self.get(x, y)))
        } else {
            None
        }
    }

    pub fn try_get_left(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x - 1, y)
    }

    pub fn try_get_right(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x + 1, y)
    }

    pub fn try_get_up(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x, y - 1)
    }

    pub fn try_get_down(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x, y + 1)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.data.iter().map(|x| *x)
    }

    pub fn i_to_coords(&self, i: usize) -> (i32, i32) {
        ((i % self.width) as i32, (i / self.width) as i32)
    }

    pub fn adjacent_mut<'a, 'b>(&'a mut self, i: usize, adjecent: &'b mut Vec<(usize, &'a mut T)>) {
        let (center_x, center_y) = self.i_to_coords(i);
        let width = self.width;
        // this is not efficient, but the borrow checker wouldn't have it any other way
        for (i, elem) in self.data.iter_mut().enumerate() {
            let (x, y) = i_to_coords(i, width);

            if x == center_x && y == center_y {
                continue;
            }

            if x >= center_x - 1 && x <= center_x + 1 && y >= center_y - 1 && y <= center_y + 1 {
                adjecent.push((i, elem));
            }
        }
    }

    pub fn adjacent_cardinal(&self, i: usize, adjacent: &mut Vec<(usize, T)>) {
        let (x, y) = self.i_to_coords(i);

        for neighbor in [
            self.try_get_with_i(x - 1, y),
            self.try_get_with_i(x + 1, y),
            self.try_get_with_i(x, y - 1),
            self.try_get_with_i(x, y + 1),
        ]
        .into_iter()
        .filter_map(|x| x)
        {
            adjacent.push(neighbor);
        }
    }

    pub fn map<B>(&self, f: impl Fn(T) -> B) -> Array2D<B> {
        let mut new_data = Vec::with_capacity(self.data.len());
        for elem in self.data.iter() {
            new_data.push(f(*elem));
        }
        Array2D {
            data: new_data,
            width: self.width,
            height: self.height,
        }
    }

    pub fn from_vec_vec(s: Vec<Vec<T>>) -> Self {
        let rows = s.len();
        let data = s.into_iter().flatten().collect();
        Array2D::from_data_and_rows(data, rows)
    }
}

fn i_to_coords(i: usize, width: usize) -> (i32, i32) {
    ((i % width) as i32, (i / width) as i32)
}

impl Array2D<u8> {
    pub fn from_string(s: &str) -> Self {
        let rows = s.lines().count();

        let data = s
            .lines()
            .flat_map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as u8))
            .collect_vec();

        Array2D::from_data_and_rows(data, rows)
    }
}

pub fn print_array2d(a: &Array2D<u8>) {
    for y in 0..a.height {
        for x in 0..a.width {
            print!("{}", a.get(x as i32, y as i32));
        }
        println!();
    }
}
