use std::{cell::RefCell, rc::Rc};

pub type EntryRef<T> = Rc<RefCell<CircularListEntry<T>>>;

pub struct CircularListEntry<T> {
    pub item: T,
    pub left: Option<EntryRef<T>>,
    pub right: Option<EntryRef<T>>,
    pub up: Option<EntryRef<T>>,
    pub down: Option<EntryRef<T>>,
}

pub fn new_list_entry<T>(val: T) -> EntryRef<T> {
    Rc::new(RefCell::new(CircularListEntry {
        item: val,
        left: None,
        right: None,
        up: None,
        down: None,
    }))
}

pub fn link_left_right<T>(left: &EntryRef<T>, right: &EntryRef<T>) {
    (*left).borrow_mut().right = Some(right.clone());
    (*right).borrow_mut().left = Some(left.clone());
}

pub fn _link_up_down<T>(
    up: &mut Option<EntryRef<T>>,
    down: &mut Option<EntryRef<T>>,
) -> Result<(), ()> {
    if let (Some(u), Some(d)) = (up, down) {
        (*u).borrow_mut().right = Some(d.clone());
        (*d).borrow_mut().left = Some(u.clone());
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_circular_list() {
        let a = new_list_entry(5);
        let b = new_list_entry(6);

        link_left_right(&a, &b);

        assert_eq!(6, (*(*a).borrow().right.as_ref().unwrap()).borrow().item);
    }
}
