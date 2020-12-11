use std::{fmt, ops::{Index, IndexMut}};

/// A two-dimensional array.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Matrix<T> {
    width: usize,
    elements: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn from_vec(elements: Vec<T>, width: usize) -> Matrix<T> {
        assert!(elements.len() > width);
        assert!(elements.len() % width == 0);

        Matrix { elements, width }
    }

    pub fn map<U>(&self, f: impl Fn(usize, usize, &T) -> U) -> Matrix<U> {
        let mut output_elements = Vec::with_capacity(self.elements.len());

        for (index, element) in self.elements.iter().enumerate() {
            let row = index / self.width;
            let column = index - (self.width * row);

            output_elements.push(f(row, column, element));
        }

        Matrix {
            width: self.width,
            elements: output_elements
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.elements.len() / self.width
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.width;
        &self.elements[start..start + self.width]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut[T] {
        let start = row * self.width;
        &mut self.elements[start..start + self.width]
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, element) in self.elements.iter().enumerate() {
            if index % self.width == 0 {
                writeln!(f)?;
            }

            write!(f, "{}", element)?;           
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_matrix() {
        let m = Matrix::from_vec(vec![1, 2, 3, 4], 2);
        let m2 = m.map(|row, column, value| (row, column, value + value));

        assert_eq!((0, 0, 2), m2[0][0]);
        assert_eq!((0, 1, 4), m2[0][1]);
        assert_eq!((1, 0, 6), m2[1][0]);
        assert_eq!((1, 1, 8), m2[1][1]);
    }
}