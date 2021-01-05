use crate::axis::binrange::SingleValuedBinRange;
use crate::axis::{Axis, Category};
use std::{convert::TryFrom, ops::Range};

fn category_ab() -> (Vec<String>, Category<String>) {
    let cats: Vec<String> = vec![String::from("A"), String::from("B")];
    (cats.clone(), Category::new(cats))
}

fn category_abc() -> (Vec<String>, Category<String>) {
    let cats: Vec<String> = vec![String::from("A"), String::from("B"), String::from("C")];
    (cats.clone(), Category::new(cats))
}

#[test]
fn test_category_numbins() {
    let (_, ax) = category_abc();
    assert_eq!(ax.numbins(), 3 + 1)
}

#[test]
fn test_category_get_index() {
    let (cats, ax) = category_ab();
    let actual: Vec<usize> = cats.iter().map(|c| ax.index(&c).unwrap()).collect();
    let expected = vec![0, 1];
    assert_eq!(expected, actual)
}

#[test]
fn test_category_get_index_overflow() {
    let cats = vec!["A", "B"];
    let ax = Category::new(cats);
    assert_eq!(ax.index(&"D").unwrap(), 2)
}

#[test]
fn test_category_get_bin() {
    let cats = vec!["A", "B", "C"];
    let ax = Category::new(cats);
    let actual: Vec<_> = (0..5).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(SingleValuedBinRange::new("A")),
        Some(SingleValuedBinRange::new("B")),
        Some(SingleValuedBinRange::new("C")),
        Some(SingleValuedBinRange::overflow()),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_category_clone() {
    let ax = Category::new(vec!["A", "B", "C"]);
    assert_eq!(ax, ax.clone());
}

#[test]
fn test_category_debug_display() {
    let ax = Category::new(vec!["A", "B", "C"]);
    println!("{:?}", ax);
}

#[test]
fn test_category_display() {
    let (_, ax) = category_abc();
    println!("{}", ax);
    assert_eq!(format!("{}", ax), "{{A}, {B}, {C}, {overflow}}")
}

#[test]
fn test_category_iterate_indices() {
    let ax = Category::new(vec!["A", "B", "C"]);
    let actual: Vec<usize> = ax.indices().collect();
    let expected = vec![0, 1, 2, 3];
    assert_eq!(expected, actual);
}

#[test]
fn test_category_iterate_items() {
    let ax = Category::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<(usize, _)> = vec![
        (0, SingleValuedBinRange::new("A")),
        (1, SingleValuedBinRange::new("B")),
        (2, SingleValuedBinRange::overflow()),
    ];
    assert_eq!(expected, actual);
}

// #[test]
// fn test_category_iterate_bin() {
//     let ax = Category::new(1, 0.0, 1.0);
//     let actual: Vec<_> = ax.bins().collect();
//     let expected: Vec<_> = vec![
//         SingleValuedBinRange::underflow(0.0),
//         SingleValuedBinRange::new(0.0, 1.0),
//         SingleValuedBinRange::overflow(1.0),
//     ];
//     assert_eq!(expected, actual);
// }

// #[test]
// fn test_iter_axis() {
//     let ax = Category::new(100, 0.0, 100.0);
//     let expected: Vec<_> = ax.iter().collect();
//     let actual: Vec<_> = ax.into_iter().collect();
//     assert_eq!(expected, actual);
// }
