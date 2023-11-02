use super::{
    id::{GroupId, HSplitId, IdSource, TabId, VSplitId},
    layout::{Group, HSplit, HSplitChild, Layout, Tab, VSplit, VSplitChild},
};

fn make_default_splits(len: usize) -> Vec<f64> {
    (0..len - 1)
        .into_iter()
        .map(|i| i as f64 / len as f64)
        .collect()
}

pub struct LayoutBuilder {
    vsplit_ids: IdSource<VSplitId>,
    hsplit_ids: IdSource<HSplitId>,
    group_ids: IdSource<GroupId>,
    tab_ids: IdSource<TabId>,
}
impl LayoutBuilder {
    pub fn new() -> Self {
        Self {
            vsplit_ids: IdSource::new(),
            hsplit_ids: IdSource::new(),
            group_ids: IdSource::new(),
            tab_ids: IdSource::new(),
        }
    }

    pub fn group<F: FnOnce(&mut GroupBuilder)>(self, cb: F) -> Layout {
        let mut cx = GroupBuilder::new(self.tab_ids.clone());
        cb(&mut cx);
        Layout::Group(Group {
            id: self.group_ids.next(),
            tabs: cx.tabs,
        })
    }

    pub fn vsplit<F: FnOnce(&mut VSplitBuilder)>(self, cb: F) -> Layout {
        let mut cx = VSplitBuilder::new(
            self.vsplit_ids.clone(),
            self.hsplit_ids.clone(),
            self.group_ids.clone(),
            self.tab_ids.clone(),
        );
        cb(&mut cx);
        Layout::VSplit(VSplit {
            id: self.vsplit_ids.next(),
            splits: make_default_splits(cx.children.len()),
            children: cx.children,
        })
    }

    pub fn hsplit<F: FnOnce(&mut HSplitBuilder)>(self, cb: F) -> Layout {
        let mut cx = HSplitBuilder::new(
            self.vsplit_ids.clone(),
            self.hsplit_ids.clone(),
            self.group_ids.clone(),
            self.tab_ids.clone(),
        );
        cb(&mut cx);
        Layout::HSplit(HSplit {
            id: self.hsplit_ids.next(),
            splits: make_default_splits(cx.children.len()),
            children: cx.children,
        })
    }
}

pub struct GroupBuilder {
    tab_ids: IdSource<TabId>,
    tabs: Vec<Tab>,
}
impl GroupBuilder {
    pub fn new(tab_ids: IdSource<TabId>) -> Self {
        Self {
            tab_ids,
            tabs: vec![],
        }
    }

    pub fn tab(&mut self, title: &str) {
        self.tabs.push(Tab {
            id: self.tab_ids.next(),
            active_in_group: false,
            title: title.to_string(),
        })
    }
}

pub struct VSplitBuilder {
    vsplit_ids: IdSource<VSplitId>,
    hsplit_ids: IdSource<HSplitId>,
    group_ids: IdSource<GroupId>,
    tab_ids: IdSource<TabId>,
    children: Vec<VSplitChild>,
}
impl VSplitBuilder {
    pub fn new(
        vsplit_ids: IdSource<VSplitId>,
        hsplit_ids: IdSource<HSplitId>,
        group_ids: IdSource<GroupId>,
        tab_ids: IdSource<TabId>,
    ) -> Self {
        Self {
            vsplit_ids,
            hsplit_ids,
            group_ids,
            tab_ids,
            children: vec![],
        }
    }

    pub fn group<F: FnOnce(&mut GroupBuilder)>(&mut self, cb: F) {
        let mut cx = GroupBuilder::new(self.tab_ids.clone());
        cb(&mut cx);
        self.children.push(VSplitChild::Group(Group {
            id: self.group_ids.next(),
            tabs: cx.tabs,
        }))
    }

    pub fn hsplit<F: FnOnce(&mut HSplitBuilder)>(&mut self, cb: F) {
        let mut cx = HSplitBuilder::new(
            self.vsplit_ids.clone(),
            self.hsplit_ids.clone(),
            self.group_ids.clone(),
            self.tab_ids.clone(),
        );
        cb(&mut cx);
        self.children.push(VSplitChild::HSplit(HSplit {
            id: self.hsplit_ids.next(),
            splits: make_default_splits(cx.children.len()),
            children: cx.children,
        }))
    }
}

pub struct HSplitBuilder {
    vsplit_ids: IdSource<VSplitId>,
    hsplit_ids: IdSource<HSplitId>,
    group_ids: IdSource<GroupId>,
    tab_ids: IdSource<TabId>,
    children: Vec<HSplitChild>,
}
impl HSplitBuilder {
    pub fn new(
        vsplit_ids: IdSource<VSplitId>,
        hsplit_ids: IdSource<HSplitId>,
        group_ids: IdSource<GroupId>,
        tab_ids: IdSource<TabId>,
    ) -> Self {
        Self {
            vsplit_ids,
            hsplit_ids,
            group_ids,
            tab_ids,
            children: vec![],
        }
    }

    pub fn group<F: FnOnce(&mut GroupBuilder)>(&mut self, cb: F) {
        let mut cx = GroupBuilder::new(self.tab_ids.clone());
        cb(&mut cx);
        self.children.push(HSplitChild::Group(Group {
            id: self.group_ids.next(),
            tabs: cx.tabs,
        }))
    }

    pub fn vsplit<F: FnOnce(&mut VSplitBuilder)>(&mut self, cb: F) {
        let mut cx = VSplitBuilder::new(
            self.vsplit_ids.clone(),
            self.hsplit_ids.clone(),
            self.group_ids.clone(),
            self.tab_ids.clone(),
        );
        cb(&mut cx);
        self.children.push(HSplitChild::VSplit(VSplit {
            id: self.vsplit_ids.next(),
            splits: make_default_splits(cx.children.len()),
            children: cx.children,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_builder() {
        let layout = LayoutBuilder::new().vsplit(|cx| {
            cx.group(|cx| {
                cx.tab("Some file");
                cx.tab("Some file with a very long path.fileextension");
            });
            cx.hsplit(|cx| {
                cx.group(|cx| {
                    cx.tab("A file 1");
                });
                cx.group(|cx| {
                    cx.tab("A file 2");
                });
            });
            cx.group(|cx| {
                cx.tab("A file 3");
                cx.tab("Another file");
            });
        });

        println!("layout {:#?}", layout);
    }
}
