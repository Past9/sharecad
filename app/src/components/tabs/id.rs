use super::SplitOrientation;
use dioxus::core::AttributeValue;
use std::{cell::RefCell, hash::Hash, rc::Rc};

pub trait Id: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + Hash {
    fn zero() -> Self;
    fn next(&self) -> Self;
    fn as_attr_val(&self) -> AttributeValue;
}

macro_rules! id_type {
    ($type_name:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type_name(u32);
        impl $type_name {
            pub fn new(id: u32) -> Self {
                Self(id)
            }

            pub fn num(&self) -> u32 {
                self.0
            }
        }
        impl Id for $type_name {
            fn zero() -> Self {
                Self(0)
            }

            fn next(&self) -> Self {
                Self(self.0 + 1)
            }

            fn as_attr_val(&self) -> AttributeValue {
                AttributeValue::Int(self.0 as i64)
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
