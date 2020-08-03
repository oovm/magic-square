//! https://github.com/GalAster/WolframFunctionRepository/blob/master/MagicSquare/MagicSquare.m

#[rustfmt::skip]
fn odd(n: usize) -> Vec<Vec<usize>> {
    (0..n).map(|r|
        (0..n).map(|c|
            n * (((c + 1) + (r + 1) - 1 + (n >> 1)) % n) + (((c + 1) + (2 * (r + 1)) - 2) % n) + 1
        ).collect()
    ).collect()
}

#[rustfmt::skip]
fn even(n: usize) -> Vec<Vec<usize>> {
    let size = n * n;
    let half = n / 2;
    let sub_square_size = size / 4;
    let sub_square = odd(half);
    let quadrant_factors = [0, 2, 3, 1];
    let cols_left = half / 2;
    let cols_right = cols_left - 1;

    (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    let cond = (c < cols_left || c >= n - cols_right || c == cols_left && r % half == cols_left) && !(c == 0 && r % half == cols_left);
                    let local = if cond { if r >= half { r - half } else { r + half } } else { r };
                    let quadrant = local / half * 2 + c / half;
                    let v = sub_square[local % half][c % half];
                    v + quadrant_factors[quadrant] * sub_square_size
                })
                .collect()
        })
        .collect()
}

#[rustfmt::skip]
fn double_even(n: usize) -> Vec<Vec<usize>> {
    let bits = 0b1001_0110_0110_1001usize;
    let size = n * n;
    let sub = n / 4;
    let mut i = 0;
    (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    i += 1;
                    let bit_pos = c / sub + (r / sub) * 4;
                    if bits & (1 << bit_pos) != 0 { i } else { size - i + 1 }
                })
                .collect()
        })
        .collect()
}

pub fn magic(n: usize) -> Vec<Vec<usize>> {
    if n == 0 | 2 {
        // no solution
        vec![]
    }
    else if n % 2 != 0 {
        odd(n)
    }
    else if n % 4 != 0 {
        even(n)
    }
    else {
        double_even(n)
    }
}

pub fn magic_print(n: usize) {
    let width = (n * n).to_string().len() + 1;
    let sum = (n * n + 1) * n / 2;
    println!("The sum of the rank-{} square is {}.", n, sum);
    for row in magic(n) {
        for em in row {
            print!("{e:>w$}", e = em, w = width);
        }
        println!();
    }
}
