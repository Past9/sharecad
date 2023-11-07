use super::{
    id::{GroupId, HSplitId, TabId, VSplitId},
    Config, GenericSplitId, Id, SplitDirection,
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

    pub fn split(&self, group_id: GroupId, direction: SplitDirection, tab: &Tab) -> Self {
        let next_group_id = self.next_group_id();
        let next_vsplit_id = self.next_vsplit_id();
        let next_hsplit_id = self.next_hsplit_id();
        self.do_split(
            group_id,
            direction,
            tab,
            next_group_id,
            next_vsplit_id,
            next_hsplit_id,
        )
    }

    fn do_split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Self {
        match self {
            Layout::Group(group) => group.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
            Layout::VSplit(split) => split.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
            Layout::HSplit(split) => split.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
        }
    }

    pub fn create_new_tab(&self, group_id: GroupId, index: usize, title: &str) -> Self {
        let next_tab_id = self.next_tab_id();
        match self {
            Layout::Group(group) => group
                .create_new_tab(group_id, index, title, next_tab_id)
                .into(),
            Layout::VSplit(split) => split
                .create_new_tab(group_id, index, title, next_tab_id)
                .into(),
            Layout::HSplit(split) => split
                .create_new_tab(group_id, index, title, next_tab_id)
                .into(),
        }
    }

    pub fn find_focused_tab(&self) -> Option<TabId> {
        match self {
            Layout::Group(group) => group.find_focused_tab(),
            Layout::VSplit(split) => split.find_focused_tab(),
            Layout::HSplit(split) => split.find_focused_tab(),
        }
    }

    pub fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        match self {
            Layout::Group(group) => group.find_tab_group(tab_id),
            Layout::VSplit(split) => split.find_tab_group(tab_id),
            Layout::HSplit(split) => split.find_tab_group(tab_id),
        }
    }

    pub fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        match self {
            Layout::Group(group) => group.find_tab_index(tab_id),
            Layout::VSplit(split) => split.find_tab_index(tab_id),
            Layout::HSplit(split) => split.find_tab_index(tab_id),
        }
    }

    pub fn focus_tab(&self, tab_id: TabId) -> Self {
        match self {
            Layout::Group(group) => group.focus_tab(tab_id).into(),
            Layout::VSplit(split) => split.focus_tab(tab_id).into(),
            Layout::HSplit(split) => split.focus_tab(tab_id).into(),
        }
    }

    pub fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        match self {
            Self::Group(group) => group.get_tab(tab_id),
            Self::VSplit(split) => split.get_tab(tab_id),
            Self::HSplit(split) => split.get_tab(tab_id),
        }
    }

    pub fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.insert_tab(group_id, index, tab)),
            Self::VSplit(split) => Self::VSplit(split.insert_tab(group_id, index, tab)),
            Self::HSplit(split) => Self::HSplit(split.insert_tab(group_id, index, tab)),
        }
    }

    pub fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.clone()),
            Self::VSplit(split) => {
                Self::VSplit(split.adjust_vsplit(vsplit_id, index, new_location))
            }
            Self::HSplit(split) => {
                Self::HSplit(split.adjust_vsplit(vsplit_id, index, new_location))
            }
        }
    }

    pub fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.clone()),
            Self::VSplit(split) => {
                Self::VSplit(split.adjust_hsplit(hsplit_id, index, new_location))
            }
            Self::HSplit(split) => {
                Self::HSplit(split.adjust_hsplit(hsplit_id, index, new_location))
            }
        }
    }

    pub fn remove_tab(&self, tab_id: TabId) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.remove_tab(tab_id)),
            Self::VSplit(split) => Self::VSplit(split.remove_tab(tab_id)),
            Self::HSplit(split) => Self::HSplit(split.remove_tab(tab_id)),
        }
    }

    pub fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.set_active_tab_in_group(group_id, tab_id)),
            Self::VSplit(split) => Self::VSplit(split.set_active_tab_in_group(group_id, tab_id)),
            Self::HSplit(split) => Self::HSplit(split.set_active_tab_in_group(group_id, tab_id)),
        }
    }

    pub fn tab_exists(&self, tab_id: TabId) -> bool {
        match self {
            Self::Group(group) => group.tab_exists(tab_id),
            Self::VSplit(split) => split.tab_exists(tab_id),
            Self::HSplit(split) => split.tab_exists(tab_id),
        }
    }

    pub fn group_exists(&self, group_id: GroupId) -> bool {
        match self {
            Self::Group(group) => group.id == group_id,
            Self::VSplit(split) => split.group_exists(group_id),
            Self::HSplit(split) => split.group_exists(group_id),
        }
    }

    pub fn trim(&self) -> Self {
        let next_group_id = self.next_group_id();
        match self.remove_empty_groups_and_splits() {
            Some(layout) => layout,
            None => Self::Group(Group {
                id: next_group_id,
                tabs: vec![],
            }),
        }
        .normalize_splits()
    }

    pub fn normalize_splits(&self) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.clone()),
            Self::VSplit(split) => Self::VSplit(split.normalize_splits()),
            Self::HSplit(split) => Self::HSplit(split.normalize_splits()),
        }
    }

    fn remove_empty_groups_and_splits(&self) -> Option<Self> {
        match self {
            Self::Group(group) => {
                if group.tabs.len() > 0 {
                    Some(Self::Group(group.clone()))
                } else {
                    None
                }
            }
            Self::VSplit(split) => split
                .remove_empty_groups_and_splits()
                .map(|split| Self::VSplit(split)),
            Self::HSplit(split) => split
                .remove_empty_groups_and_splits()
                .map(|split| Self::HSplit(split)),
        }
    }

    pub fn activate_one_tab_per_group(&self) -> Self {
        match self {
            Self::Group(group) => Self::Group(group.activate_one_tab_per_group()),
            Self::VSplit(split) => Self::VSplit(split.activate_one_tab_per_group()),
            Self::HSplit(split) => Self::HSplit(split.activate_one_tab_per_group()),
        }
    }

    pub fn clean(&self) -> Self {
        self.trim().activate_one_tab_per_group()
    }

    pub fn next_vsplit_id(&self) -> VSplitId {
        self.highest_vsplit_id().next()
    }

    pub fn highest_vsplit_id(&self) -> VSplitId {
        self.find_highest_vsplit_id(VSplitId::zero())
    }

    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId {
        match self {
            Self::Group(group) => cur_highest,
            Self::VSplit(split) => split.find_highest_vsplit_id(cur_highest),
            Self::HSplit(split) => split.find_highest_vsplit_id(cur_highest),
        }
    }

    pub fn next_hsplit_id(&self) -> HSplitId {
        self.highest_hsplit_id().next()
    }

    pub fn highest_hsplit_id(&self) -> HSplitId {
        self.find_highest_hsplit_id(HSplitId::zero())
    }

    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId {
        match self {
            Self::Group(group) => cur_highest,
            Self::VSplit(split) => split.find_highest_hsplit_id(cur_highest),
            Self::HSplit(split) => split.find_highest_hsplit_id(cur_highest),
        }
    }

    pub fn next_group_id(&self) -> GroupId {
        self.highest_group_id().next()
    }

    pub fn highest_group_id(&self) -> GroupId {
        self.find_highest_group_id(GroupId::zero())
    }

    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId {
        match self {
            Self::Group(group) => cur_highest.max(group.id),
            Self::VSplit(split) => split.find_highest_group_id(cur_highest),
            Self::HSplit(split) => split.find_highest_group_id(cur_highest),
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
            Self::Group(group) => group.find_highest_tab_id(cur_highest),
            Self::VSplit(split) => split.find_highest_tab_id(cur_highest),
            Self::HSplit(split) => split.find_highest_tab_id(cur_highest),
        }
    }
}
impl From<Group> for Layout {
    fn from(group: Group) -> Self {
        Self::Group(group)
    }
}
impl From<VSplit> for Layout {
    fn from(split: VSplit) -> Self {
        Self::VSplit(split)
    }
}
impl From<HSplit> for Layout {
    fn from(split: HSplit) -> Self {
        Self::HSplit(split)
    }
}

