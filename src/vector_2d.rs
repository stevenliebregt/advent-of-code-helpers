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

        for chunk in self.inner.chunks(self.width) {
            inner_format.push(format!("{chunk:?}"));
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
}
