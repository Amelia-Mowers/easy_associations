use std::rc::{Rc, Weak};
use std::cell::RefCell;
// use std::fmt;
use easy_associations::*;

// #[derive(Debug)]
pub struct One {
    me: Weak<RefCell<One>>,
    many_collection: Vec<Rc<RefCell<Many>>>
}

impl One {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(
                Self {
                    me: me.clone(),
                    many_collection: Vec::new()
                }
            )
        })
    }
}

pub struct Many {
    me: Weak<RefCell<Many>>,
    one_ref: Option<Rc<RefCell<One>>>
}

impl Many {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(
                Self {
                    me: me.clone(),
                    one_ref: None
                }
            )
        })
    }
}

// impl fmt::Debug for Many {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut f = f.debug_struct("ChildSelfRef");
//         let f = f.field("me", &self.me);
//         let mut f = f.field("name", &self.name);
//
//         if let Some(p) = &self.many {
//             f = f.field("parent", &p.borrow().name)
//         } else {
//             f = f.field("parent", &"none")
//         }
//
//         f.finish()
//     }
// }

bidirectional_one_to_many!(
    One, 
    me,
    many_collection, 
    add_many,
    remove_many,
    Many, 
    me,
    one_ref,
    set_one
);

#[cfg(test)]
mod tests {
    use crate::{One, Many};
    use std::rc::Rc;

    #[test]
    fn one_add_many_test() {
        let one = One::new();
        let many = Many::new();

        one.borrow_mut().add_many(many.clone());

        {
            assert!(
                one
                .borrow()
                .many_collection.iter()
                .any(|x| Rc::ptr_eq(&x, &many))
            );

            assert!(
                Rc::ptr_eq(
                    many.borrow().one_ref.as_ref().unwrap(), 
                    &one
                )
            );
        }
    }

    #[test]
    fn one_remove_many_test() {
        let one = One::new();
        let many = Many::new();

        one.borrow_mut().add_many(many.clone());
        one.borrow_mut().remove_many(many.clone());

        {
            assert!(
                !one
                .borrow()
                .many_collection.iter()
                .any(|x| Rc::ptr_eq(&x, &many))
            );

            assert!(
                many
                .borrow()
                .one_ref
                .is_none()
            );
        }
    }

    #[test]
    fn many_set_one_test_some_one() {
        let one = One::new();
        let many = Many::new();

        many.borrow_mut().set_one(Some(one.clone()));

        {
            assert!(
                one
                .borrow()
                .many_collection.iter()
                .any(|x| Rc::ptr_eq(&x, &many))
            );

            assert!(
                Rc::ptr_eq(
                    many.borrow().one_ref.as_ref().unwrap(), 
                    &one
                )
            );
        }
    }

    #[test]
    fn many_set_one_test_none() {
        let one = One::new();
        let many = Many::new();

        many.borrow_mut().set_one(Some(one.clone()));
        many.borrow_mut().set_one(None);

        {
            assert!(
                !one
                .borrow()
                .many_collection.iter()
                .any(|x| Rc::ptr_eq(&x, &many))
            );

            assert!(
                many
                .borrow()
                .one_ref
                .is_none()
            );
        }
    }
}