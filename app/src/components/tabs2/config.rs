use super::{Command, DropTabOffer, Layout, TabId};

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    pub dragging_tab: Option<TabId>,
    pub drop_tab_offer: Option<DropTabOffer>,
    pub layout: Layout,
}
impl Config {
    pub fn modify(&self, command: &Command) -> Self {
        match command {
            Command::DragTab { tab_id } => {
                let mut new_config = self.clone();
                new_config.dragging_tab = Some(*tab_id);
                new_config
            }
            Command::OfferDropTab(offer) => {
                let mut new_config = self.clone();
                new_config.drop_tab_offer = Some(offer.clone());
                new_config
            }
            Command::DropTab => {
                let mut new_config = self.clone();
                log::debug!("drop_tab_offer {:?}", self.drop_tab_offer);
                if let Some(tab_id) = self.dragging_tab {
                    if let Some(tab) = new_config.layout.get_tab(tab_id) {
                        if let Some(ref offer) = self.drop_tab_offer {
                            match offer {
                                DropTabOffer::InGroup { group_id, index } => {
                                    new_config.layout = new_config
                                        .layout
                                        .remove_tab(tab_id)
                                        .insert_tab(*group_id, *index, &tab)
                                        .set_active_tab_in_group(*group_id, tab_id);
                                }
                                DropTabOffer::Split {
                                    group_id,
                                    direction,
                                } => todo!(),
                            }
                        }
                    }
                }
                new_config
            }
            Command::CloseTab { tab_id } => {
                let mut new_config = self.clone();
                new_config.layout = new_config.layout.remove_tab(*tab_id);
                new_config
            }
            Command::AdjustVSplit {
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
            Command::AdjustHSplit {
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
            Command::SetActiveTabInGroup { group_id, tab_id } => {
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

        if let Some(tab_id) = dragging_tab {
            if !layout.tab_exists(tab_id) {
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
