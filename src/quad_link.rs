use std::{cell::RefCell, rc::Rc};

type EntryRef<T> = Rc<RefCell<CircularListEntry<T>>>;

pub struct CircularListEntry<T>
where
    T: Sized + Copy,
{
    pub item: T,
    pub left: Option<QuadLinks<T>>,
    pub right: Option<QuadLinks<T>>,
    pub up: Option<QuadLinks<T>>,
    pub down: Option<QuadLinks<T>>,
}

#[derive(Clone)]
pub struct QuadLinks<T>
where
    T: Sized + Copy,
{
    this_entry: EntryRef<T>,
}

trait QuadLinkList: Sized {
    type Item;
    fn left(&self) -> Option<Self>;
    fn set_left(&self, new: Option<Self>);
    fn item(&self) -> Self::Item;
    fn set_item(&self, item: Self::Item);
}

trait QuadLinkBorrowedItem {}

impl<T> QuadLinks<T>
where
    T: Sized + Copy,
{
    pub fn new(item: T) -> Self {
        QuadLinks {
            this_entry: Rc::new(RefCell::new(CircularListEntry {
                item: item,
                left: None,
                right: None,
                up: None,
                down: None,
            })),
        }
    }
}

impl<T> QuadLinkList for QuadLinks<T>
where
    T: Sized + Copy,
{
    type Item = T;

    fn left(&self) -> Option<Self> {
        match &(*self.this_entry).borrow().left {
            Some(left) => Some(left.clone()),
            None => None,
        }
    }

    fn set_left(&self, new: Option<Self>) {
        if let Some(uw_new) = new {
            (*self.this_entry).borrow_mut().left = Some(uw_new.clone());
            (*uw_new.this_entry).borrow_mut().right = Some(self.clone());
        } else {
            (*self.this_entry).borrow_mut().left = None;
        }
    }

    fn item(&self) -> Self::Item {
        (*self.this_entry).borrow().item
    }

    fn set_item(&self, item: Self::Item) {
        (*self.this_entry).borrow_mut().item = item;
    }
}

// pub fn new_list_entry<T>(val: T) -> EntryRef<T> {
//     Rc::new(RefCell::new(CircularListEntry {
//         item: val,
//         left: None,
//         right: None,
//         up: None,
//         down: None,
//     }))
// }

// pub fn link_left_right<T>(left: &EntryRef<T>, right: &EntryRef<T>) {
//     (*left).borrow_mut().right = Some(right.clone());
//     (*right).borrow_mut().left = Some(left.clone());
// }

// pub fn _link_up_down<T>(
//     up: &mut Option<EntryRef<T>>,
//     down: &mut Option<EntryRef<T>>,
// ) -> Result<(), ()> {
//     if let (Some(u), Some(d)) = (up, down) {
//         (*u).borrow_mut().right = Some(d.clone());
//         (*d).borrow_mut().left = Some(u.clone());
//         Ok(())
//     } else {
//         Err(())
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_circular_list() {
        let a = QuadLinks::new(5);
        let b = QuadLinks::new(10);

        a.set_left(Some(b.clone()));
        assert_eq!(b.item(), a.left().unwrap().item());

        b.set_item(1000);
        assert_eq!(1000, b.item());
        assert_eq!(b.item(), a.left().unwrap().item());

        a.left().unwrap().set_item(1);
        assert_eq!(1, b.item());
    }
}
