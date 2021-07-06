// /// dancing links for algorithm X ;)
use super::quad_link::{self, QuadLinkList, QuadLinksFactory};

#[derive(Debug, Clone, Copy)]
struct Head {
    pub num_entries: u32,
}

impl Head {
    fn new() -> Self {
        Head { num_entries: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
enum ListEntry {
    Head(Head),
    Node(bool),
}

pub struct DlxSolver {
    list_factory: QuadLinksFactory<ListEntry>,
    list: Option<quad_link::QuadLinks<ListEntry>>,
}

impl DlxSolver {
    pub fn new(n_cols: u32, _n_rows: u32, _col_row_entries: &[(u32, u32)]) -> Self {
        let mut ret = Self {
            list_factory: QuadLinksFactory::new(),
            list: None
        };
        let temp = ret.list_factory.create(ListEntry::Head(Head::new()));
        let mut idx = temp.clone();
        //let mut next;
        for _ in 1..n_cols {
            let next = ret.list_factory.create(ListEntry::Head(Head::new()));
            {
                quad_link::link_left_right(&idx, &next);
            }
            idx = next.clone();
        }

        ret.list = Some(temp);
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new() {
        let algo = DlxSolver::new(10, 10, &[]);
        let mut list = algo.list.clone();
        let mut num_headers = 0;

        loop {
            num_headers += 1;
            match (&list).as_ref().unwrap().right() {
                Some(r) => list = Some(r.clone()),
                None => break,
            }
        }

        assert_eq!(10, num_headers);
        println!("{:?}", (&algo.list).as_ref().unwrap().right_iter().nth(5).unwrap().item());
    }
}
