use std::fmt::{Debug, Formatter};

#[derive(Default, Eq, PartialEq)]
pub struct Vec2D<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
    negative_width: usize,
    negative_height: usize
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
            negative_width: 0,
            negative_height: 0,
        }
    }

    pub fn from(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            inner: data,
            width,
            height,
            negative_width: 0,
            negative_height: 0,
        }
    }

    pub fn from_negative(data: Vec<T>, width: usize, height: usize, negative_width: usize, negative_height: usize) -> Self {
        Self {
            inner: data,
            width,
            height,
            negative_width,
            negative_height
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

    pub fn at(&self, row: isize, column: isize) -> &T {
        &self.inner[self.to_index(row, column)]
    }

    pub fn at_mut(&mut self, row: isize, column: isize) -> &mut T {
        let index = self.to_index(row, column);

        &mut self.inner[index]
    }

    // pub fn at_range(&self, from: (usize, usize), to: (usize, usize)) -> &[T] {
    //     &self.inner[self.to_index(from.0, from.1)..self.to_index(to.0, to.1)]
    // }
    //
    // pub fn at_range_mut(&mut self, from: (isize, isize), to: (isize, isize)) -> &[T] {
    //     let index_from = self.to_index(from.0, from.1);
    //     let index_to = self.to_index(to.0, to.1);
    //
    //     &mut self.inner[index_from..index_to]
    // }

    pub fn growing_at_mut(&mut self, row: isize, column: isize) -> &mut T
    where
        T: Default + Debug,
    {
        // If empty add at least one
        if self.inner.is_empty() {
            self.inner.push(T::default());
            self.set_size(1, 1);
        }

        // Check if we need to grow in the positive axis
        if column >= self.width as isize {
            let missing = ((column - self.width as isize) + 1) as usize;

            // Extend capacity // TODO: Fix reservesusize
            self.inner.reserve(missing * self.height);
            println!("grow width");
            for i in (0..(self.height + self.negative_height)) {
                let index = self.to_index(i as isize, (self.width + self.negative_width) as isize) - 1;
                println!("\t idnex / {i} - {} = {index}", self.width + self.negative_width);
                for _ in 0..missing { // TODO: Optimize this
                    //self.width * (i + 1) + (i * missing) + self.negative_width
                    self.inner
                        .insert(index, T::default());
                }
            }

            self.width += missing;
        }

        println!("after width");
        dbg!(&self);

        if row >= self.height as isize {
            let missing = ((row - self.height as isize) + 1) as usize;

            // Extend capacity
            self.inner.reserve(missing * self.width);

            for _ in 0..missing {
                for _ in 0..self.width { // TODO: Optimize this
                    self.push(T::default());
                }
            }

            self.height += missing;
        }

        println!("after height");
        dbg!(&self);

        // Check if we need to grow in the negative axis
        if column < self.negative_width as isize {
            let missing = (column + self.negative_width as isize).abs() as usize;

            // Extend capacity
            self.inner.reserve(missing * self.height);

            for i in (0..(self.height + self.negative_height)).rev() {
                let index = self.to_index(i as isize, 0) - self.width;

                for _ in 0..missing { // TODO: Optimize this
                    self.inner.insert(index, T::default());
                }
            }

            self.negative_width += missing;
        }

        if row < self.negative_height as isize {
            let missing = (row + self.negative_height as isize).abs() as usize;

            // Extend capacity
            self.inner.reserve(missing * self.width);

            for _ in 0..missing {
                for _ in 0..self.width { // TODO: Optimize this
                    self.inner.insert(0, T::default());
                }
            }

            self.negative_height += missing;

        }

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
    fn to_index(&self, row: isize, column: isize) -> usize {
        let row_adjusted = (row + self.negative_height as isize) as usize;
        let column_adjusted = (column + self.negative_width as isize) as usize;

       let adjusted_index =  (row_adjusted * (self.width + self.negative_width)) + column_adjusted;

        println!("- {row},{column} / {},{} = {}", row_adjusted, column_adjusted, adjusted_index);
        adjusted_index

    }
}

// TODO: Debug doesn't work in negatives yet
impl<T> Debug for Vec2D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut inner_format: Vec<String> = Vec::new();

        if !self.inner.is_empty() {
            for chunk in self.inner.chunks(self.width + self.negative_width) {
                inner_format.push(format!("{chunk:?}"));
            }
        }

        f.debug_struct(&format!("Vec2D<{}>", std::any::type_name::<T>()))
            .field("width", &self.width)
            .field("height", &self.height)
            .field("negative_width", &self.negative_width)
            .field("negative_height", &self.negative_height)
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
    }

    #[test]
    fn growing_works() {
        let mut vec2d: Vec2D<i32> = Vec2D::default();

        *vec2d.growing_at_mut(0, 0) = 1;
        let expected = Vec2D::from(vec![1], 1, 1);
        assert_eq!(expected, vec2d);

        *vec2d.growing_at_mut(2, 2) = 2;
        let expected = Vec2D::from(
            vec![
                1, 0, 0, //
                0, 0, 0, //
                0, 0, 2, //
            ],
            3,
            3,
        );
        assert_eq!(expected, vec2d);

        *vec2d.growing_at_mut(2, 4) = 3;
        let expected = Vec2D::from(
            vec![
                1, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, //
                0, 0, 2, 0, 3, //
            ],
            5,
            3,
        );
        assert_eq!(expected, vec2d);

        *vec2d.growing_at_mut(4, 1) = 4;
        let expected = Vec2D::from(
            vec![
                1, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, //
                0, 0, 2, 0, 3, //
                0, 0, 0, 0, 0, //
                0, 4, 0, 0, 0, //
            ],
            5,
            5,
        );
        assert_eq!(expected, vec2d);
    }

    #[test]
    fn indexing_negatively_works() {
        let vec2d: Vec2D<i32> = Vec2D::from_negative(vec![
            4, 5, // (-2, -1) and (-2, 0)
            9, 0, // (-1, -1) and (-1, 0)
            0, 0, // (0, -1) and (0, 0)
        ], 1, 1, 1, 2);

        dbg!(&vec2d);

        assert_eq!(&9, vec2d.at(-1, -1));
        assert_eq!(&4, vec2d.at(-2, -1));
        assert_eq!(&5, vec2d.at(-2, 0));
    }

    #[test]
    fn growing_negatively_works() {
        struct Char(char);

        impl Default for Char {
            fn default() -> Self {
                Self('.')
            }
        }

        impl Debug for Char {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        // let mut vec2d: Vec2D<Char> = Vec2D::from(vec![Char('X'), Char('Y')], 1, 2);
        //
        // dbg!(&vec2d);
        //
        // *vec2d.growing_at_mut(1, -3) = Char('A');
        //
        // dbg!(&vec2d);

        // let mut a = Vec2D::from_negative(vec![
        //     '.', '.', '.', 'X', //
        //     'A', '.', '.', 'Y', //
        // ], 1, 2, 3, 0);
        //
        // assert_eq!(&'A', a.at(1, -3));


        let mut vec2d: Vec2D<i32> = Vec2D::from(vec![
            1, 0, 0, //
            0, 2, 0, //
            0, 0, 3, //
        ], 3, 3);

        *vec2d.growing_at_mut(-1, 0) = 4;
        let expected = Vec2D::from_negative(
            vec![
                4, 0, 0, // row -1, column 0
                1, 0, 0, //
                0, 2, 0, //
                0, 0, 3, //
            ],
            3,
            3,
            0,
            1
        );
        assert_eq!(expected, vec2d);

        *vec2d.growing_at_mut(1, -3) = 5;
        dbg!(&vec2d);
        // Insertions should go
        // For first column, insert at:
        //      0 (before 4)
        //
        let expected = Vec2D::from_negative(
            vec![
                0, 0, 0, 4, 0, 0, // row -1
                0, 0, 0, 1, 0, 0, //
                5, 0, 0, 0, 2, 0, // row 1, column -3
                0, 0, 0, 0, 0, 3, //
            ],
            3,
            3,
            3,
            1
        );
        assert_eq!(expected, vec2d);

        dbg!(&vec2d);

        *vec2d.growing_at_mut(3, 3) = 6;

        dbg!(&vec2d);
    }

    mod it_generates_correct_vec_index {
        use super::*;

        #[test]
        fn normal_case() {
            let vec2d = Vec2D::from(vec![
                'A', 'B', 'C', 'D', //
                'E', 'F', 'G', 'H', //
                'I', 'J', 'K', 'L', //
            ], 4, 3);

            // We want G
            assert_eq!(6, vec2d.to_index(1, 2));
        }

        #[test]
        fn case_with_negative_height() {
            let vec2d = Vec2D::from_negative(vec![
                'A', 'B', 'C', 'D', //
                'E', 'F', 'G', 'H', //
                'I', 'J', 'K', 'L', //
            ], 4, 2, 0, 1);

            // The first row is the negative
            // We want J
            assert_eq!(9, vec2d.to_index(1, 1));
        }

        #[test]
        fn case_with_negative_width() {
            let vec2d = Vec2D::from_negative(vec![
                'A', 'B', 'C', 'D', //
                'E', 'F', 'G', 'H', //
                'I', 'J', 'K', 'L', //
            ], 3, 3, 1, 0);

            // The first column is the negative
            // We want E
            assert_eq!(4, vec2d.to_index(1, -1));
        }

        #[test]
        fn case_with_negative_height_and_width() {
            let vec2d = Vec2D::from_negative(vec![
                'A', 'B', 'C', 'D', //
                'E', 'F', 'G', 'H', //
                'I', 'J', 'K', 'L', //
            ], 2, 1, 2, 2);

            // The first 2 columns and rows are the negative
            // We want B
            assert_eq!(1, vec2d.to_index(-2, -1), "Invalid index for B");

            // We want L
            assert_eq!(11, vec2d.to_index(0, 1), "Invalid index for L");
        }
    }
}
