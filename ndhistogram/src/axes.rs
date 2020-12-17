use crate::axis::Axis;

pub trait Axes {
    type Coordinate;
    fn index(&self, coordinate: &Self::Coordinate) -> usize;
}

struct Axes1D<X: Axis> {
    x: X,
}
struct Axes2D<X: Axis, Y: Axis> {
    x: X,
    y: Y,
}

impl<X: Axis> Axes for Axes1D<X> {
    type Coordinate = X::Coordinate;

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        self.x.index(coordinate)
    }
}

impl<X: Axis, Y: Axis> Axes for Axes2D<X, Y> {
    type Coordinate = (X::Coordinate, Y::Coordinate);

    fn index(&self, coordinate: &Self::Coordinate) -> usize {
        let ix = self.x.index(&coordinate.0);
        let iy = self.y.index(&coordinate.1);
        ix + self.x.size() * iy
    }
}
