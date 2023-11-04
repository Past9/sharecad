use std::{cell::RefCell, rc::Rc};

use super::SplitOrientation;

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

#[derive(PartialEq, Debug, Clone)]
pub struct GenericSplitId(u32, SplitOrientation);
impl GenericSplitId {
    pub fn as_hsplit_id(&self) -> HSplitId {
        match self.1 {
            SplitOrientation::Horizontal => HSplitId(self.0),
            SplitOrientation::Vertical => {
                panic!("Tried to get HSplitId from GenericSplitId with vertical orientation")
            }
        }
    }

    pub fn as_vsplit_id(&self) -> VSplitId {
        match self.1 {
            SplitOrientation::Vertical => VSplitId(self.0),
            SplitOrientation::Horizontal => {
                panic!("Tried to get VSplitId from GenericSplitId with horizontal orientation")
            }
        }
    }
}
impl From<VSplitId> for GenericSplitId {
    fn from(value: VSplitId) -> Self {
        Self(value.0, SplitOrientation::Vertical)
    }
}
impl From<HSplitId> for GenericSplitId {
    fn from(value: HSplitId) -> Self {
        Self(value.0, SplitOrientation::Horizontal)
    }
}

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
