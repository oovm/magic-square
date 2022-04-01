mod dim2;
mod dim2_all;
pub mod dim3;

#[test]
fn test() {
    let m3 = MagicSquare::new(7);
    println!("{}", m3);
}

pub use dim2::MagicSquare;
