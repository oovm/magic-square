const SIZE: usize = 3;
const SUM: u32 = magic_constant(SIZE as u32);

const fn magic_constant(n: u32) -> u32 {
    return n * (n * n + 1) / 2;
}

const fn initial() -> [[u32; SIZE]; SIZE] {
    return [[0; SIZE]; SIZE];
}

fn display(grid: &[[u32; SIZE]; SIZE]) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{} ", col);
        }
        println!();
    }
    println!();
}

fn check(grid: &[[u32; SIZE]; SIZE]) -> bool {
    if grid[SIZE - 1][SIZE - 1] == 0 {
        return false;
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..SIZE {
        sum1 += grid[i][i];
        sum2 += grid[i][SIZE - i - 1];
    }
    if sum1 != SUM || sum2 != SUM {
        return false;
    }

    for row in grid.iter() {
        let mut total = 0;
        for elt in row.iter() {
            total += elt;
        }
        if total != SUM {
            return false;
        }
    }

    for row in grid.iter() {
        for (col_idx, _) in row.iter().enumerate() {
            let mut sum = 0;
            for i in 0..SIZE {
                sum += grid[i][col_idx];
            }
            if sum != SUM {
                return false;
            }
        }
    }

    return true;
}

fn in_grid(val: u32, grid: &[[u32; SIZE]; SIZE]) -> bool {
    for elt in grid.iter().flat_map(|r| r.iter()) {
        if u32::from(val) == *elt {
            return true;
        }
    }
    return false;
}

fn solve(mut grid: &mut [[u32; SIZE]; SIZE]) {
    let mut row = 0;
    let mut col = 0;
    for i in 0..SIZE.pow(2) {
        row = i / SIZE;
        col = i % SIZE;

        if grid[row][col] == 0 {
            for value in 1..SIZE.pow(2) + 1 {
                if !in_grid(value as u32, &grid) {
                    grid[row][col] = value as u32;
                    if check(&grid) {
                        display(&grid);
                    }
                    else {
                        solve(&mut grid);
                    }
                }
            }
            break;
        }
    }
    grid[row][col] = 0;
}

fn main() {
    solve(&mut initial());
}
