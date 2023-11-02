use super::id::{GroupId, HSplitId, TabId, VSplitId};

#[derive(Clone, Debug)]
pub enum Layout {
    Group(Group),
    VSplit(VSplit),
    HSplit(HSplit),
}

#[derive(Clone, Debug)]
pub struct VSplit {
    pub id: VSplitId,
    pub children: Vec<VSplitChild>,
}

#[derive(Clone, Debug)]
pub enum VSplitChild {
    Group(Group),
    HSplit(HSplit),
}

#[derive(Clone, Debug)]
pub struct HSplit {
    pub id: HSplitId,
    pub children: Vec<HSplitChild>,
}

#[derive(Clone, Debug)]
pub enum HSplitChild {
    Group(Group),
    VSplit(VSplit),
}

#[derive(Clone, Debug)]
pub struct Group {
    pub id: GroupId,
    pub tabs: Vec<Tab>,
}

#[derive(Clone, Debug)]
pub struct Tab {
    pub id: TabId,
    pub title: String,
}
