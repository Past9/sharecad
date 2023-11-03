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
            Command::DragTab { tab_id } => self.clone(),
            Command::OfferDropTab(offer) => {
                let mut new_config = self.clone();
                new_config.drop_tab_offer = Some(offer.clone());
                new_config
            }
            Command::DropTab => self.clone(),
            Command::CloseTab { tab_id } => {
                let mut new_config = self.clone();
                new_config.layout = new_config.layout.remove_tab(*tab_id);
                new_config
            }
            Command::AdjustVSplit {
                vsplit_id,
                index,
                new_location,
            } => self.clone(),
            Command::AdjustHSplit {
                hsplit_id,
                index,
                new_location,
            } => self.clone(),
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
