//! https://github.com/GalAster/WolframFunctionRepository/blob/master/MagicCube/MagicCube.m

use ndarray::Array3;

pub struct MagicCube(usize);

impl MagicCube {
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
        let mut matrix = Array3::zeros((self.0, self.0, self.0));
        for i in 0..self.0 {
            for j in 0..self.0 {
                todo!()
            }
        }
        matrix
    }
    /// [MagicEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L28-L41)
    fn even(&self) -> Array3<usize> {
        let mut matrix = Array3::zeros((self.0, self.0, self.0));

        matrix
    }
    /// [MagicDoubleEven2D](https://github.com/oovm/WolframFunctionRepository/blob/512caa033d3dde70d50317e3e4acc4f7fd2dbe4c/MagicSquare/MagicSquare.m#L42-L48)
    fn double_even(&self) -> Array3<usize> {
        let mut matrix = Array3::zeros((self.0, self.0, self.0));

        matrix
    }
}

#[rustfmt::skip]
fn odd(n: isize) -> Vec<Vec<Vec<usize>>> {
    (0..n).map(|i|
        (0..n).map(|j|
            (0..n).map(|k|
                (n.pow(2) * (i - j + k).rem_euclid(n) + n * (i - j - k - 1).rem_euclid(n) + (i + j + k + 1).rem_euclid(n) + 1) as usize
            ).collect()
        ).collect()
    ).collect()
}

fn even(_n: usize) -> Vec<Vec<Vec<usize>>> {
    unimplemented!()
}

fn double_even(_n: usize) -> Vec<Vec<Vec<usize>>> {
    unimplemented!()
}