pub trait OrientedSplitChild
where
    Self: Sized,
{
    fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout;
    fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self;
    fn find_focused_tab(&self) -> Option<TabId>;
    fn find_tab_index(&self, tab_id: TabId) -> Option<usize>;
    fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId>;
    fn focus_tab(&self, tab_id: TabId) -> Self;
    fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self;
    fn get_tab(&self, tab_id: TabId) -> Option<Tab>;
    fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self;
    fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self;
    fn remove_tab(&self, tab_id: TabId) -> Self;
    fn set_active_tab_in_group(&self, group_id: GroupId, tab_id: TabId) -> Self;
    fn activate_one_tab_per_group(&self) -> Self;
    fn tab_exists(&self, tab_id: TabId) -> bool;
    fn group_exists(&self, group_id: GroupId) -> bool;
    fn normalize_splits(&self) -> Self;
    fn remove_empty_groups_and_splits(&self) -> Option<Self>;
    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId;
    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId;
    fn find_highest_group_id(&self, cur_highest: GroupId) -> GroupId;
    fn find_highest_tab_id(&self, cur_highest: TabId) -> TabId;
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplitChild<T> {
    pub width: f64,
    pub child: T,
}
impl<T: OrientedSplitChild> SplitChild<T> {
    pub fn find_focused_tab(&self) -> Option<TabId> {
        self.child.find_focused_tab()
    }

    pub fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self {
        Self {
            width: self.width,
            child: self
                .child
                .create_new_tab(group_id, index, title, next_tab_id),
        }
    }

    pub fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        self.child.find_tab_group(tab_id)
    }

    pub fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        self.child.find_tab_index(tab_id)
    }

    pub fn focus_tab(&self, tab_id: TabId) -> Self {
        Self {
            width: self.width,
            child: self.child.focus_tab(tab_id),
        }
    }

    pub fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout {
        self.child.split(
            group_id,
            direction,
            tab,
            next_group_id,
            next_vsplit_id,
            next_hsplit_id,
        )
    }

    pub fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        Self {
            width: self.width,
            child: self.child.insert_tab(group_id, index, tab),
        }
    }

    pub fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        self.child.get_tab(tab_id)
    }

    pub fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        Self {
            width: self.width,
            child: self.child.adjust_vsplit(vsplit_id, index, new_location),
        }
    }

    pub fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        Self {
            width: self.width,
            child: self.child.adjust_hsplit(hsplit_id, index, new_location),
        }
    }

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

    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId {
        self.child.find_highest_vsplit_id(cur_highest)
    }

    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId {
        self.child.find_highest_hsplit_id(cur_highest)
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
    pub fn find_focused_tab(&self) -> Option<TabId> {
        self.children
            .iter()
            .filter_map(|child| child.find_focused_tab())
            .next()
    }

    pub fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.create_new_tab(group_id, index, title, next_tab_id))
                .collect(),
        }
    }

    pub fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        self.children
            .iter()
            .filter_map(|child| child.find_tab_group(tab_id))
            .next()
    }

    pub fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        self.children
            .iter()
            .filter_map(|child| child.find_tab_index(tab_id))
            .next()
    }

    pub fn focus_tab(&self, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.focus_tab(tab_id))
                .collect(),
        }
        .into()
    }

    pub fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout {
        let mut children = vec![];

        for child in self.children.iter() {
            match child.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ) {
                Layout::VSplit(split) => {
                    for vs_child in split.children.into_iter() {
                        children.push(SplitChild {
                            width: vs_child.width * child.width,
                            child: vs_child.child,
                        });
                    }
                }
                Layout::Group(group) => children.push(SplitChild {
                    width: child.width,
                    child: VSplitChild::Group(group),
                }),
                Layout::HSplit(split) => children.push(SplitChild {
                    width: child.width,
                    child: VSplitChild::HSplit(split),
                }),
            }
        }

        Self {
            id: self.id,
            children,
        }
        .into()
    }

    pub fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.insert_tab(group_id, index, tab))
                .collect(),
        }
    }

    pub fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        self.children
            .iter()
            .filter_map(|child| child.get_tab(tab_id))
            .next()
    }

    pub fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        const MIN_SPLIT_WIDTH: f64 = 0.1;

        let mut new_children = self
            .children
            .iter()
            .map(|child| child.adjust_vsplit(vsplit_id, index, new_location))
            .collect::<Vec<SplitChild<VSplitChild>>>();

        let len = new_children.len();

        if self.id == vsplit_id && index < len - 1 {
            let widths = new_children
                .iter()
                .map(|child| child.width)
                .collect::<Vec<_>>();

            let new_widths = slide_numbers(widths, MIN_SPLIT_WIDTH, index, new_location);

            for (i, child) in new_children.iter_mut().enumerate() {
                child.width = new_widths[i];
            }
        }

        Self {
            id: self.id,
            children: new_children,
        }
    }

    pub fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.adjust_hsplit(hsplit_id, index, new_location))
                .collect(),
        }
    }

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

    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId {
        self.children
            .iter()
            .map(|child| child.find_highest_vsplit_id(cur_highest))
            .max()
            .unwrap_or(VSplitId::zero())
            .max(self.id)
            .max(cur_highest)
    }

    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId {
        self.children
            .iter()
            .map(|child| child.find_highest_hsplit_id(cur_highest))
            .max()
            .unwrap_or(HSplitId::zero())
            .max(cur_highest)
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
    fn find_focused_tab(&self) -> Option<TabId> {
        match self {
            VSplitChild::Group(group) => group.find_focused_tab(),
            VSplitChild::HSplit(split) => split.find_focused_tab(),
        }
    }

    fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self {
        match self {
            VSplitChild::Group(group) => {
                Self::Group(group.create_new_tab(group_id, index, title, next_tab_id))
            }
            VSplitChild::HSplit(split) => {
                Self::HSplit(split.create_new_tab(group_id, index, title, next_tab_id))
            }
        }
    }

    fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        match self {
            VSplitChild::Group(group) => group.find_tab_group(tab_id),
            VSplitChild::HSplit(split) => split.find_tab_group(tab_id),
        }
    }

    fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        match self {
            VSplitChild::Group(group) => group.find_tab_index(tab_id),
            VSplitChild::HSplit(split) => split.find_tab_index(tab_id),
        }
    }

    fn focus_tab(&self, tab_id: TabId) -> Self {
        match self {
            VSplitChild::Group(group) => VSplitChild::Group(group.focus_tab(tab_id)),
            VSplitChild::HSplit(split) => VSplitChild::HSplit(split.focus_tab(tab_id)),
        }
    }

    fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout {
        match self {
            VSplitChild::Group(group) => group.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
            VSplitChild::HSplit(split) => split.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
        }
    }

    fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        match self {
            VSplitChild::Group(group) => Self::Group(group.insert_tab(group_id, index, tab)),
            VSplitChild::HSplit(split) => Self::HSplit(split.insert_tab(group_id, index, tab)),
        }
    }

    fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        match self {
            VSplitChild::Group(group) => group.get_tab(tab_id),
            VSplitChild::HSplit(split) => split.get_tab(tab_id),
        }
    }

    fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        match self {
            VSplitChild::Group(group) => Self::Group(group.clone()),
            VSplitChild::HSplit(split) => {
                Self::HSplit(split.adjust_vsplit(vsplit_id, index, new_location))
            }
        }
    }

    fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        match self {
            VSplitChild::Group(group) => Self::Group(group.clone()),
            VSplitChild::HSplit(split) => {
                Self::HSplit(split.adjust_hsplit(hsplit_id, index, new_location))
            }
        }
    }

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

    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId {
        match self {
            VSplitChild::Group(group) => cur_highest,
            VSplitChild::HSplit(split) => split.find_highest_vsplit_id(cur_highest),
        }
    }

    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId {
        match self {
            VSplitChild::Group(group) => cur_highest,
            VSplitChild::HSplit(split) => split.find_highest_hsplit_id(cur_highest),
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
    fn find_focused_tab(&self) -> Option<TabId> {
        self.children
            .iter()
            .filter_map(|child| child.find_focused_tab())
            .next()
    }

    fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.create_new_tab(group_id, index, title, next_tab_id))
                .collect(),
        }
    }

    pub fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        self.children
            .iter()
            .filter_map(|child| child.find_tab_group(tab_id))
            .next()
    }

    pub fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        self.children
            .iter()
            .filter_map(|child| child.find_tab_index(tab_id))
            .next()
    }

    pub fn focus_tab(&self, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.focus_tab(tab_id))
                .collect(),
        }
    }

    pub fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout {
        let mut children = vec![];

        for child in self.children.iter() {
            match child.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ) {
                Layout::HSplit(split) => {
                    for hs_child in split.children.into_iter() {
                        children.push(SplitChild {
                            width: hs_child.width * child.width,
                            child: hs_child.child,
                        });
                    }
                }
                Layout::Group(group) => children.push(SplitChild {
                    width: child.width,
                    child: HSplitChild::Group(group),
                }),
                Layout::VSplit(split) => children.push(SplitChild {
                    width: child.width,
                    child: HSplitChild::VSplit(split),
                }),
            }
        }

        Self {
            id: self.id,
            children,
        }
        .into()
    }

    pub fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.insert_tab(group_id, index, tab))
                .collect(),
        }
    }

    pub fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        self.children
            .iter()
            .filter_map(|child| child.get_tab(tab_id))
            .next()
    }

    pub fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        Self {
            id: self.id,
            children: self
                .children
                .iter()
                .map(|child| child.adjust_vsplit(vsplit_id, index, new_location))
                .collect(),
        }
    }

    pub fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        const MIN_SPLIT_WIDTH: f64 = 0.1;

        let mut new_children = self
            .children
            .iter()
            .map(|child| child.adjust_hsplit(hsplit_id, index, new_location))
            .collect::<Vec<SplitChild<HSplitChild>>>();

        let len = new_children.len();

        if self.id == hsplit_id && index < len - 1 {
            let widths = new_children
                .iter()
                .map(|child| child.width)
                .collect::<Vec<_>>();

            let new_widths = slide_numbers(widths, MIN_SPLIT_WIDTH, index, new_location);

            for (i, child) in new_children.iter_mut().enumerate() {
                child.width = new_widths[i];
            }
        }

        Self {
            id: self.id,
            children: new_children,
        }
    }

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

    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId {
        self.children
            .iter()
            .map(|child| child.find_highest_vsplit_id(cur_highest))
            .max()
            .unwrap_or(VSplitId::zero())
            .max(cur_highest)
    }

    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId {
        self.children
            .iter()
            .map(|child| child.find_highest_hsplit_id(cur_highest))
            .max()
            .unwrap_or(HSplitId::zero())
            .max(self.id)
            .max(cur_highest)
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
    fn find_focused_tab(&self) -> Option<TabId> {
        match self {
            HSplitChild::Group(group) => group.find_focused_tab(),
            HSplitChild::VSplit(split) => split.find_focused_tab(),
        }
    }

    fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self {
        match self {
            HSplitChild::Group(group) => {
                Self::Group(group.create_new_tab(group_id, index, title, next_tab_id))
            }
            HSplitChild::VSplit(split) => {
                Self::VSplit(split.create_new_tab(group_id, index, title, next_tab_id))
            }
        }
    }

    fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        match self {
            HSplitChild::Group(group) => group.find_tab_group(tab_id),
            HSplitChild::VSplit(split) => split.find_tab_group(tab_id),
        }
    }

    fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        match self {
            HSplitChild::Group(group) => group.find_tab_index(tab_id),
            HSplitChild::VSplit(split) => split.find_tab_index(tab_id),
        }
    }

    fn focus_tab(&self, tab_id: TabId) -> Self {
        match self {
            HSplitChild::Group(group) => HSplitChild::Group(group.focus_tab(tab_id)),
            HSplitChild::VSplit(split) => HSplitChild::VSplit(split.focus_tab(tab_id)),
        }
    }

    fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout {
        match self {
            HSplitChild::Group(group) => group.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
            HSplitChild::VSplit(split) => split.split(
                group_id,
                direction,
                tab,
                next_group_id,
                next_vsplit_id,
                next_hsplit_id,
            ),
        }
    }

    fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        match self {
            HSplitChild::Group(group) => Self::Group(group.insert_tab(group_id, index, tab)),
            HSplitChild::VSplit(split) => Self::VSplit(split.insert_tab(group_id, index, tab)),
        }
    }

    fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        match self {
            HSplitChild::Group(group) => group.get_tab(tab_id),
            HSplitChild::VSplit(split) => split.get_tab(tab_id),
        }
    }

    fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        match self {
            HSplitChild::Group(group) => Self::Group(group.clone()),
            HSplitChild::VSplit(split) => {
                Self::VSplit(split.adjust_vsplit(vsplit_id, index, new_location))
            }
        }
    }

    fn adjust_hsplit(&self, hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        match self {
            HSplitChild::Group(group) => Self::Group(group.clone()),
            HSplitChild::VSplit(split) => {
                Self::VSplit(split.adjust_hsplit(hsplit_id, index, new_location))
            }
        }
    }

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

    fn find_highest_vsplit_id(&self, cur_highest: VSplitId) -> VSplitId {
        match self {
            HSplitChild::Group(group) => cur_highest,
            HSplitChild::VSplit(split) => split.find_highest_vsplit_id(cur_highest),
        }
    }

    fn find_highest_hsplit_id(&self, cur_highest: HSplitId) -> HSplitId {
        match self {
            HSplitChild::Group(group) => cur_highest,
            HSplitChild::VSplit(split) => split.find_highest_hsplit_id(cur_highest),
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
    pub fn find_focused_tab(&self) -> Option<TabId> {
        self.tabs
            .iter()
            .filter_map(|tab| if tab.focused { Some(tab.id) } else { None })
            .next()
    }

    pub fn create_new_tab(
        &self,
        group_id: GroupId,
        index: usize,
        title: &str,
        next_tab_id: TabId,
    ) -> Self {
        let mut new_tabs = self.tabs.clone();

        if self.id == group_id {
            let new_tab = Tab {
                id: next_tab_id,
                active_in_group: false,
                focused: false,
                title: title.to_string(),
            };

            if index <= self.tabs.len() {
                new_tabs.insert(index, new_tab);
            } else {
                new_tabs.push(new_tab);
            }
        }

        Self {
            id: self.id,
            tabs: new_tabs,
        }
    }

    pub fn find_tab_group(&self, tab_id: TabId) -> Option<GroupId> {
        if self.tabs.iter().any(|tab| tab.id == tab_id) {
            Some(self.id)
        } else {
            None
        }
    }

    pub fn find_tab_index(&self, tab_id: TabId) -> Option<usize> {
        self.tabs
            .iter()
            .enumerate()
            .filter_map(|(i, tab)| if tab.id == tab_id { Some(i) } else { None })
            .next()
    }

    pub fn focus_tab(&self, tab_id: TabId) -> Self {
        Self {
            id: self.id,
            tabs: if self.tab_exists(tab_id) {
                self.tabs
                    .iter()
                    .map(|tab| Tab {
                        id: tab.id,
                        active_in_group: tab.active_in_group,
                        focused: tab.id == tab_id,
                        title: tab.title.clone(),
                    })
                    .collect()
            } else {
                self.tabs
                    .iter()
                    .map(|tab| Tab {
                        id: tab.id,
                        active_in_group: tab.active_in_group,
                        focused: false,
                        title: tab.title.clone(),
                    })
                    .collect()
            },
        }
    }

    pub fn split(
        &self,
        group_id: GroupId,
        direction: SplitDirection,
        tab: &Tab,
        next_group_id: GroupId,
        next_vsplit_id: VSplitId,
        next_hsplit_id: HSplitId,
    ) -> Layout {
        if group_id == self.id {
            match direction {
                SplitDirection::Left => VSplit {
                    id: next_vsplit_id,
                    children: vec![
                        SplitChild {
                            width: 0.5,
                            child: VSplitChild::Group(Group {
                                id: next_group_id,
                                tabs: vec![tab.clone()],
                            }),
                        },
                        SplitChild {
                            width: 0.5,
                            child: VSplitChild::Group(self.clone()),
                        },
                    ],
                }
                .into(),
                SplitDirection::Right => VSplit {
                    id: next_vsplit_id,
                    children: vec![
                        SplitChild {
                            width: 0.5,
                            child: VSplitChild::Group(self.clone()),
                        },
                        SplitChild {
                            width: 0.5,
                            child: VSplitChild::Group(Group {
                                id: next_group_id,
                                tabs: vec![tab.clone()],
                            }),
                        },
                    ],
                }
                .into(),
                SplitDirection::Up => HSplit {
                    id: next_hsplit_id,
                    children: vec![
                        SplitChild {
                            width: 0.5,
                            child: HSplitChild::Group(Group {
                                id: next_group_id,
                                tabs: vec![tab.clone()],
                            }),
                        },
                        SplitChild {
                            width: 0.5,
                            child: HSplitChild::Group(self.clone()),
                        },
                    ],
                }
                .into(),
                SplitDirection::Down => HSplit {
                    id: next_hsplit_id,
                    children: vec![
                        SplitChild {
                            width: 0.5,
                            child: HSplitChild::Group(self.clone()),
                        },
                        SplitChild {
                            width: 0.5,
                            child: HSplitChild::Group(Group {
                                id: next_group_id,
                                tabs: vec![tab.clone()],
                            }),
                        },
                    ],
                }
                .into(),
            }
        } else {
            self.clone().into()
        }
    }

    pub fn insert_tab(&self, group_id: GroupId, index: usize, tab: &Tab) -> Self {
        if group_id == self.id {
            let mut tabs = self.tabs.clone();

            if index > tabs.len() {
                tabs.push(tab.clone());
            } else {
                tabs.insert(index, tab.clone())
            }

            Self { id: self.id, tabs }
        } else {
            self.clone()
        }
    }

    pub fn get_tab(&self, tab_id: TabId) -> Option<Tab> {
        self.tabs.iter().find(|tab| tab.id == tab_id).cloned()
    }

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
        let active_tab_index = match self
            .tabs
            .iter()
            .enumerate()
            .filter_map(|(index, tab)| if tab.focused { Some(index) } else { None })
            .next()
        {
            // If a tab has focus, that's the active tab
            Some(index) => index,
            // Otherwise, we need to choose one
            None => {
                // Get all the tabs that are marked as active.
                let active_tab_indices = self
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

                if active_tab_indices.len() == 0 {
                    // If none are active, activate the first one
                    0
                } else {
                    // Otherwise, use the first of the active ones. The
                    // others will be deactivated.
                    active_tab_indices[0]
                }
            }
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
    pub focused: bool,
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

fn slide_numbers(nums: Vec<f64>, min_dist: f64, index: usize, move_to: f64) -> Vec<f64> {
    let total: f64 = nums.iter().sum();

    let mut new_nums = nums.to_vec();

    let len = new_nums.len();

    // Find the minimum and maximum possible value for the new position,
    // given the number of elements before and after index.
    let min = (index + 1) as f64 * min_dist;
    let max = total - (len - index - 1) as f64 * min_dist;

    // Clamp the movement to the min and max position
    let move_to = move_to.clamp(min, max);

    // The original position of the index
    let original_pos = (0..=index).map(|i| new_nums[i]).sum::<f64>();

    if move_to < original_pos {
        // Compress numbers before index
        for i in (0..index + 1).rev() {
            // The total of all the numbers to the left of the index,
            // not including the one touching the index
            let leftmost_width = (0..index).map(|i| new_nums[i]).sum::<f64>();
            // The total of all the numbers to the left of the index,
            // including the one touching the index
            let left_width = leftmost_width + new_nums[index];

            // How much we still need to "shave off" the numbers to the left of index
            let excess = left_width - move_to;

            // If we need to shave more off...
            if excess > 0.0 {
                // Reduce the next number to the left by as much as possible, not
                // making it smaller than min_dist
                new_nums[i] = (new_nums[i] - excess).max(min_dist);
            }
        }

        // Expand the width to the right of the index so the total is the same
        new_nums[index + 1] += total - new_nums.iter().sum::<f64>();
    } else if move_to > original_pos {
        // Compress numbers after index
        for i in index + 1..new_nums.len() {
            // The total of all the numbers to the right of the index,
            // not including the one touching the index
            let rightmost_width = (index + 2..new_nums.len())
                .map(|i| new_nums[i])
                .sum::<f64>();
            // The total of all the numbers to the right of the index,
            // including the one touching the index
            let right_width = rightmost_width + new_nums[index + 1];

            // How much we still need to "shave off" the numbers to the wright
            let excess = right_width - (total - move_to);

            // If we need to shave more off...
            if excess > 0.0 {
                // Reduce the next number to the left by as much as possible, not
                // making it smaller than min_dist
                new_nums[i] = (new_nums[i] - excess).max(min_dist);
            }

            // Expand the width to the left of the index so the total is the same
            new_nums[index] += total - new_nums.iter().sum::<f64>();
        }
    }

    new_nums
}

#[cfg(test)]
mod tests {
    use super::slide_numbers;

    fn approx_eq(left: Vec<f64>, right: Vec<f64>) {
        const TOL: f64 = 0.000000000001;
        assert!(
            left.len() == right.len(),
            "unequal lengths: {} != {}",
            left.len(),
            right.len()
        );
        for i in 0..left.len() {
            assert!(
                (left[i] - right[i]).abs() < TOL,
                "not approx eq: {:?} != {:?}",
                left,
                right
            );
        }
    }

    #[test]
    fn slide_numbers_left() {
        approx_eq(
            slide_numbers(vec![0.2, 0.2, 0.2, 0.2, 0.2, 0.2], 0.1, 1, 0.25),
            vec![0.15, 0.1, 0.35, 0.2, 0.2, 0.2],
        );
    }

    #[test]
    fn slide_numbers_right() {
        approx_eq(
            slide_numbers(vec![0.2, 0.2, 0.2, 0.2, 0.2, 0.2], 0.1, 1, 0.65),
            vec![0.2, 0.45, 0.1, 0.1, 0.15, 0.2],
        );
    }
}
