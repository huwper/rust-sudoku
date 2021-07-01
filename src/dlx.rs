/// dancing links for algorithm X ;) 


use super::quad_link;


struct Head {
    pub num_entries: u32,
}

enum ListEntry {
    Head(Head),
    Node(bool)
}

pub struct DlxSolver {
    list: Option<quad_link::EntryRef<ListEntry>>
}

fn create_header_list_r(remaining: u32, prev: &mut Option<quad_link::EntryRef<ListEntry>>) {
    if remaining > 0 {
        let mut next = Some(quad_link::new_list_entry(ListEntry::Head(Head{num_entries: 0})));
        quad_link::link_left_right(prev, &mut next).unwrap();
        create_header_list_r(remaining - 1, &mut next);
    }
}

impl DlxSolver {
    pub fn new(n_cols: u32, n_rows: u32, col_row_entries: &[(u32, u32)]) -> Self {
        let mut ret = Self {
            list: Some(quad_link::new_list_entry(ListEntry::Head(Head{num_entries: 0})))
        };

        let mut prev = &mut ret.list;
        create_header_list_r(n_cols - 1, &mut prev);

        ret
    }
}

