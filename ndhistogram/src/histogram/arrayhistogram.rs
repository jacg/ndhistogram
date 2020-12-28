use std::{iter::Map, ops::AddAssign};

use num::One;

use super::{Fill, FillWeight, Histogram, Item, MutableHistogram};
use crate::axes::Axes;
pub struct ArrayHistogram<A, V> {
    axes: A,
    values: Vec<V>,
}

impl<A: Axes, V: Default + Clone> ArrayHistogram<A, V> {
    pub fn new(axes: A) -> ArrayHistogram<A, V> {
        let size = axes.size();
        ArrayHistogram {
            axes,
            values: vec![V::default(); size],
        }
    }
}

impl<'a, A: Axes + 'a, V: One + AddAssign + 'a> Histogram<'a, A, V> for ArrayHistogram<A, V> {
    type Values = std::slice::Iter<'a, V>;
    type Iter = Box<dyn Iterator<Item = Item<'a, A::BinRange, V>> + 'a>;

    fn value(&self, coordinate: A::Coordinate) -> Option<&V> {
        let index = self.axes.index(coordinate);
        self.values.get(index)
    }

    fn axes(&self) -> &A {
        &self.axes
    }

    fn value_at_index(&self, index: usize) -> Option<&V> {
        self.values.get(index)
    }

    fn values(&'a self) -> Self::Values {
        self.values.iter()
    }

    fn iter(&'a self) -> Self::Iter {
        Box::new(self.axes().iter().map(move |(index, binrange)| Item {
            index,
            bin: binrange,
            value: self.value_at_index(index).unwrap(),
        }))
    }
}

impl<A: Axes, V: One + AddAssign> Fill<A> for ArrayHistogram<A, V> {
    fn fill(&mut self, coordinate: A::Coordinate) {
        let index = self.axes.index(coordinate);
        self.values[index] += V::one();
    }
}

impl<A: Axes, V: AddAssign<W>, W> FillWeight<A, W> for ArrayHistogram<A, V> {
    fn fill_weight(&mut self, coordinate: A::Coordinate, weight: W) {
        let index = self.axes.index(coordinate);
        self.values[index] += weight;
    }
}

impl<'a, A: Axes + 'a, V: One + AddAssign + 'a> MutableHistogram<'a, A, V>
    for ArrayHistogram<A, V>
{
    type ValuesMut = std::slice::IterMut<'a, V>;

    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.values.get_mut(index)
    }

    fn values(&'a mut self) -> Self::ValuesMut {
        todo!()
    }
}
