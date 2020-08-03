//! https://github.com/GalAster/WolframFunctionRepository/blob/master/MagicCube/MagicCube.m

#[rustfmt::skip]
fn odd(n: isize) -> Vec<Vec<Vec<usize>>> {
    (0..n).map(|i|
        (0..n).map(|j|
            (0..n).map(|k|
                (n.pow(2)*(i - j + k).rem_euclid(n) + n * (i - j - k -1).rem_euclid(n) + (i + j + k +1).rem_euclid(n) + 1) as usize
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

pub fn magic(n: usize) -> Vec<Vec<Vec<usize>>> {
    if n == 0 | 2 {
        // no solution
        vec![]
    }
    else if n % 2 != 0 {
        odd(n as isize)
    }
    else if n % 4 != 0 {
        even(n)
    }
    else {
        double_even(n)
    }
}

pub fn magic_print(n: usize) {
    let width = (n * n * n).to_string().len() + 1;
    println!("The sum of the rank-{} cube is {}.", n, (n.pow(3) + 1) * n.pow(3) / 2);
    for plane in magic(n) {
        for row in plane {
            for em in row {
                print!("{e:>w$}", e = em, w = width);
            }
            println!();
        }
        println!();
    }
}

#[test]
fn main() {
    magic_print(3)
}
