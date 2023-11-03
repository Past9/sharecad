use super::{
    id::{GroupId, HSplitId, TabId, VSplitId},
    Config, GenericSplitId, Id,
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
        .clean()
    }

    pub fn remove_tab(&self, tab_id: TabId) -> Self {
        match self {
            Layout::Group(group) => Layout::Group(group.remove_tab(tab_id)),
            Layout::VSplit(split) => Layout::VSplit(split.remove_tab(tab_id)),
            Layout::HSplit(split) => Layout::HSplit(split.remove_tab(tab_id)),
        }
    }

    pub fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        match self {
            Layout::Group(group) => Layout::Group(group.set_active_tab_in_group(group_id, tab_id)),
            Layout::VSplit(split) => {
                Layout::VSplit(split.set_active_tab_in_group(group_id, tab_id))
            }
            Layout::HSplit(split) => {
                Layout::HSplit(split.set_active_tab_in_group(group_id, tab_id))
            }
        }
    }

    pub fn tab_exists(&self, tab_id: TabId) -> bool {
        match self {
            Layout::Group(group) => group.tab_exists(tab_id),
            Layout::VSplit(split) => split.tab_exists(tab_id),
            Layout::HSplit(split) => split.tab_exists(tab_id),
        }
    }

    pub fn group_exists(&self, group_id: GroupId) -> bool {
        match self {
            Layout::Group(group) => group.id == group_id,
            Layout::VSplit(split) => split.group_exists(group_id),
            Layout::HSplit(split) => split.group_exists(group_id),
        }
    }

    pub fn trim(&self) -> Self {
        let next_group_id = self.next_group_id();
        match self.remove_empty_groups_and_splits() {
            Some(layout) => layout,
            None => Layout::Group(Group {
                id: next_group_id,
                tabs: vec![],
            }),
        }
        .normalize_splits()
    }

    pub fn normalize_splits(&self) -> Self {
        match self {
            Layout::Group(group) => Layout::Group(group.clone()),
            Layout::VSplit(split) => Layout::VSplit(split.normalize_splits()),
            Layout::HSplit(split) => Layout::HSplit(split.normalize_splits()),
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        match self {
            Layout::Group(group) => {
                if group.tabs.len() > 0 {
                    Some(Self::Group(group.clone()))
                } else {
                    None
                }
            }
            Layout::VSplit(split) => split
                .remove_empty_groups_and_splits()
                .map(|split| Layout::VSplit(split)),
            Layout::HSplit(split) => split
                .remove_empty_groups_and_splits()
                .map(|split| Layout::HSplit(split)),
        }
    }

    pub fn activate_one_tab_per_group(&self) -> Self {
        match self {
            Layout::Group(group) => Layout::Group(group.activate_one_tab_per_group()),
            Layout::VSplit(split) => Layout::VSplit(split.activate_one_tab_per_group()),
            Layout::HSplit(split) => Layout::HSplit(split.activate_one_tab_per_group()),
        }
    }

    pub fn clean(&self) -> Self {
        self.trim().activate_one_tab_per_group()
    }

    pub fn next_group_id(&self) -> GroupId {
        self.highest_group_id().next()
    }

    pub fn highest_group_id(&self) -> GroupId {
        self.find_highest_group_id(GroupId::zero())
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        match self {
            Layout::Group(group) => cur_highest.max(group.id),
            Layout::VSplit(split) => split.find_highest_group_id(cur_highest),
            Layout::HSplit(split) => split.find_highest_group_id(cur_highest),
        }
    }

    pub fn next_tab_id(&self) -> TabId {
        self.highest_tab_id().next()
    }

    pub fn highest_tab_id(&self) -> TabId {
        self.find_highest_tab_id(TabId::zero())
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        match self {
            Layout::Group(group) => group.find_highest_tab_id(cur_highest),
            Layout::VSplit(split) => split.find_highest_tab_id(cur_highest),
            Layout::HSplit(split) => split.find_highest_tab_id(cur_highest),
        }
    }
}

