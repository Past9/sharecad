use crate::{
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use async_channel::Sender;
use dioxus::{core::AttributeValue, html::input_data::MouseButton, prelude::*};
use futures::executor::block_on;

struct IdSeries {
    id: u32,
}
impl IdSeries {
    pub fn new() -> Self {
        Self::seed(0)
    }

    pub fn seed(highest_current: u32) -> Self {
        Self {
            id: highest_current,
        }
    }

    pub fn next(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}

pub fn tab(id: u32) -> TabProps {
    TabProps { tab_id: id }
}

pub fn group<const N: usize>(tabs: [TabProps; N]) -> TabLayout {
    TabLayout::Group(TabGroup {
        tabs: tabs.to_vec(),
    })
}

pub fn vsplit(id: u32, split: f64, left: TabLayout, right: TabLayout) -> TabLayout {
    TabLayout::Split(TabSplit {
        kind: TabSplitKind::Vertical,
        layout_id: id,
        location: split,
        a: Box::new(left),
        b: Box::new(right),
    })
}

pub fn hsplit(id: u32, split: f64, top: TabLayout, bottom: TabLayout) -> TabLayout {
    TabLayout::Split(TabSplit {
        kind: TabSplitKind::Horizontal,
        layout_id: id,
        location: split,
        a: Box::new(top),
        b: Box::new(bottom),
    })
}

#[derive(Clone, PartialEq, Debug)]
pub enum TabLayout {
    Group(TabGroup),
    Split(TabSplit),
}
impl TabLayout {
    fn modify(&self, command: &TabLayoutCommand) -> Self {
        match command {
            TabLayoutCommand::OnAdjustSplit {
                layout_id,
                new_location,
            } => self.apply_adjust_split(*layout_id, *new_location),
        }
    }

    fn apply_adjust_split(&self, target_layout_id: u32, new_location: f64) -> Self {
        match self {
            TabLayout::Split(TabSplit {
                kind,
                layout_id,
                location,
                a,
                b,
            }) => {
                if *layout_id == target_layout_id {
                    TabLayout::Split(TabSplit {
                        kind: kind.clone(),
                        layout_id: *layout_id,
                        location: new_location,
                        a: a.clone(),
                        b: b.clone(),
                    })
                } else {
                    TabLayout::Split(TabSplit {
                        kind: kind.clone(),
                        layout_id: *layout_id,
                        location: location.clone(),
                        a: Box::new(a.apply_adjust_split(target_layout_id, new_location)),
                        b: Box::new(b.apply_adjust_split(target_layout_id, new_location)),
                    })
                }
            }
            other => other.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabGroup {
    tabs: Vec<TabProps>,
}

#[derive(Clone, Debug, PartialEq)]
enum TabSplitKind {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabSplit {
    kind: TabSplitKind,
    layout_id: u32,
    location: f64,
    a: Box<TabLayout>,
    b: Box<TabLayout>,
}

#[derive(PartialEq, Debug, Clone, Props)]
pub struct TabProps {
    tab_id: u32,
}

#[derive(Debug, PartialEq)]
enum TabLayoutCommand {
    OnAdjustSplit { layout_id: u32, new_location: f64 },
}

#[derive(Clone)]
struct CommandBus {
    sender: Sender<TabLayoutCommand>,
}
impl CommandBus {
    fn new(sender: Sender<TabLayoutCommand>) -> Self {
        Self { sender }
    }

    async fn send(&self, command: TabLayoutCommand) {
        self.sender.send(command).await.unwrap();
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
    layout: &'a TabLayout,
    on_layout_changed: EventHandler<'a, TabLayout>,
) -> Element {
    let (sender, receiver) = cx.use_hook(|| async_channel::unbounded::<TabLayoutCommand>());
    let bus = CommandBus::new(sender.clone());

    let next_command = use_state::<Option<TabLayoutCommand>>(cx, || None);

    let cr = {
        to_owned![next_command, receiver];
        use_coroutine(cx, |_rx: UnboundedReceiver<()>| async move {
            loop {
                if let Ok(command) = receiver.recv().await {
                    log::debug!("receive {:?}", command);
                    next_command.set(Some(command));
                }
            }
        })
    };

    if let Some(command) = &**next_command {
        next_command.set(None);
        on_layout_changed.call(layout.modify(command));
        cx.needs_update();
    }

    cx.render(rsx! {
        TabLayoutComponent {
            layout: layout,
            bus: bus
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabLayoutComponent<'a>(cx: Scoped, layout: &'a TabLayout, bus: CommandBus) -> Element {
    match layout {
        TabLayout::Group(group) => cx.render(rsx! {
            TabGroupComponent { group: group }
        }),
        TabLayout::Split(split) => cx.render(rsx! {
            TabSplitComponent { split: split.clone(), bus: bus.clone() }
        }),
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn TabComponent(cx: Scoped, tab: TabProps) -> Element {
    cx.render(rsx! {
        div {
            "Tab "
            "{tab.tab_id}"
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabGroupComponent<'a>(cx: Scoped, group: &'a TabGroup) -> Element<'a> {
    cx.render(rsx! {
        for tab in group.tabs.iter() {
            rsx! {
                TabComponent {
                    tab: tab.clone()
                }
            }
        }
    })
}

#[derive(Clone, Debug)]
struct DragPosition {
    start_split: f64,
    start_mousepos: f64,
    current_mousepos: f64,
}
impl DragPosition {
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
fn TabSplitComponent(cx: Scoped, split: TabSplit, bus: CommandBus) -> Element<'a> {
    let size = use_ref(cx, ComponentSize::default);
    let drag_pos = use_state(cx, || -> Option<DragPosition> { None });

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
                        let (position, space) = match split.kind {
                            TabSplitKind::Vertical => {
                                (evt.client_coordinates().x, size.read().width)
                            }
                            TabSplitKind::Horizontal => {
                                (evt.client_coordinates().y, size.read().height)
                            }
                        };

                        let new_drag_pos = pos.clone().with_current(position);
                        let new_location = new_drag_pos.adjust_split(space);
                        drag_pos.set(Some(new_drag_pos));
                        let command = TabLayoutCommand::OnAdjustSplit {
                            layout_id: split.layout_id,
                            new_location,
                        };

                        block_on(bus.send(command));
                    } else {
                        drag_pos.set(None);
                    }
                }
            }
        },
    );

    let direction_class = match split.kind {
        TabSplitKind::Vertical => "vertical",
        TabSplitKind::Horizontal => "horizontal",
    };

    let dragging_class = match drag_pos.is_some() {
        true => "dragging",
        false => "",
    };

    cx.render(rsx! {
        if drag_pos.is_some() {
            rsx! {
                div {
                    class: "overlay split-cursor"
                }
            }
        }
        div {
            class: "split {direction_class}",
            onmounted: move |evt| {
                on_resize.mount(evt);
            },
            div {
                class: "split-pane split-left",
                flex: split.location,
                TabLayoutComponent {
                    layout: split.a.as_ref(),
                    bus: bus.clone()
                }
            }
            div {
                class: "splitter {dragging_class}",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        let pos = match split.kind {
                            TabSplitKind::Vertical => evt.client_coordinates().x,
                            TabSplitKind::Horizontal => evt.client_coordinates().y,
                        };
                        drag_pos.set(Some(DragPosition {
                            start_split: split.location,
                            start_mousepos: pos,
                            current_mousepos: pos
                        }));
                    }
                },
            }
            div {
                class: "split-pane split-right",
                flex: 1.0 - split.location,
                TabLayoutComponent {
                    layout: split.b.as_ref(),
                    bus: bus.clone()
                }
            }
        }
    })
}
