//! https://github.com/GalAster/WolframFunctionRepository/blob/master/MagicSquare/MagicSquare.m

use ndarray::{arr2, Array2, NdIndex};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fmt::{Debug, Display, Formatter};

pub struct MagicSquare(usize);

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
        (n * n + 1) * n / 2
    }
    pub fn get_position(&self, index: usize) -> (usize, usize) {
        let _ = index;
        todo!()
    }

    pub fn get_array(&self) -> Option<Array2<usize>> {
        match self.0 {
            0 | 2 => None,
            n if n % 2 != 0 => Some(self.odd()),
            n if n % 4 != 0 => Some(self.even()),
            _ => Some(self.double_even()),
        }
    }
    /// [MagicOdd2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L24-L27)
    fn odd(&self) -> Array2<usize> {
        let n = self.0;
        let mut matrix = Array2::zeros((n, n));
        for r in 0..n {
            for c in 0..n {
                let a = ((r + 1) + (c + 1) - 1 + (n >> 1));
                let b = ((r + 1) + (2 * (c + 1)) - 2);
                matrix[[r, c]] = n * (a % n) + (b % n) + 1
            }
        }
        matrix
    }
    /// [MagicEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L28-L41)
    fn even(&self) -> Array2<usize> {
        let mut matrix = Array2::zeros((self.0, self.0));

        matrix
    }
    /// [MagicDoubleEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L42-L48)
    fn double_even(&self) -> Array2<usize> {
        let n: usize = self.0;
        let mut matrix = Array2::zeros((self.0, self.0));
        let bits = 0b1001_0110_0110_1001usize;
        let size = n * n;
        let mult = n / 4;
        for r in 0..n {
            for c in 0..n {
                let i = n * r + c;
                let bit_pos = c / mult + (r / mult) * 4;
                matrix[[r, c]] = match bits & (1 << bit_pos) != 0 {
                    true => i + 1,
                    false => size - i,
                }
            }
        }
        matrix
    }
}

#[rustfmt::skip]
fn double_even(n: usize) -> Vec<Vec<usize>> {
    let bits = 0b1001_0110_0110_1001usize;
    let size = n * n;
    let sub = n / 4;
    (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    let i = r * n + c + 1;
                    let bit_pos = c / sub + (r / sub) * 4;
                    if bits & (1 << bit_pos) != 0 { i } else { size - i + 1 }
                })
                .collect()
        })
        .collect()
}

impl Display for MagicSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array = match self.get_array() {
            None => {
                return Ok(());
            }
            Some(s) => s,
        };
        let width = (self.0 * self.0).to_string().len() + 1;
        for row in array.rows() {
            for column in row {
                write!(f, "{e:>w$}", e = column, w = width);
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