pub trait OrientedSplitChild
where
    Self: Sized,
{
    fn remove_tab(&self, tab_id: TabId) -> Self;
    fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self;
    fn activate_one_tab_per_group(&self) -> Self;
    fn tab_exists(&self, tab_id: TabId) -> bool;
    fn group_exists(&self, group_id: GroupId) -> bool;
    fn normalize_splits(&self) -> Self;
    fn remove_empty_groups_and_splits(&self) -> Option<Self>;
    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId;
    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplitChild<T> {
    pub width: f64,
    pub child: T,
}
impl<T: OrientedSplitChild> SplitChild<T> {
    pub fn remove_tab(&self, tab_id: TabId) -> Self {
        Self {
            width: self.width,
            child: self.child.remove_tab(tab_id),
        }
    }

    pub fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        Self {
            width: self.width,
            child: self.child.set_active_tab_in_group(group_id, tab_id),
        }
    }

    pub fn activate_one_tab_per_group(&self) -> Self {
        Self {
            width: self.width,
            child: self.child.activate_one_tab_per_group(),
        }
    }

    pub fn tab_exists(&self, tab_id: TabId) -> bool {
        self.child.tab_exists(tab_id)
    }

    pub fn group_exists(&self, group_id: GroupId) -> bool {
        self.child.group_exists(group_id)
    }

    pub fn normalize_splits(&self) -> Self {
        Self {
            width: self.width,
            child: self.child.normalize_splits(),
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        self.child
            .remove_empty_groups_and_splits()
            .map(|child| Self {
                width: self.width,
                child,
            })
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        self.child.find_highest_group_id(cur_highest)
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        self.child.find_highest_tab_id(cur_highest)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VSplit {
    pub id: VSplitId,
    pub children: Vec<SplitChild<VSplitChild>>,
}
impl VSplit {
    pub fn remove_tab(&self, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.remove_tab(tab_id))
                .collect(),
        }
    }

    pub fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.set_active_tab_in_group(group_id, tab_id))
                .collect(),
        }
    }

    pub fn activate_one_tab_per_group(&self) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.activate_one_tab_per_group())
                .collect(),
        }
    }

    pub fn tab_exists(&self, tab_id: TabId) -> bool {
        self.children.iter().any(|child| child.tab_exists(tab_id))
    }

    pub fn group_exists(&self, group_id: GroupId) -> bool {
        self.children
            .iter()
            .any(|child| child.group_exists(group_id))
    }

    pub fn normalize_splits(&self) -> Self {
        let total_width: f64 = self.children.iter().map(|child| child.width).sum();

        if total_width > 0.0 {
            let scale = 1.0 / total_width;
            Self {
                id: self.id,
                children: self
                    .children
                    .iter()
                    .map(|child| SplitChild {
                        width: child.width * scale,
                        child: child.child.normalize_splits(),
                    })
                    .collect(),
            }
        } else {
            self.clone()
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        let children = self
            .children
            .iter()
            .filter_map(|child| child.remove_empty_groups_and_splits())
            .collect::<Vec<_>>();

        if children.len() > 0 {
            Some(Self {
                id: self.id,
                children,
            })
        } else {
            None
        }
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        self.children
            .iter()
            .map(|child| child.find_highest_group_id(cur_highest))
            .max()
            .unwrap_or(GroupId::zero())
            .max(cur_highest)
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        self.children
            .iter()
            .map(|child| child.find_highest_tab_id(cur_highest))
            .max()
            .unwrap_or(TabId::zero())
            .max(cur_highest)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VSplitChild {
    Group(Group),
    HSplit(HSplit),
}
impl OrientedSplitChild for VSplitChild {
    fn remove_tab(&self, tab_id: TabId) -> Self {
        match self {
            VSplitChild::Group(group) => Self::Group(group.remove_tab(tab_id)),
            VSplitChild::HSplit(split) => Self::HSplit(split.remove_tab(tab_id)),
        }
    }

    fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        match self {
            VSplitChild::Group(group) => {
                Self::Group(group.set_active_tab_in_group(group_id, tab_id))
            }
            VSplitChild::HSplit(split) => {
                Self::HSplit(split.set_active_tab_in_group(group_id, tab_id))
            }
        }
    }

    fn activate_one_tab_per_group(&self) -> Self {
        match self {
            VSplitChild::Group(group) => Self::Group(group.activate_one_tab_per_group()),
            VSplitChild::HSplit(split) => Self::HSplit(split.activate_one_tab_per_group()),
        }
    }

    fn tab_exists(&self, tab_id: TabId) -> bool {
        match self {
            VSplitChild::Group(group) => group.tab_exists(tab_id),
            VSplitChild::HSplit(split) => split.tab_exists(tab_id),
        }
    }

    fn group_exists(&self, group_id: GroupId) -> bool {
        match self {
            VSplitChild::Group(group) => group.id == group_id,
            VSplitChild::HSplit(split) => split.group_exists(group_id),
        }
    }

    fn normalize_splits(&self) -> Self {
        match self {
            VSplitChild::Group(group) => VSplitChild::Group(group.clone()),
            VSplitChild::HSplit(split) => VSplitChild::HSplit(split.normalize_splits()),
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        match self {
            VSplitChild::Group(group) => match group.tabs.len() > 0 {
                true => Some(VSplitChild::Group(group.clone())),
                false => None,
            },
            VSplitChild::HSplit(split) => split
                .remove_empty_groups_and_splits()
                .map(|split| VSplitChild::HSplit(split)),
        }
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        match self {
            VSplitChild::Group(group) => cur_highest.max(group.id),
            VSplitChild::HSplit(split) => split.find_highest_group_id(cur_highest),
        }
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        match self {
            VSplitChild::Group(group) => group.find_highest_tab_id(cur_highest),
            VSplitChild::HSplit(split) => split.find_highest_tab_id(cur_highest),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HSplit {
    pub id: HSplitId,
    pub children: Vec<SplitChild<HSplitChild>>,
}
impl HSplit {
    pub fn remove_tab(&self, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.remove_tab(tab_id))
                .collect(),
        }
    }

    pub fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.set_active_tab_in_group(group_id, tab_id))
                .collect(),
        }
    }

    pub fn activate_one_tab_per_group(&self) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.activate_one_tab_per_group())
                .collect(),
        }
    }

    pub fn tab_exists(&self, tab_id: TabId) -> bool {
        self.children.iter().any(|child| child.tab_exists(tab_id))
    }

    pub fn group_exists(&self, group_id: GroupId) -> bool {
        self.children
            .iter()
            .any(|child| child.group_exists(group_id))
    }

    pub fn normalize_splits(&self) -> Self {
        let total_width: f64 = self.children.iter().map(|child| child.width).sum();

        if total_width > 0.0 {
            let scale = 1.0 / total_width;
            Self {
                id: self.id,
                children: self
                    .children
                    .iter()
                    .map(|child| SplitChild {
                        width: child.width * scale,
                        child: child.child.normalize_splits(),
                    })
                    .collect(),
            }
        } else {
            self.clone()
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        let children = self
            .children
            .iter()
            .filter_map(|child| child.remove_empty_groups_and_splits())
            .collect::<Vec<_>>();

        if children.len() > 0 {
            Some(Self {
                id: self.id,
                children,
            })
        } else {
            None
        }
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        self.children
            .iter()
            .map(|child| child.find_highest_group_id(cur_highest))
            .max()
            .unwrap_or(GroupId::zero())
            .max(cur_highest)
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        self.children
            .iter()
            .map(|child| child.find_highest_tab_id(cur_highest))
            .max()
            .unwrap_or(TabId::zero())
            .max(cur_highest)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HSplitChild {
    Group(Group),
    VSplit(VSplit),
}
impl OrientedSplitChild for HSplitChild {
    fn remove_tab(&self, tab_id: TabId) -> Self {
        match self {
            HSplitChild::Group(group) => Self::Group(group.remove_tab(tab_id)),
            HSplitChild::VSplit(split) => Self::VSplit(split.remove_tab(tab_id)),
        }
    }

    fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        match self {
            HSplitChild::Group(group) => {
                Self::Group(group.set_active_tab_in_group(group_id, tab_id))
            }
            HSplitChild::VSplit(split) => {
                Self::VSplit(split.set_active_tab_in_group(group_id, tab_id))
            }
        }
    }

    fn activate_one_tab_per_group(&self) -> Self {
        match self {
            HSplitChild::Group(group) => Self::Group(group.activate_one_tab_per_group()),
            HSplitChild::VSplit(split) => Self::VSplit(split.activate_one_tab_per_group()),
        }
    }

    fn tab_exists(&self, tab_id: TabId) -> bool {
        match self {
            HSplitChild::Group(group) => group.tab_exists(tab_id),
            HSplitChild::VSplit(split) => split.tab_exists(tab_id),
        }
    }

    fn group_exists(&self, group_id: GroupId) -> bool {
        match self {
            HSplitChild::Group(group) => group.id == group_id,
            HSplitChild::VSplit(split) => split.group_exists(group_id),
        }
    }

    fn normalize_splits(&self) -> Self {
        match self {
            HSplitChild::Group(group) => HSplitChild::Group(group.clone()),
            HSplitChild::VSplit(split) => HSplitChild::VSplit(split.normalize_splits()),
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        match self {
            HSplitChild::Group(group) => match group.tabs.len() > 0 {
                true => Some(HSplitChild::Group(group.clone())),
                false => None,
            },
            HSplitChild::VSplit(split) => split
                .remove_empty_groups_and_splits()
                .map(|split| HSplitChild::VSplit(split)),
        }
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        match self {
            HSplitChild::Group(group) => cur_highest.max(group.id),
            HSplitChild::VSplit(split) => split.find_highest_group_id(cur_highest),
        }
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        match self {
            HSplitChild::Group(group) => group.find_highest_tab_id(cur_highest),
            HSplitChild::VSplit(split) => split.find_highest_tab_id(cur_highest),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub id: GroupId,
    pub tabs: Vec<Tab>,
}
impl Group {
    pub fn remove_tab(&self, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            tabs: self
                .tabs
                .iter()
                .filter(|tab| tab.id != tab_id)
                .cloned()
                .collect(),
        }
    }

    pub fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            tabs: self
                .tabs
                .iter()
                .map(|tab| {
                    let mut new_tab = tab.clone();
                    if group_id == self.id {
                        new_tab.active_in_group = tab_id == tab.id;
                    }
                    new_tab
                })
                .collect(),
        }
    }

    pub fn activate_one_tab_per_group(&self) -> Self {
        let active_tabs = self
            .tabs
            .iter()
            .enumerate()
            .filter_map(|(index, tab)| {
                if tab.active_in_group {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let active_tab_index = if active_tabs.len() == 0 {
            0
        } else {
            active_tabs[0]
        };

        Self {
            id: self.id,
            tabs: self
                .tabs
                .iter()
                .enumerate()
                .map(|(i, tab)| {
                    let mut new_tab = tab.clone();
                    new_tab.active_in_group = i == active_tab_index;
                    new_tab
                })
                .collect(),
        }
    }

    pub fn tab_exists(&self, tab_id: TabId) -> bool {
        self.tabs.iter().any(|tab| tab.id == tab_id)
    }

    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId {
        self.tabs
            .iter()
            .map(|tab| tab.id)
            .max()
            .unwrap_or(TabId::zero())
            .max(cur_highest)
    }
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
