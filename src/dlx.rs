/// dancing links for algorithm X ;)
use super::quad_link;

struct Head {
    pub num_entries: u32,
}

enum ListEntry {
    Head(Head),
    Node(bool),
}

pub struct DlxSolver {
    list: Option<quad_link::EntryRef<ListEntry>>,
}

impl DlxSolver {
    pub fn new(n_cols: u32, _n_rows: u32, _col_row_entries: &[(u32, u32)]) -> Self {
        let temp = quad_link::new_list_entry(ListEntry::Head(Head { num_entries: 0 }));
        let mut idx = temp.clone();
        //let mut next;
        for _ in 0..n_cols {
            let next = quad_link::new_list_entry(ListEntry::Head(Head { num_entries: 0 }));
            {
                quad_link::link_left_right(&idx, &next);
            }
            idx = next.clone();
        }
        let ret = Self { list: Some(temp) };

        ret
    }
}
