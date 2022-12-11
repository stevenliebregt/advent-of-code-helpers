use std::fmt::{Debug, Formatter};

#[derive(Default, Eq, PartialEq)]
pub struct Vec2D<T> {
    inner: Vec<T>,
    positive_width: usize,
    positive_height: usize,
    negative_width: usize,
    negative_height: usize,
}

impl<T> Vec2D<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_size(capacity, 0, 0)
    }

    pub fn with_capacity_and_size(capacity: usize, width: usize, height: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            positive_width: width,
            positive_height: height,
            negative_width: 0,
            negative_height: 0,
        }
    }

    pub fn from(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            inner: data,
            positive_width: width,
            positive_height: height,
            negative_width: 0,
            negative_height: 0,
        }
    }

    pub fn from_negative(
        data: Vec<T>,
        width: usize,
        height: usize,
        negative_width: usize,
        negative_height: usize,
    ) -> Self {
        Self {
            inner: data,
            positive_width: width,
            positive_height: height,
            negative_width,
            negative_height,
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

    // TODO: Ensure that we reserve enough space at each step according to negative_width and height
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
        if column >= self.positive_width as isize {
            let missing = (column as usize - self.positive_width) + 1;

            // Extend capacity
            self.inner
                .reserve(missing * (self.positive_height + self.negative_height));

            for i in 0..(self.positive_height + self.negative_height) {
                for _ in 0..missing {
                    self.inner
                        .insert(self.positive_width * (i + 1) + (i * missing), T::default());
                }
            }

            self.positive_width += missing;
        }

        if row >= self.positive_height as isize {
            let missing = (row as usize - self.positive_height) + 1;

            // Extend capacity
            self.inner
                .reserve(missing * (self.positive_width + self.negative_width));

            for _ in 0..missing {
                for _ in 0..self.positive_width {
                    self.push(T::default());
                }
            }

            self.positive_height += missing;
        }

        // Check if we need to grow in negative axis
        if column < -(self.negative_width as isize) {
            println!("grow in negative width");
            let missing = (column + self.negative_width as isize).abs() as usize;
            println!("missing = {missing}");
            // TODO: reserve

            // Grow every row
            for i in (0..self.positive_height + self.negative_height).rev() {
                // let mut index = self.to_index(i as isize, 0);
                // println!("\tgrowing row = {i} @ {index}");
                //
                // // TODO: Check
                // index = index.saturating_sub(self.negative_width + self.width);
                // println!("\t\tindex_mod = {index}");

                let index = i * (self.positive_width + self.negative_width);
                println!("\tgrowing row = {i} @ {index}");

                for _ in 0..missing {
                    self.inner.insert(index, T::default());
                }
            }

            self.negative_width += missing;
        }

        if row < -(self.negative_height as isize) {
            println!("grow in negative height");
            let missing = (row + self.negative_height as isize).abs() as usize; // TODO: Should logic for growing heights not be the same (except insert index)
            println!("missing = {missing}");
            // TODO: reserve

            // Grow every column
            for i in 0..self.positive_width + self.negative_width {
                let index = self.to_index(0, i as isize);
                // println!("\tgrowing column = {i} @ {index}");
                for _ in 0..missing {
                    self.inner.insert(0, T::default()); // TODO: Verify, growing height is always inserted in front
                                                        // self.inner.insert(index, T::default());
                }
            }

            self.negative_height += missing;
        }

        /*
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
         */

        self.at_mut(row, column)
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.positive_width = width;
        self.positive_height = height;
    }

    pub fn size(&self) -> (usize, usize) {
        (self.positive_width, self.positive_height)
    }

    #[inline]
    fn to_index(&self, row: isize, column: isize) -> usize {
        let row_adjusted = (row + self.negative_height as isize) as usize;
        let column_adjusted = (column + self.negative_width as isize) as usize;

        let adjusted_index = (row_adjusted * (self.positive_width + self.negative_width)) + column_adjusted;

        adjusted_index
    }
}

