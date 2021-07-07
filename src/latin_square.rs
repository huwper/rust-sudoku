use std::{char::MAX, iter::{Chain, Map}, ops::Range};

/// solve a 2x2 latin square
/// 4 squares
/// each row must contain each number
/// each colum must contain each number
/// coordinates
/// (0,0)  (1,0)
/// (1,0)  (1,1)

///constraints

struct RowEntry {
    number: usize,
    location: (usize, usize),
}

const ROW_LENGTH: usize = 2;
const MAX_VAL: usize = ROW_LENGTH.pow(2);


trait Constraint {
    fn check(&self, l: RowEntry) -> bool;
}

#[derive(Clone, Copy)]
struct RowConstraint {
    n: usize,
}

impl Constraint for RowConstraint {
    fn check(&self, l: RowEntry) -> bool {
        l.location.1 == self.n
    }
}

#[derive(Clone, Copy)]
struct ColConstraint {
    n: usize,
}

impl Constraint for ColConstraint {
    fn check(&self, l: RowEntry) -> bool {
        l.location.0 == self.n
    }
}

pub fn demo() {
    let all_location_iter = (0..ROW_LENGTH).flat_map(move |r| {
        (0..ROW_LENGTH).flat_map(move |c| {
            (0..MAX_VAL).map(move |v| RowEntry {
                number: v,
                location: (c, r),
            })
        })
    });

    let row_constraints = (0..ROW_LENGTH).map(move |n| Box::new(RowConstraint{n: n}));
    let col_constraints = (0..ROW_LENGTH).map(move |n| Box::new(ColConstraint{n: n}) );

    let dlx_table: Vec<Vec<bool>> = all_location_iter.map(|location| all_constraints.map(|c| c.check(location)).collect()).collect();
}
