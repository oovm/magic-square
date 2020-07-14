fn odd(n: usize) -> Vec<Vec<usize>> {
    let mut square = vec![vec![0; n]; n];
    for (i, row) in square.iter_mut().enumerate() {
        for (j, e) in row.iter_mut().enumerate() {
            *e = n * (((i + 1) + (j + 1) - 1 + (n >> 1)) % n) + (((i + 1) + (2 * (j + 1)) - 2) % n) + 1;
        }
    }
    return square;
}

fn even(n: usize) -> Vec<Vec<usize>> {
    unimplemented!()
}

fn double_even(n: usize) -> Vec<Vec<usize>> {
    unimplemented!()
}

pub fn magic(n: usize) -> Vec<Vec<usize>> {
    if n == 0 | 2 {
        // no solution
        vec![]
    } else if n % 2 == 1 {
        odd(n)
    } else if n % 4 != 0 {
        even(n)
    } else {
        double_even(n)
    }
}

#[test]
fn main() {
    let n = 9;
    for i in magic(n) {
        for j in i {
            print!("{} ", j)
        }
        println!();
    }
    let sum = n * (((n * n) + 1) / 2);
    println!("The sum of the square is {}.", sum);
}