impl<T> Debug for Vec2D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut inner_format: Vec<String> = Vec::new();

        if !self.inner.is_empty() {
            for chunk in self.inner.chunks(self.positive_width + self.negative_width) {
                inner_format.push(format!("{chunk:?}"));
            }
        }

        f.debug_struct(&format!("Vec2D<{}>", std::any::type_name::<T>()))
            .field("width", &self.positive_width)
            .field("height", &self.positive_height)
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

    mod growing {
        use super::*;

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

        // TODO: Grow with negative width and positive height
        // TODO: Grow with negative width and negative height

        #[test]
        fn growing_width_with_negative_height_works() {
            let mut vec2d: Vec2D<i32> = Vec2D::from_negative(vec![
                1, 2, //
                3, 4, //
                5, 6, //
            ], 2, 1, 0, 2);

            *vec2d.growing_at_mut(-2, 3) = 9;

            let expected = Vec2D::from_negative(vec![
                1, 2, 0, 9, //
                3, 4, 0, 0, //
                5, 6, 0, 0, //
            ],4, 1, 0, 2);
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_width_with_negative_width_works() {
            let mut vec2d: Vec2D<i32> = Vec2D::from_negative(vec![1, 0], 1, 1, 1, 0);

            *vec2d.growing_at_mut(0, 3) = 9;

            let expected = Vec2D::from_negative(vec![1, 0, 0, 0, 9], 4, 1, 1, 0);
            dbg!(&vec2d);
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_to_negative_width_1_height_works() {
            let mut vec2d: Vec2D<i32> = Vec2D::from(vec![1], 1, 1);

            *vec2d.growing_at_mut(0, -4) = 9;

            let expected = Vec2D::from_negative(vec![9, 0, 0, 0, 1], 1, 1, 4, 0);
            dbg!(&vec2d);
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_to_negative_width_3_height_works() {
            let mut vec2d: Vec2D<i32> = Vec2D::from(
                vec![
                    1, 2, //
                    3, 4, //
                    5, 6, //
                ],
                2,
                3,
            );

            *vec2d.growing_at_mut(1, -4) = 9;

            let expected = Vec2D::from_negative(
                vec![
                    0, 0, 0, 0, 1, 2, //
                    9, 0, 0, 0, 3, 4, //
                    0, 0, 0, 0, 5, 6, //
                ],
                2,
                3,
                4,
                0,
            );
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_to_negative_width_with_negative_height() {
            let mut vec2d: Vec2D<i32> = Vec2D::from_negative(
                vec![
                    1, 2, //
                    3, 4, //
                ],
                2,
                1,
                0,
                1,
            );

            *vec2d.growing_at_mut(0, -2) = 9;

            let expected = Vec2D::from_negative(
                vec![
                    0, 0, 1, 2, //
                    9, 0, 3, 4, //
                ],
                2, 1, 2, 1
            );
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_height_with_negative_height_works() {
            let mut vec2d: Vec2D<i32> = Vec2D::from_negative(
                vec![
                    4, //
                    3, //
                    2, //
                    1, //
                    0, //
                ],
                1,
                1,
                0,
                4,
            );

            *vec2d.growing_at_mut(3, 0) = 9;

            let expected = Vec2D::from_negative(
                vec![
                    4, //
                    3, //
                    2, //
                    1, //
                    0, // 0, 0
                    0, //
                    0, //
                    9, //
                ],
                1,
                4,
                0,
                4,
            );
            dbg!(&vec2d);
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_to_negative_height_1_width() {
            let mut vec2d = Vec2D::from(
                vec![
                    1, //
                    2, //
                ],
                1,
                2,
            );

            *vec2d.growing_at_mut(-3, 0) = 9;

            let expected = Vec2D::from_negative(
                vec![
                    9, //,
                    0, //
                    0, //
                    1, //
                    2, //
                ],
                1,
                2,
                0,
                3,
            );

            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_to_negative_height_3_width() {
            let mut vec2d = Vec2D::from(
                vec![
                    1, 2, 3, //
                    4, 5, 6, //
                    7, 8, 9, //
                ],
                3,
                3,
            );

            *vec2d.growing_at_mut(-2, 2) = 9;

            let expected = Vec2D::from_negative(
                vec![
                    0, 0, 9, //
                    0, 0, 0, //
                    1, 2, 3, //
                    4, 5, 6, //
                    7, 8, 9, //
                ],
                3,
                3,
                0,
                2,
            );

            assert_eq!(expected, vec2d);
        }

        // TODO: growing_to_negative_height_negative_width

        #[test]
        fn growing_to_negative_height_with_negative_width() {
            let mut vec2d = Vec2D::from_negative(
                vec![
                    1, 2, //
                    3, 4, //
                ],
                1,
                1,
                1,
                1,
            );

            *vec2d.growing_at_mut(-3, -1) = 9;

            let expected = Vec2D::from_negative(
                vec![
                    9, 0, //
                    0, 0, //
                    1, 2, //
                    3, 4, //
                ],
                1,
                1,
                1,
                3,
            );
            assert_eq!(expected, vec2d);
        }

        #[test]
        fn growing_height_with_negative_width_works() {
            let mut vec2d: Vec2D<i32> = Vec2D::from_negative(vec![
                1, 2, 3, //
            ], 1, 1, 2, 0);

            *vec2d.growing_at_mut(2, -1) = 9;

            let expected = Vec2D::from_negative(vec![
                1, 2, 3, //
                0, 0, 0, //
                0, 9, 0, //
            ], 1, 3, 2, 0);
            assert_eq!(expected, vec2d);
        }

        // TODO: Same as above but also negative height
    }

    #[test]
    fn indexing_negatively_works() {
        let vec2d: Vec2D<i32> = Vec2D::from_negative(
            vec![
                4, 5, // (-2, -1) and (-2, 0)
                9, 0, // (-1, -1) and (-1, 0)
                0, 0, // (0, -1) and (0, 0)
            ],
            1,
            1,
            1,
            2,
        );

        dbg!(&vec2d);

        assert_eq!(&9, vec2d.at(-1, -1));
        assert_eq!(&4, vec2d.at(-2, -1));
        assert_eq!(&5, vec2d.at(-2, 0));
    }

    mod it_generates_correct_vec_index {
        use super::*;

        #[test]
        fn normal_case() {
            let vec2d = Vec2D::from(
                vec![
                    'A', 'B', 'C', 'D', //
                    'E', 'F', 'G', 'H', //
                    'I', 'J', 'K', 'L', //
                ],
                4,
                3,
            );

            // We want G
            assert_eq!(6, vec2d.to_index(1, 2));
        }

        #[test]
        fn case_with_negative_height() {
            let vec2d = Vec2D::from_negative(
                vec![
                    'A', 'B', 'C', 'D', //
                    'E', 'F', 'G', 'H', //
                    'I', 'J', 'K', 'L', //
                ],
                4,
                2,
                0,
                1,
            );

            // The first row is the negative
            // We want J
            assert_eq!(9, vec2d.to_index(1, 1));
        }

        #[test]
        fn case_with_negative_width() {
            let vec2d = Vec2D::from_negative(
                vec![
                    'A', 'B', 'C', 'D', //
                    'E', 'F', 'G', 'H', //
                    'I', 'J', 'K', 'L', //
                ],
                3,
                3,
                1,
                0,
            );

            // The first column is the negative
            // We want E
            assert_eq!(4, vec2d.to_index(1, -1));
        }

        #[test]
        fn case_with_negative_height_and_width() {
            let vec2d = Vec2D::from_negative(
                vec![
                    'A', 'B', 'C', 'D', //
                    'E', 'F', 'G', 'H', //
                    'I', 'J', 'K', 'L', //
                ],
                2,
                1,
                2,
                2,
            );

            // The first 2 columns and rows are the negative
            // We want B
            assert_eq!(1, vec2d.to_index(-2, -1), "Invalid index for B");

            // We want L
            assert_eq!(11, vec2d.to_index(0, 1), "Invalid index for L");
        }

        #[test]
        fn case_special() {
            let vec2d = Vec2D::from_negative(vec![
                1, 2, 0, 9, //
                3, 4, 0, 0, //
                5, 6, 0, 0, //
            ],4, 1, 0, 2);

            // We want the 9
            assert_eq!(3, vec2d.to_index(-2, 3));
        }
    }
}
