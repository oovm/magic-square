//! https://github.com/GalAster/WolframFunctionRepository/blob/master/MagicSquare/MagicSquare.m

use ndarray::Array2;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MagicSquare(usize);

// noinspection DuplicatedCode
impl MagicSquare {
    /// Create a rank-n magic square.
    pub fn new(n: usize) -> Self {
        Self(n)
    }
    /// The line sum of each line in magic square
    pub const fn line_sum(&self) -> usize {
        let n = self.0;
        (n * n + 1) * n / 2
    }
    /// The total sum of all numbers
    pub const fn sum(&self) -> usize {
        let n = self.0;
        (n * n + 1) * n * n / 2
    }
    /// Get the index of the position (row, col)
    pub fn get_position(&self, index: usize) -> (usize, usize) {
        let _ = index;
        todo!()
    }
    pub fn get_array(&self) -> Option<Array2<usize>> {
        match self.0 {
            0 | 2 => None,
            n if n % 2 != 0 => Some(self.odd()),
            n if n % 4 != 0 => Some(self.single_even()),
            _ => Some(self.double_even()),
        }
    }
    /// [MagicOdd2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L24-L27)
    fn odd(&self) -> Array2<usize> {
        let n = self.0;
        let mut matrix = Array2::zeros((n, n));
        for r in 0..n {
            for c in 0..n {
                let a = 1 + r + c + (n >> 1);
                let b = 1 + r + 2 * c;
                matrix[[r, c]] = n * (a % n) + (b % n) + 1
            }
        }
        matrix
    }
    /// [MagicEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L28-L41)
    fn single_even(&self) -> Array2<usize> {
        let n = self.0;
        let mut matrix = Array2::zeros((n, n));
        let size = n * n;
        let half = n / 2;
        let sub_size = size / 4;
        let half_square = MagicSquare::new(half).odd();
        let factors = [0, 2, 3, 1];
        let cols_left = half / 2;
        let cols_right = cols_left - 1;
        for r in 0..n {
            for c in 0..n {
                let local_r = if (c < cols_left || c >= n - cols_right || c == cols_left && r % half == cols_left)
                    && !(c == 0 && r % half == cols_left)
                {
                    if r >= half { r - half } else { r + half }
                }
                else {
                    r
                };
                let quadrant = local_r / half * 2 + c / half;
                let v = half_square[[local_r % half, c % half]];
                matrix[[r, c]] = v + factors[quadrant] * sub_size
            }
        }
        matrix
    }
    /// [MagicDoubleEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L42-L48)
    fn double_even(&self) -> Array2<usize> {
        let n: usize = self.0;
        let mut matrix = Array2::zeros((self.0, self.0));
        const BITS: usize = 0x9669usize;
        let size = n * n;
        let mult = n / 4;
        for r in 0..n {
            for c in 0..n {
                let i = n * r + c;
                let bit_pos = c / mult + (r / mult) * 4;
                matrix[[r, c]] = match BITS & (1 << bit_pos) != 0 {
                    true => i + 1,
                    false => size - i,
                }
            }
        }
        matrix
    }
}

impl Debug for MagicSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MagicSquare") //
            .field("rank", &self.0)
            .field("line_sum", &self.line_sum())
            .field("sum", &self.sum())
            .finish()
    }
}

impl Display for MagicSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array = match self.get_array() {
            None => return Ok(()),
            Some(s) => s,
        };
        let width = (self.0 * self.0).to_string().len() + 1;
        for row in array.rows() {
            for column in row {
                write!(f, "{e:>w$}", e = column, w = width)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
