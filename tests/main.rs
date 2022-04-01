use magicsquare::MagicSquare;

#[test]
fn test_dim2() {
    let ms = format!("{}", MagicSquare::new(8));
    let out = include_str!("d2-8.txt");
    assert_eq!(ms, out);
    let ms = format!("{}", MagicSquare::new(9));
    let out = include_str!("d2-9.txt");
    assert_eq!(ms, out);
    let ms = format!("{}", MagicSquare::new(10));
    let out = include_str!("d2-10.txt");
    assert_eq!(ms, out);
}
