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

    pub fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        match self {
            Layout::Group(group) => Layout::Group(group.clone()),
            Layout::VSplit(split) => {
                Layout::VSplit(split.adjust_vsplit(vsplit_id, index, new_location))
            }
            Layout::HSplit(split) => {
                Layout::HSplit(split.adjust_vsplit(vsplit_id, index, new_location))
            }
        }
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
    fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self;
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
    pub fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        Self {
            width: self.width,
            child: self.child.adjust_vsplit(vsplit_id, index, new_location),
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
    fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        match self {
            VSplitChild::Group(group) => Self::Group(group.clone()),
            VSplitChild::HSplit(split) => {
                Self::HSplit(split.adjust_vsplit(vsplit_id, index, new_location))
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
    fn adjust_vsplit(&self, vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        match self {
            HSplitChild::Group(group) => Self::Group(group.clone()),
            HSplitChild::VSplit(split) => {
                Self::VSplit(split.adjust_vsplit(vsplit_id, index, new_location))
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
