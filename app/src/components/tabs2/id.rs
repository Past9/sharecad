use std::{cell::RefCell, rc::Rc};

pub trait Id: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug {
    fn zero() -> Self;
    fn next(&self) -> Self;
}

macro_rules! id_type {
    ($type_name:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $type_name(u32);
        impl Id for $type_name {
            fn zero() -> Self {
                Self(0)
            }

            fn next(&self) -> Self {
                Self(self.0 + 1)
            }
        }
    };
}

id_type!(GroupId);
id_type!(VSplitId);
id_type!(HSplitId);
id_type!(TabId);

trait IdTraits: From<u32> + Into<u32> {}

#[derive(Clone)]
pub struct IdSource<T: Id> {
    current: Rc<RefCell<T>>,
}
impl<T: Id> IdSource<T> {
    pub fn new() -> Self {
        Self {
            current: Rc::new(RefCell::new(T::zero())),
        }
    }

    pub fn next(&self) -> T {
        let mut current = self.current.borrow_mut();
        let next = current.next();
        *current = next;
        next
    }
}
