mod dim2;
pub mod dim3;
mod dim2_all;

#[test]
fn test() {
    let m3 = MagicSquare::new(11);
    println!("{}", m3);
}

pub use dim2::MagicSquare;
