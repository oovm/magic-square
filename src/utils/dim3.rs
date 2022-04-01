//! https://github.com/GalAster/WolframFunctionRepository/blob/master/MagicCube/MagicCube.m

use ndarray::Array3;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Add,
};

pub struct MagicCube(usize);

// noinspection DuplicatedCode
impl MagicCube {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
    pub fn sum_line(&self) {}
    pub fn sum_face(&self) {}
    pub fn sum_all(&self) -> usize {
        let a = self.0.pow(3);
        let b = self.0.add(1).pow(3);
        a * b / 2
    }
    pub fn get_array(&self) -> Option<Array3<usize>> {
        match self.0 {
            0 | 2 => None,
            n if n % 2 != 0 => Some(self.odd()),
            n if n % 4 != 0 => Some(self.even()),
            _ => Some(self.double_even()),
        }
    }
    /// [MagicOdd2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L24-L27)
    fn odd(&self) -> Array3<usize> {
        let n = self.0 as i128;
        let mut matrix = Array3::zeros((self.0, self.0, self.0));
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let a = n.pow(2) * (i - j + k).rem_euclid(n);
                    let b = n * (i - j - k - 1).rem_euclid(n);
                    let c = (i + j + k + 1).rem_euclid(n);
                    matrix[[i as usize, j as usize, k as usize]] = (a + b + c + 1) as usize
                }
            }
        }
        matrix
    }
    /// [MagicEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L28-L41)
    fn even(&self) -> Array3<usize> {
        let matrix = Array3::zeros((self.0, self.0, self.0));

        matrix
    }
    /// [MagicDoubleEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L42-L48)
    fn double_even(&self) -> Array3<usize> {
        let matrix = Array3::zeros((self.0, self.0, self.0));

        matrix
    }
}

impl Debug for MagicCube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MagicCube") //
            .field("rank", &self.0)
            .field("line-sum", &self.sum_line())
            .field("face-sum", &self.sum_face())
            .finish()
    }
}

impl Display for MagicCube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array = match self.get_array() {
            None => return Ok(()),
            Some(s) => s,
        };

        let width = self.0.pow(3).to_string().len() + 1;
        for row in array.rows() {
            for column in row {
                write!(f, "{e:>w$}", e = column, w = width)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
