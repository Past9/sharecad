use super::{DropTabOffer, GroupId, Layout, TabId, TabsCommand};

#[derive(Clone, PartialEq, Debug)]
pub struct DraggingTab {
    pub group_id: GroupId,
    pub index: usize,
    pub tab_id: TabId,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    pub dragging_tab: Option<DraggingTab>,
    pub drop_tab_offer: Option<DropTabOffer>,
    pub layout: Layout,
}
impl Config {
    pub fn modify(&self, command: &TabsCommand) -> Self {
        match command {
            TabsCommand::DragTab {
                group_id,
                index,
                tab_id,
            } => {
                let mut new_config = self.clone();
                new_config.dragging_tab = Some(DraggingTab {
                    group_id: *group_id,
                    index: *index,
                    tab_id: *tab_id,
                });
                new_config
            }
            TabsCommand::OfferDropTab(offer) => {
                let mut new_config = self.clone();
                new_config.drop_tab_offer = Some(offer.clone());
                new_config
            }
            TabsCommand::CancelOfferDropTab => {
                let mut new_config = self.clone();
                new_config.drop_tab_offer = None;
                new_config
            }
            TabsCommand::DropTab => {
                let mut new_config = self.clone();

                if let Some(ref dragging_tab) = self.dragging_tab {
                    if let Some(tab) = new_config.layout.get_tab(dragging_tab.tab_id) {
                        if let Some(ref offer) = self.drop_tab_offer {
                            match offer {
                                DropTabOffer::InGroup { group_id, index } => {
                                    let index = if *group_id == dragging_tab.group_id
                                        && *index > dragging_tab.index
                                    {
                                        // We're going remove the tab from its current group before
                                        // inserting it into the new group. If we're dropping it into
                                        // the same group we're removing it from (just moving it within
                                        // the group), this will reduce the indices after it by 1. If
                                        // we're moving the tab to "later" in the group, we need to
                                        // reduce the drop index by 1 as well to compensate for this.
                                        index - 1
                                    } else {
                                        *index
                                    };

                                    new_config.layout = new_config
                                        .layout
                                        .remove_tab(dragging_tab.tab_id)
                                        .insert_tab(*group_id, index, &tab)
                                        .set_active_tab_in_group(*group_id, dragging_tab.tab_id);
                                }
                                DropTabOffer::Split {
                                    group_id,
                                    direction,
                                } => {
                                    new_config.layout = new_config
                                        .layout
                                        .remove_tab(dragging_tab.tab_id)
                                        .split(*group_id, *direction, &tab);
                                }
                            }
                        }
                    }
                }

                new_config.dragging_tab = None;
                new_config.drop_tab_offer = None;
                new_config
            }
            TabsCommand::CloseTab { tab_id } => {
                let mut new_config = self.clone();
                new_config.layout = new_config.layout.remove_tab(*tab_id);
                new_config
            }
            TabsCommand::AdjustVSplit {
                vsplit_id,
                index,
                new_location,
            } => {
                let mut new_config = self.clone();
                new_config.layout =
                    new_config
                        .layout
                        .adjust_vsplit(*vsplit_id, *index, *new_location);
                new_config
            }
            TabsCommand::AdjustHSplit {
                hsplit_id,
                index,
                new_location,
            } => {
                let mut new_config = self.clone();
                new_config.layout =
                    new_config
                        .layout
                        .adjust_hsplit(*hsplit_id, *index, *new_location);
                new_config
            }
            TabsCommand::SetActiveTabInGroup { group_id, tab_id } => {
                let mut new_config = self.clone();
                new_config.layout = new_config
                    .layout
                    .set_active_tab_in_group(*group_id, *tab_id);
                new_config
            }
        }
        .clean()
    }

    pub fn clean(&self) -> Self {
        let Self {
            mut dragging_tab,
            mut drop_tab_offer,
            mut layout,
        } = self.clone();

        layout = layout.clean();

        if let Some(ref d_tab) = dragging_tab {
            if !layout.tab_exists(d_tab.tab_id) {
                dragging_tab = None;
            }
        }

        if let Some(ref offer) = drop_tab_offer {
            match offer {
                DropTabOffer::InGroup { group_id, .. } => {
                    if !layout.group_exists(*group_id) {
                        drop_tab_offer = None;
                    }
                }
                DropTabOffer::Split { group_id, .. } => {
                    if !layout.group_exists(*group_id) {
                        drop_tab_offer = None;
                    }
                }
            }
        }

        Self {
            dragging_tab,
            drop_tab_offer,
            layout,
        }
    }
}
