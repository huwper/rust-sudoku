use std::{cell::RefCell, rc::Rc};


pub struct CircularListEntry<T> {
    pub item: T,
    pub prev: Option<Rc<RefCell<CircularListEntry<T>>>>,
    pub next: Option<Rc<RefCell<CircularListEntry<T>>>>
}

pub struct CircularList<T> {
    pub head: Rc<RefCell<CircularListEntry<T>>>
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_circular_list() {
        let list = CircularList {
            head: Rc::new(RefCell::new(CircularListEntry {
                item: 5,
                prev: None,
                next: None
            }))
        };

        (*list.head).borrow_mut().next = Some(Rc::new(RefCell::new(CircularListEntry {
            item: 6,
            prev: None,
            next: None
        })));
        {
            let first = list.head.borrow();
            let second = first.next.as_ref().unwrap();
            let mut second_entry = (**second).borrow_mut();
            second_entry.prev = Some(Rc::clone(&list.head));
        }

        let first = list.head.borrow();
        assert_eq!(6, (*first.next.as_ref().unwrap()).borrow().item);
        assert_eq!(5, (*(*first.next.as_ref().unwrap()).borrow().prev.as_ref().unwrap()).borrow().item);
    }
}
