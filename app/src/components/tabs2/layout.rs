use super::{
    id::{GroupId, HSplitId, TabId, VSplitId},
    Config, GenericSplitId,
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
    pub children: Vec<SplitChild<VSplitChild>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplitChild<T> {
    pub width: f64,
    pub child: T,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VSplitChild {
    Group(Group),
    HSplit(HSplit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HSplit {
    pub id: HSplitId,
    pub children: Vec<SplitChild<HSplitChild>>,
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
    pub active_in_group: bool,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericLayout {
    Group(Group),
    Split(GenericSplit),
}
impl From<Layout> for GenericLayout {
    fn from(layout: Layout) -> Self {
        match layout {
            Layout::Group(group) => Self::Group(group),
            Layout::VSplit(vsplit) => Self::Split(vsplit.into()),
            Layout::HSplit(hsplit) => Self::Split(hsplit.into()),
        }
    }
}
impl From<VSplitChild> for GenericLayout {
    fn from(value: VSplitChild) -> Self {
        match value {
            VSplitChild::Group(group) => Self::Group(group.clone()),
            VSplitChild::HSplit(hsplit) => Self::Split(hsplit.into()),
        }
    }
}
impl From<HSplitChild> for GenericLayout {
    fn from(value: HSplitChild) -> Self {
        match value {
            HSplitChild::Group(group) => Self::Group(group.clone()),
            HSplitChild::VSplit(vsplit) => Self::Split(vsplit.into()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum SplitOrientation {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Debug, Clone)]
pub struct GenericSplit {
    pub id: GenericSplitId,
    pub orientation: SplitOrientation,
    pub children: Vec<SplitChild<GenericLayout>>,
}
impl From<VSplit> for GenericSplit {
    fn from(value: VSplit) -> Self {
        GenericSplit {
            id: value.id.into(),
            orientation: SplitOrientation::Vertical,
            children: value
                .children
                .into_iter()
                .map(|c| SplitChild {
                    width: c.width,
                    child: c.child.into(),
                })
                .collect(),
        }
    }
}
impl From<HSplit> for GenericSplit {
    fn from(value: HSplit) -> Self {
        GenericSplit {
            id: value.id.into(),
            orientation: SplitOrientation::Horizontal,
            children: value
                .children
                .into_iter()
                .map(|c| SplitChild {
                    width: c.width,
                    child: c.child.into(),
                })
                .collect(),
        }
    }
}
