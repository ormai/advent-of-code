pub mod solution;

/// The eight ordinal directions
#[rustfmt::skip]
pub const DIRECTIONS: [(isize, isize); 8] = [
    (-1,  1), (0,  1), (1,  1),
    (-1,  0),          (1,  0),
    (-1, -1), (0, -1), (1, -1),
];

/// The four cardinal directions. The first is North and the other follow in clockwise order.
pub const ORTHOGONAL_DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
