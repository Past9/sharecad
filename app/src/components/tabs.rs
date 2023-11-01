use crate::{
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use async_channel::Sender;
use dioxus::{html::input_data::MouseButton, prelude::*};
use futures::executor::block_on;
use web_sys::DragEventInit;

pub fn config(layout: TabLayout) -> TabConfig {
    TabConfig {
        dragged_tab: None,
        drop_tab_offer: None,
        layout,
    }
}

pub fn tab(id: u32, title: &str, active_in_group: bool) -> TabProps {
    TabProps {
        tab_id: id,
        title: title.to_string(),
        active_in_group,
    }
}

pub fn group<const N: usize>(id: u32, tabs: [TabProps; N]) -> TabLayout {
    TabLayout::Group(TabGroup {
        group_id: id,
        tabs: tabs.to_vec(),
    })
}

pub fn vsplit(id: u32, split: f64, left: TabLayout, right: TabLayout) -> TabLayout {
    TabLayout::Split(TabSplit {
        direction: TabSplitDirection::Vertical,
        layout_id: id,
        location: split,
        a: Box::new(left),
        b: Box::new(right),
    })
}

pub fn hsplit(id: u32, split: f64, top: TabLayout, bottom: TabLayout) -> TabLayout {
    TabLayout::Split(TabSplit {
        direction: TabSplitDirection::Horizontal,
        layout_id: id,
        location: split,
        a: Box::new(top),
        b: Box::new(bottom),
    })
}

#[derive(Clone, PartialEq, Debug)]
pub struct TabConfig {
    dragged_tab: Option<u32>,
    drop_tab_offer: Option<TabDropInExistingGroup>,
    layout: TabLayout,
}
impl TabConfig {
    fn modify(&self, command: &ConfigCommand) -> Self {
        match command {
            ConfigCommand::StartDraggingTab { tab_id } => {
                let mut new_config = self.clone();
                new_config.dragged_tab = Some(*tab_id);
                new_config
            }
            ConfigCommand::StopDraggingTab => {
                let mut new_config = self.clone();
                new_config.dragged_tab = None;
                new_config.drop_tab_offer = None;
                new_config
            }
            ConfigCommand::DropTabInExistingGroup => self.clone(),
            ConfigCommand::OfferTabDropInExistingGroup(ref offer) => {
                log::debug!("drop tab offer: {:?}", offer);
                let mut new_config = self.clone();
                new_config.drop_tab_offer = Some(offer.clone());
                new_config
            }
            ConfigCommand::Layout(command) => {
                let mut new_config = self.clone();
                new_config.layout = new_config.layout.modify(command);
                new_config
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TabDropInExistingGroup {
    group_id: u32,
    index: usize,
}

#[derive(Debug, PartialEq)]
enum ConfigCommand {
    StartDraggingTab { tab_id: u32 },
    StopDraggingTab,
    DropTabInExistingGroup,
    OfferTabDropInExistingGroup(TabDropInExistingGroup),
    Layout(LayoutCommand),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TabLayout {
    Group(TabGroup),
    Split(TabSplit),
}
impl TabLayout {
    fn modify(&self, command: &LayoutCommand) -> Self {
        match command {
            LayoutCommand::AdjustSplit {
                layout_id,
                new_location,
            } => self.adjust_split(*layout_id, *new_location),
            LayoutCommand::SetActiveInGroup { group_id, tab_id } => {
                self.set_active_tab_in_group(*group_id, *tab_id)
            }
        }
    }

    fn set_active_tab_in_group(&self, group_id: u32, tab_id: u32) -> Self {
        match self {
            TabLayout::Group(group) => {
                let new_group = if group.group_id == group_id {
                    let tabs = group
                        .tabs
                        .iter()
                        .map(|tab| {
                            let mut tab = tab.to_owned();
                            tab.active_in_group = tab.tab_id == tab_id;
                            tab
                        })
                        .collect::<Vec<_>>();
                    TabGroup { group_id, tabs }
                } else {
                    TabGroup {
                        group_id: group.group_id,
                        tabs: group.tabs.clone(),
                    }
                };

                TabLayout::Group(new_group)
            }
            TabLayout::Split(TabSplit {
                direction,
                layout_id,
                location,
                a,
                b,
            }) => TabLayout::Split(TabSplit {
                direction: *direction,
                layout_id: *layout_id,
                location: *location,
                a: Box::new(a.set_active_tab_in_group(group_id, tab_id)),
                b: Box::new(b.set_active_tab_in_group(group_id, tab_id)),
            }),
        }
    }

    fn adjust_split(&self, target_layout_id: u32, new_location: f64) -> Self {
        match self {
            TabLayout::Split(TabSplit {
                direction,
                layout_id,
                location,
                a,
                b,
            }) => {
                if *layout_id == target_layout_id {
                    TabLayout::Split(TabSplit {
                        direction: direction.clone(),
                        layout_id: *layout_id,
                        location: new_location,
                        a: a.clone(),
                        b: b.clone(),
                    })
                } else {
                    TabLayout::Split(TabSplit {
                        direction: direction.clone(),
                        layout_id: *layout_id,
                        location: location.clone(),
                        a: Box::new(a.adjust_split(target_layout_id, new_location)),
                        b: Box::new(b.adjust_split(target_layout_id, new_location)),
                    })
                }
            }
            other => other.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TabSplitDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabSplit {
    direction: TabSplitDirection,
    layout_id: u32,
    location: f64,
    a: Box<TabLayout>,
    b: Box<TabLayout>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabGroup {
    group_id: u32,
    tabs: Vec<TabProps>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TabProps {
    tab_id: u32,
    title: String,
    active_in_group: bool,
}

#[derive(Debug, PartialEq)]
enum LayoutCommand {
    AdjustSplit { layout_id: u32, new_location: f64 },
    SetActiveInGroup { group_id: u32, tab_id: u32 },
}

#[derive(Clone)]
struct CommandBus {
    sender: Sender<ConfigCommand>,
}
impl CommandBus {
    fn new(sender: Sender<ConfigCommand>) -> Self {
        Self { sender }
    }

    async fn send(&self, command: ConfigCommand) {
        self.sender.send(command).await.unwrap();
    }

    fn send_blocking(&self, command: ConfigCommand) {
        block_on(self.send(command))
    }
}
impl PartialEq for CommandBus {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
#[inline_props]
pub fn TabArea<'a>(
    cx: Scope,
    config: &'a TabConfig,
    on_config_changed: EventHandler<'a, TabConfig>,
) -> Element {
    let (sender, receiver) = cx.use_hook(|| async_channel::unbounded::<ConfigCommand>());
    let bus = CommandBus::new(sender.clone());

    let next_command = use_state::<Option<ConfigCommand>>(cx, || None);

    let _cr = {
        to_owned![next_command, receiver];
        use_coroutine(cx, |_rx: UnboundedReceiver<()>| async move {
            loop {
                if let Ok(command) = receiver.recv().await {
                    next_command.set(Some(command));
                }
            }
        })
    };

    if let Some(command) = &**next_command {
        next_command.set(None);
        let new_config = config.modify(command);
        if new_config != **config {
            on_config_changed.call(new_config);
        }
    }

    cx.render(rsx! {
        div {
            class: "tab-area",
            TabLayoutComponent {
                layout: &config.layout,
                tab_drop_offer: config.drop_tab_offer.clone(),
                dragged_tab: config.dragged_tab,
                bus: bus
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabLayoutComponent<'a>(
    cx: Scoped,
    layout: &'a TabLayout,
    #[props(!optional)] tab_drop_offer: Option<TabDropInExistingGroup>,
    #[props(!optional)] dragged_tab: Option<u32>,
    bus: CommandBus,
) -> Element {
    match layout {
        TabLayout::Group(group) => cx.render(rsx! {
            TabGroupComponent {
                group: group,
                tab_drop_offer: tab_drop_offer.clone(),
                dragged_tab: dragged_tab.clone(),
                bus: bus.clone()
            }
        }),
        TabLayout::Split(split) => cx.render(rsx! {
            TabSplitComponent {
                split: split.clone(),
                tab_drop_offer: tab_drop_offer.clone(),
                dragged_tab: dragged_tab.clone(),
                bus: bus.clone()
            }
        }),
    }
}

#[derive(Clone, Debug)]
struct SplitDragPosition {
    start_split: f64,
    start_mousepos: f64,
    current_mousepos: f64,
}
impl SplitDragPosition {
    pub fn dist(&self) -> f64 {
        self.current_mousepos - self.start_mousepos
    }

    pub fn with_current(self, current: f64) -> Self {
        Self {
            start_split: self.start_split,
            start_mousepos: self.start_mousepos,
            current_mousepos: current,
        }
    }

    pub fn split_dist(&self, size: f64) -> f64 {
        self.dist() / (size - 5.0)
    }

    pub fn adjust_split(&self, size: f64) -> f64 {
        (self.start_split + self.split_dist(size)).clamp(0.0, 1.0)
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn TabSplitComponent(
    cx: Scoped,
    split: TabSplit,
    #[props(!optional)] tab_drop_offer: Option<TabDropInExistingGroup>,
    #[props(!optional)] dragged_tab: Option<u32>,
    bus: CommandBus,
) -> Element<'a> {
    let size = use_ref(cx, ComponentSize::default);
    let drag_pos = use_state(cx, || -> Option<SplitDragPosition> { None });

    let on_resize = use_state(cx, || {
        to_owned![size];
        OnResize::new(move |new_size: ComponentSize| size.set(new_size))
    });

    use_window_mouseup(cx, drag_pos, |drag_pos| {
        move |_| {
            drag_pos.set(None);
        }
    });

    use_window_mousemove(
        cx,
        (drag_pos, size, split, bus),
        |(drag_pos, size, split, bus)| {
            move |evt| {
                if let Some(ref pos) = *drag_pos.current() {
                    if evt.held_buttons().contains(MouseButton::Primary) {
                        let (position, space) = match split.direction {
                            TabSplitDirection::Vertical => {
                                (evt.client_coordinates().x, size.read().width)
                            }
                            TabSplitDirection::Horizontal => {
                                (evt.client_coordinates().y, size.read().height)
                            }
                        };

                        let new_drag_pos = pos.clone().with_current(position);
                        let new_location = new_drag_pos.adjust_split(space);
                        drag_pos.set(Some(new_drag_pos));
                        let command = ConfigCommand::Layout(LayoutCommand::AdjustSplit {
                            layout_id: split.layout_id,
                            new_location,
                        });

                        bus.send_blocking(command);
                    } else {
                        drag_pos.set(None);
                    }
                }
            }
        },
    );

    let direction_class = match split.direction {
        TabSplitDirection::Vertical => "vertical",
        TabSplitDirection::Horizontal => "horizontal",
    };

    let dragging_class = match drag_pos.is_some() {
        true => "dragging",
        false => "",
    };

    cx.render(rsx! {
        if drag_pos.is_some() {
            rsx! {
                div {
                    class: "split-drag-overlay {direction_class}"
                }
            }
        }
        div {
            class: "split {direction_class}",
            onmounted: move |evt| {
                on_resize.mount(evt);
            },
            div {
                class: "split-pane",
                flex: split.location,
                TabLayoutComponent {
                    layout: split.a.as_ref(),
                    tab_drop_offer: tab_drop_offer.clone(),
                    dragged_tab: dragged_tab.clone(),
                    bus: bus.clone()
                }
            }
            div {
                class: "splitter {dragging_class}",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        let pos = match split.direction {
                            TabSplitDirection::Vertical => evt.client_coordinates().x,
                            TabSplitDirection::Horizontal => evt.client_coordinates().y,
                        };
                        drag_pos.set(Some(SplitDragPosition {
                            start_split: split.location,
                            start_mousepos: pos,
                            current_mousepos: pos
                        }));
                    }
                },
            }
            div {
                class: "split-pane",
                flex: 1.0 - split.location,
                TabLayoutComponent {
                    layout: split.b.as_ref(),
                    tab_drop_offer: tab_drop_offer.clone(),
                    dragged_tab: dragged_tab.clone(),
                    bus: bus.clone()
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabGroupComponent<'a>(
    cx: Scoped,
    group: &'a TabGroup,
    #[props(!optional)] tab_drop_offer: Option<TabDropInExistingGroup>,
    #[props(!optional)] dragged_tab: Option<u32>,
    bus: CommandBus,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "group",
            div {
                class: "group-header",
                for (index, tab) in group.tabs.iter().enumerate() {
                    rsx! {
                        if let Some(offer) = tab_drop_offer {
                            if offer.group_id == group.group_id && offer.index == index {
                                rsx! {
                                    div {
                                        class: "tab-drop-offer"
                                    }
                                }
                            } else {
                                rsx! { "" }
                            }
                        }
                        /*
                        div {
                            class: "tab-drop-offer"
                        }
                         */
                        TabHeaderComponent {
                            key: "{tab.tab_id}",
                            group_id: group.group_id,
                            index: index,
                            tab: tab.clone(),
                            dragged_tab: dragged_tab.clone(),
                            bus: bus.clone()
                        }
                    }
                }
                if let Some(offer) = tab_drop_offer {
                    if offer.group_id == group.group_id && offer.index >= group.tabs.len() {
                        rsx! {
                            div {
                                class: "tab-drop-offer"
                            }
                        }
                    } else {
                        rsx! { "" }
                    }
                }
            }
            if let Some(tab) = group.tabs.iter().find(|tab| tab.active_in_group) {
                rsx! {
                    div {
                        class: "active-content",
                        "Active tab {tab.title}"
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "no-active-tab",
                        p {
                            "Click a tab to open it"
                        }
                    }
                }
            }
        }
    })
}

#[derive(Debug)]
enum TabDragState {
    Standby {
        element_offset: (f64, f64),
        client_start_pos: (f64, f64),
    },
    Dragging {
        element_offset: (f64, f64),
        client_start_pos: (f64, f64),
        client_current_pos: (f64, f64),
    },
}

#[allow(non_snake_case)]
#[inline_props]
fn TabHeaderComponent(
    cx: Scoped,
    group_id: u32,
    index: usize,
    tab: TabProps,
    #[props(!optional)] dragged_tab: Option<u32>,
    bus: CommandBus,
) -> Element {
    const DRAG_TRIGGER_DIST: f64 = 5.0;

    let size = use_ref(cx, ComponentSize::default);
    let drag_state = use_state(cx, || -> Option<TabDragState> { None });
    //let offer_tab_drop = use_state(cx, || None);

    let on_resize = use_state(cx, || {
        to_owned![size];
        OnResize::new(move |new_size: ComponentSize| size.set(new_size))
    });

    use_window_mouseup(cx, (drag_state, bus), |(drag_state, bus)| {
        move |_| {
            drag_state.set(None);
            bus.send_blocking(ConfigCommand::StopDraggingTab);
        }
    });

    use_window_mousemove(cx, (drag_state, tab, bus), |(drag_state, tab, bus)| {
        move |evt| {
            if let Some(ref state) = *drag_state.current() {
                if evt.held_buttons().contains(MouseButton::Primary) {
                    let client_current_pos = evt.client_coordinates().to_tuple();
                    match state {
                        TabDragState::Standby {
                            element_offset,
                            client_start_pos,
                        } => {
                            // Distance mouse has traveled from mousedown by Pythagorean theorem
                            let dist = ((client_current_pos.0 - client_start_pos.0).powi(2)
                                + (client_current_pos.1 - client_start_pos.1).powi(2))
                            .sqrt();

                            if dist > DRAG_TRIGGER_DIST {
                                drag_state.set(Some(TabDragState::Dragging {
                                    element_offset: *element_offset,
                                    client_start_pos: client_current_pos,
                                    client_current_pos: client_current_pos,
                                }));
                                bus.send_blocking(ConfigCommand::StartDraggingTab {
                                    tab_id: tab.tab_id,
                                });
                            }
                        }
                        TabDragState::Dragging {
                            element_offset,
                            client_start_pos,
                            ..
                        } => {
                            {
                                let win = web_sys::window().unwrap();
                                let doc = win.document().unwrap();
                                let elements = doc.elements_from_point(
                                    client_current_pos.0 as f32,
                                    client_current_pos.1 as f32,
                                );

                                let mut found_overlay = false;
                                let mut element_under_cursor = None;
                                for el in elements.iter() {
                                    let el = match web_sys::Element::try_from(el) {
                                        Ok(el) => el,
                                        Err(_) => {
                                            continue;
                                        }
                                    };

                                    if !found_overlay {
                                        if el.id() == "tab-drag-overlay" {
                                            found_overlay = true;
                                        }
                                        continue;
                                    }

                                    element_under_cursor = Some(el);
                                    break;
                                }

                                if let Some(element_under_cursor) = element_under_cursor {
                                    let event = web_sys::DragEvent::new_with_event_init_dict(
                                        "dragover",
                                        DragEventInit::new()
                                            .bubbles(true)
                                            .cancelable(true)
                                            .view(Some(&win))
                                            .client_x(evt.client_coordinates().x as i32)
                                            .client_y(evt.client_coordinates().y as i32)
                                            .screen_x(evt.screen_coordinates().x as i32)
                                            .screen_y(evt.screen_coordinates().y as i32)
                                            .buttons(1),
                                    )
                                    .unwrap();

                                    element_under_cursor.dispatch_event(&event).unwrap();
                                }
                            }

                            drag_state.set(Some(TabDragState::Dragging {
                                element_offset: *element_offset,
                                client_start_pos: *client_start_pos,
                                client_current_pos,
                            }));
                        }
                    }
                } else {
                    drag_state.set(None);
                    bus.send_blocking(ConfigCommand::StopDraggingTab);
                }
            }
        }
    });

    let (is_dragging, pos) = match *drag_state.current() {
        Some(ref drag_state) => match drag_state {
            TabDragState::Dragging {
                element_offset,
                client_current_pos,
                ..
            } => (
                true,
                Some((
                    client_current_pos.0 - element_offset.0,
                    client_current_pos.1 - element_offset.1,
                )),
            ),
            _ => (false, None),
        },
        None => (false, None),
    };

    cx.render(rsx! {
        TabHeaderComponentInner {
            title: tab.title.clone(),
            active_in_group: tab.active_in_group,
            absolute_pos: None,
            opacity: 1.0,
            onmousedown: move |evt: Event<MouseData>| {
                let client_coords = evt.client_coordinates().to_tuple();
                let element_coords = evt.element_coordinates().to_tuple();
                drag_state.set(Some(TabDragState::Standby {
                    element_offset: element_coords,
                    client_start_pos: client_coords
                }));
                bus.send_blocking(ConfigCommand::Layout(LayoutCommand::SetActiveInGroup {
                    group_id: *group_id,
                    tab_id: tab.tab_id
                }));
            },
            ondragover: move |evt: Event<DragData>| {
                if let Some(dragged_tab) = dragged_tab {
                    if *dragged_tab != tab.tab_id {
                        let drop_index = if evt.mouse.element_coordinates().x < size.read().width / 2.0 {
                            *index
                        } else {
                            index + 1
                        };

                        bus.send_blocking(ConfigCommand::OfferTabDropInExistingGroup(TabDropInExistingGroup {
                            group_id: *group_id,
                            index: drop_index
                        }));
                    }
                }
            },
            onmounted: move |evt| {
                on_resize.mount(evt);
            },
            if is_dragging {
                rsx! {
                    div {
                        id: "tab-drag-overlay",
                        class: "tab-drag-overlay",
                        TabHeaderComponentInner {
                            title: tab.title.clone(),
                            active_in_group: true,
                            absolute_pos: pos,
                            opacity: 0.8,
                            onmousedown: |_| {},
                            ondragover: |_| {},
                            onmounted: |_| {},
                        }
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabHeaderComponentInner<'a>(
    cx: Scoped,
    title: String,
    active_in_group: bool,
    #[props(!optional)] absolute_pos: Option<(f64, f64)>,
    opacity: f64,
    onmousedown: EventHandler<'a, Event<MouseData>>,
    ondragover: EventHandler<'a, Event<DragData>>,
    onmounted: EventHandler<'a, Event<MountedData>>,
    children: Element<'a>,
) -> Element {
    let active_in_group_class = match active_in_group {
        true => "active-in-group",
        false => "",
    };

    let (position_attr, left_attr, top_attr) = match absolute_pos {
        Some(pos) => ("absolute", pos.0, pos.1),
        None => ("static", 0.0, 0.0),
    };

    let div = cx.render(rsx! {
        div {
            class: "tab-header {active_in_group_class}",
            onmousedown: |evt| { onmousedown.call(evt) },
            ondragover: |evt| { ondragover.call(evt) },
            onmounted: |evt| { onmounted.call(evt) },
            position: position_attr,
            left: "{left_attr}px",
            top: "{top_attr}px",
            opacity: "{opacity}",
            div {
                class: "tab-icon",
                "Ϣ"
            }
            div {
                class: "tab-title",
                "{title}"
            }
            div {
                class: "tab-close",
                onclick: move |evt| {
                    log::debug!("CLOSE");
                    evt.stop_propagation();
                },
                onmousedown: move |evt| {
                    evt.stop_propagation();
                },
                "✕"
            }
            children
        }
    });

    div
}
