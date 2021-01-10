use crate::{
    axis::{binrange::SingleValuedBinRange, Category},
    {Fill, Histogram, Item},
};

#[test]
#[should_panic]
fn test_histogram_category_grows() {
    let mut hist = ndhistogram!(Category::growable(vec!["A", "B"]); i32);
    assert_eq!(hist.value(&"C"), None);
    hist.fill(&"C");
    assert_eq!(hist.value(&"C"), Some(&1));
    // check every bin value
    let actual: Vec<_> = hist.iter().collect();
    let expected = vec![
        Item::new(0, SingleValuedBinRange::new("A"), &0),
        Item::new(1, SingleValuedBinRange::new("B"), &0),
        Item::new(2, SingleValuedBinRange::new("C"), &1),
    ];
    assert_eq!(expected, actual);
}
