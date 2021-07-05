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

pub trait QuadLinkList: Sized + Clone {
    type Item;
    fn left(&self) -> Option<Self>;
    fn set_left(&self, new: Option<Self>);

    fn right(&self) -> Option<Self>;
    fn set_right(&self, new: Option<Self>);

    fn up(&self) -> Option<Self>;
    fn set_up(&self, new: Option<Self>);

    fn down(&self) -> Option<Self>;
    fn set_down(&self, new: Option<Self>);

    fn item(&self) -> Self::Item;
    fn set_item(&self, item: Self::Item);

    fn right_iter(&self) -> QuadLinkListIter<Self>;
}

enum IterDirection {
    Left,
    Right,
    Up,
    Down
}

pub struct QuadLinkListIter<T>
where T: QuadLinkList
{
    direction: IterDirection,
    current: Option<T>,
}

impl<T> Iterator for QuadLinkListIter<T>
where T: QuadLinkList
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current.clone();
        if let Some(current) = (&self.current).as_ref() {
            self.current = match self.direction {
                IterDirection::Left => current.left(),
                IterDirection::Right => current.right(),
                IterDirection::Up => current.up(),
                IterDirection::Down => current.down(),
            };
        }
        ret
    }
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

    fn item(&self) -> Self::Item {
        (*self.this_entry).borrow().item
    }

    fn set_item(&self, item: Self::Item) {
        (*self.this_entry).borrow_mut().item = item;
    }

    fn left(&self) -> Option<Self> {
        match &(*self.this_entry).borrow().left {
            Some(left) => Some(left.clone()),
            None => None,
        }
    }

    fn set_left(&self, new: Option<Self>) {
        if let Some(uw_new) = new {
            (*self.this_entry).borrow_mut().left = Some(uw_new.clone());
        } else {
            (*self.this_entry).borrow_mut().left = None;
        }
    }

    fn right(&self) -> Option<Self> {
        match &(*self.this_entry).borrow().right {
            Some(right) => Some(right.clone()),
            None => None,
        }
    }

    fn set_right(&self, new: Option<Self>) {
        if let Some(uw_new) = new {
            (*self.this_entry).borrow_mut().right = Some(uw_new.clone());
        } else {
            (*self.this_entry).borrow_mut().right = None;
        }
    }

    fn up(&self) -> Option<Self> {
        match &(*self.this_entry).borrow().up {
            Some(up) => Some(up.clone()),
            None => None,
        }
    }

    fn set_up(&self, new: Option<Self>) {
        if let Some(uw_new) = new {
            (*self.this_entry).borrow_mut().up = Some(uw_new.clone());
        } else {
            (*self.this_entry).borrow_mut().up = None;
        }
    }

    fn down(&self) -> Option<Self> {
        match &(*self.this_entry).borrow().down {
            Some(down) => Some(down.clone()),
            None => None,
        }
    }

    fn set_down(&self, new: Option<Self>) {
        if let Some(uw_new) = new {
            (*self.this_entry).borrow_mut().down = Some(uw_new.clone());
        } else {
            (*self.this_entry).borrow_mut().down = None;
        }
    }

    fn right_iter(&self) -> QuadLinkListIter<Self> {
        QuadLinkListIter {
            direction: IterDirection::Right,
            current: Some(self.clone())
        }
    }
}

pub fn link_up_down<T>(up: &T, down: &T)
where
    T: QuadLinkList,
{
    up.set_down(Some(down.clone()));
    down.set_up(Some(up.clone()));
}

pub fn link_left_right<T>(left: &T, right: &T)
where
    T: QuadLinkList,
{
    left.set_right(Some(right.clone()));
    right.set_left(Some(left.clone()));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_circular_list() {
        let a = QuadLinks::new('a');
        let b = QuadLinks::new('b');
        let c = QuadLinks::new('c');
        let d = QuadLinks::new('d');

        link_up_down(&b, &a);
        link_left_right(&b, &c);
        link_up_down(&c, &d);
        link_left_right(&a, &d);

        //   |     |
        // - b <-> c -
        //   ^     ^
        //   |     |
        //   v     v
        // - a <-> d -
        //   |     |

        assert_eq!('d', a.right().unwrap().item());

        assert_eq!('c', a.right().unwrap().up().unwrap().item());
        assert_eq!('b', a.right().unwrap().up().unwrap().left().unwrap().item());
        assert_eq!(
            'a',
            a.right()
                .unwrap()
                .up()
                .unwrap()
                .left()
                .unwrap()
                .down()
                .unwrap()
                .item()
        );
    }
}
