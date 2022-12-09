use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct Vec2D<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Vec2D<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_size(capacity, 0, 0)
    }

    pub fn with_capacity_and_size(capacity: usize, width: usize, height: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            width,
            height,
        }
    }

    pub fn from(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            inner: data,
            width,
            height,
        }
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
    {
        self.inner.extend(iter);
    }

    pub fn at(&self, row: usize, column: usize) -> &T {
        &self.inner[self.to_index(row, column)]
    }

    pub fn at_mut(&mut self, row: usize, column: usize) -> &mut T {
        let index = self.to_index(row, column);

        &mut self.inner[index]
    }

    pub fn at_range(&self, from: (usize, usize), to: (usize, usize)) -> &[T] {
        &self.inner[self.to_index(from.0, from.1)..self.to_index(to.0, to.1)]
    }

    pub fn at_range_mut(&mut self, from: (usize, usize), to: (usize, usize)) -> &[T] {
        let index_from = self.to_index(from.0, from.1);
        let index_to = self.to_index(to.0, to.1);

        &mut self.inner[index_from..index_to]
    }

    pub fn growing_at_mut(&mut self, row: usize, column: usize) -> &mut T where T: Default + Debug {
        // If empty add at least one
        if self.inner.is_empty() {
            self.inner.push(T::default());
            self.set_size(1, 1);
        }

        // TODO: Reserve, and this goes wrong when height > 1
        if column >= self.width {
            println!("growing width");

            let missing = (column - self.width) + 1;

            // Extend capacity
            self.inner.reserve(missing * self.height);

            // for _ in 0..missing {
                for i in 0..self.height {
                    println!("should grow row {i} by {missing}");
                    for _ in 0..missing {
                        self.inner.insert(self.width * i, T::default());
                    }
                }

                // println!("mssing = {missing}, insert {i}");
                // self.inner.insert(self.width * i, T::default());
                // // self.push(T::default())
            // }
            self.width += missing;
        }

        if row >= self.height {
            let missing = (row - self.height) + 1;

            // Extend capacity
            self.inner.reserve(missing * self.width);

            for _ in 0..missing {
                for _ in 0..self.width {
                    self.push(T::default());
                }
            }
            self.height += missing;
        }

        dbg!(&self);

        self.at_mut(row, column)
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    fn to_index(&self, row: usize, column: usize) -> usize {
        (row * self.width) + column
    }
}

impl<T> Debug for Vec2D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut inner_format: Vec<String> = Vec::new();

        if !self.inner.is_empty() {
            for chunk in self.inner.chunks(self.width) {
                inner_format.push(format!("{chunk:?}"));
            }
        }

        f.debug_struct(&format!("Vec2D<{}>", stringify!(T)))
            .field("width", &self.width)
            .field("height", &self.height)
            .field("inner", &inner_format)
            .finish()
            }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing_works() {
        let vec2d = Vec2D::from(
            vec![
                1, 2, 3, // Row 1
                4, 5, 6, // Row 2
                7, 8, 9, // Row 3
            ],
            3,
            3,
        );

        assert_eq!((3, 3), vec2d.size());

        assert_eq!(&1, vec2d.at(0, 0));
        assert_eq!(&3, vec2d.at(0, 2));
        assert_eq!(&4, vec2d.at(1, 0));
        assert_eq!(&8, vec2d.at(2, 1));

        assert_eq!(&[5, 6, 7], vec2d.at_range((1, 1), (2, 1)));
    }

    #[test]
    fn growing_works() {
        let mut vec2d: Vec2D<i32> = Vec2D::default();

        *vec2d.growing_at_mut(0, 0) = 1;
        assert_eq!((1, 1), vec2d.size());
        assert_eq!(&1, vec2d.at(0, 0));

        *vec2d.growing_at_mut(2, 2) = 1;
        assert_eq!((3, 3), vec2d.size());
        assert_eq!(&1, vec2d.at(2, 2));
    }

    #[test]
    fn growing_works_2() {
        let mut vec2d: Vec2D<i32> = Vec2D::from(vec![
            1, 0, 0,
            0, 1, 0,
            0, 0, 1
        ], 3, 3);

        dbg!(&vec2d);

        *vec2d.growing_at_mut(2, 4) = 1;

        // TODO: Something wrong, expect
        // 1, 0, 0, 0, 0
        // 0, 1, 0, 0, 0
        // 0, 0, 1, 0, 1

        dbg!(&vec2d);

        println!("--");

        *vec2d.growing_at_mut(4, 2) = 1;

        dbg!(&vec2d);
    }
}
