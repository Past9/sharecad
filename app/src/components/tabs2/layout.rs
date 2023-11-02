use super::{
    id::{GroupId, HSplitId, TabId, VSplitId},
    Config,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Layout {
    Group(Group),
    VSplit(VSplit),
    HSplit(HSplit),
}
impl Layout {
    pub fn as_new_config(self) -> Config {
        Config {
            dragging_tab: None,
            drop_tab_offer: None,
            layout: self,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VSplit {
    pub id: VSplitId,
    pub children: Vec<VSplitChild>,
    pub splits: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VSplitChild {
    Group(Group),
    HSplit(HSplit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HSplit {
    pub id: HSplitId,
    pub children: Vec<HSplitChild>,
    pub splits: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HSplitChild {
    Group(Group),
    VSplit(VSplit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub id: GroupId,
    pub tabs: Vec<Tab>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tab {
    pub id: TabId,
    pub title: String,
}
