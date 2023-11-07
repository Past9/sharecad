use crate::command::Command;

use super::{GroupId, HSplitId, TabId, VSplitId};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SplitDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DropTabOffer {
    InGroup {
        group_id: GroupId,
        index: usize,
    },
    Split {
        group_id: GroupId,
        direction: SplitDirection,
    },
}
impl DropTabOffer {
    pub fn group_id(&self) -> GroupId {
        match self {
            DropTabOffer::InGroup { group_id, .. } => *group_id,
            DropTabOffer::Split { group_id, .. } => *group_id,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TabsCommand {
    DragTab {
        group_id: GroupId,
        index: usize,
        tab_id: TabId,
    },
    OfferDropTab(DropTabOffer),
    CancelOfferDropTab,
    DropTab,
    CloseTab {
        tab_id: TabId,
    },
    AdjustVSplit {
        vsplit_id: VSplitId,
        index: usize,
        new_location: f64,
    },
    AdjustHSplit {
        hsplit_id: HSplitId,
        index: usize,
        new_location: f64,
    },
    SetActiveTabInGroup {
        group_id: GroupId,
        tab_id: TabId,
    },
    FocusTab {
        tab_id: TabId,
    },
}
impl Command for TabsCommand {
    const TYPE_NAME: &'static str = "TabsCommand";
}
impl TabsCommand {
    pub fn drag_tab(group_id: GroupId, index: usize, tab_id: TabId) -> Self {
        Self::DragTab {
            group_id,
            index,
            tab_id,
        }
    }

    pub fn drop_tab() -> Self {
        Self::DropTab
    }

    pub fn close_tab(tab_id: TabId) -> Self {
        Self::CloseTab { tab_id }
    }

    pub fn adjust_vsplit(vsplit_id: VSplitId, index: usize, new_location: f64) -> Self {
        Self::AdjustVSplit {
            vsplit_id,
            index,
            new_location,
        }
    }

    pub fn adjust_hsplit(hsplit_id: HSplitId, index: usize, new_location: f64) -> Self {
        Self::AdjustHSplit {
            hsplit_id,
            index,
            new_location,
        }
    }

    pub fn focus_tab(tab_id: TabId) -> Self {
        Self::FocusTab { tab_id }
    }

    pub fn set_active_tab_in_group(group_id: GroupId, tab_id: TabId) -> Self {
        Self::SetActiveTabInGroup { group_id, tab_id }
    }

    pub fn offer_drop_tab_in_group(group_id: GroupId, index: usize) -> Self {
        Self::OfferDropTab(DropTabOffer::InGroup { group_id, index })
    }

    pub fn cancel_offer_drop_tab() -> Self {
        Self::CancelOfferDropTab
    }

    pub fn offer_drop_tab(offer: DropTabOffer) -> Self {
        Self::OfferDropTab(offer)
    }

    pub fn offer_drop_tab_split_group(group_id: GroupId, direction: SplitDirection) -> Self {
        Self::OfferDropTab(DropTabOffer::Split {
            group_id,
            direction,
        })
    }

    pub fn offer_drop_tab_split_group_left(group_id: GroupId) -> Self {
        Self::OfferDropTab(DropTabOffer::Split {
            group_id,
            direction: SplitDirection::Left,
        })
    }

    pub fn offer_drop_tab_split_group_right(group_id: GroupId) -> Self {
        Self::OfferDropTab(DropTabOffer::Split {
            group_id,
            direction: SplitDirection::Right,
        })
    }

    pub fn offer_drop_tab_split_group_up(group_id: GroupId) -> Self {
        Self::OfferDropTab(DropTabOffer::Split {
            group_id,
            direction: SplitDirection::Up,
        })
    }

    pub fn offer_drop_tab_split_group_down(group_id: GroupId) -> Self {
        Self::OfferDropTab(DropTabOffer::Split {
            group_id,
            direction: SplitDirection::Down,
        })
    }
}
