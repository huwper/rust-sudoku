use std::char::MAX;


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
    location: (usize, usize)
}


const ROW_LENGTH: usize = 2;
const MAX_VAL: usize = ROW_LENGTH.pow(2);

pub fn demo() {

    let all_location_iter = (0..ROW_LENGTH).flat_map(
        move |r| (0..ROW_LENGTH).flat_map(
            move |c| (0..MAX_VAL).map(
                move |v| RowEntry {
                    number: v,
                    location: (c, r)
                }
            )
        )
    );


}