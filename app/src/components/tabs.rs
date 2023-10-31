use crate::{
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use async_channel::Sender;
use dioxus::{html::input_data::MouseButton, prelude::*};
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
    TabLayout::VSplit(TabVSplit {
        layout_id: id,
        split,
        left: Box::new(left),
        right: Box::new(right),
    })
}

pub fn hsplit(split: f64, top: TabLayout, bottom: TabLayout) -> TabLayout {
    TabLayout::HSplit(TabHSplit {
        split,
        top: Box::new(top),
        bottom: Box::new(bottom),
    })
}

#[derive(Clone, PartialEq, Debug)]
pub enum TabLayout {
    Group(TabGroup),
    VSplit(TabVSplit),
    HSplit(TabHSplit),
}
impl TabLayout {
    fn modify(&self, command: &TabLayoutCommand) -> Self {
        match command {
            TabLayoutCommand::Nop => self.clone(),
            TabLayoutCommand::OnAdjustVSplit {
                layout_id,
                new_split,
            } => self.apply_adjust_vsplit(*layout_id, *new_split),
        }
    }

    fn apply_adjust_vsplit(&self, target_layout_id: u32, new_split: f64) -> Self {
        match self {
            TabLayout::VSplit(TabVSplit {
                layout_id,
                left,
                right,
                ..
            }) if *layout_id == target_layout_id => TabLayout::VSplit(TabVSplit {
                layout_id: *layout_id,
                split: new_split,
                left: left.clone(),
                right: right.clone(),
            }),
            other => other.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabGroup {
    tabs: Vec<TabProps>,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabVSplit {
    layout_id: u32,
    split: f64,
    left: Box<TabLayout>,
    right: Box<TabLayout>,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabHSplit {
    split: f64,
    top: Box<TabLayout>,
    bottom: Box<TabLayout>,
}

#[derive(PartialEq, Debug, Clone, Props)]
pub struct TabProps {
    tab_id: u32,
}

#[derive(Debug, PartialEq)]
enum TabLayoutCommand {
    Nop,
    OnAdjustVSplit { layout_id: u32, new_split: f64 },
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
        //log::debug!("process {:?}", command);
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
        TabLayout::Group(group) => {
            //
            cx.render(rsx! {
                TabGroupComponent { group: group }
            })
        }
        TabLayout::VSplit(vsplit) => cx.render(rsx! {
            TabVSplitComponent { vsplit: vsplit.clone(), bus: bus.clone() }
        }),
        TabLayout::HSplit(hsplit) => cx.render(rsx! {
            TabHSplitComponent { hsplit: hsplit.clone(), bus: bus.clone() }
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
fn TabVSplitComponent(cx: Scoped, vsplit: TabVSplit, bus: CommandBus) -> Element<'a> {
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
        (drag_pos, size, vsplit, bus),
        |(drag_pos, size, vsplit, bus)| {
            move |evt| {
                if let Some(ref pos) = *drag_pos.current() {
                    if evt.held_buttons().contains(MouseButton::Primary) {
                        let new_drag_pos = pos.clone().with_current(evt.client_coordinates().x);
                        let new_split = new_drag_pos.adjust_split(size.read().width);
                        drag_pos.set(Some(new_drag_pos));
                        let command = TabLayoutCommand::OnAdjustVSplit {
                            layout_id: vsplit.layout_id,
                            new_split,
                        };

                        block_on(bus.send(command));
                    } else {
                        drag_pos.set(None);
                    }
                }
            }
        },
    );

    cx.render(rsx! {
        div {
            class: "vsplit",
            onmounted: move |evt| {
                on_resize.mount(evt);
            },
            div {
                class: "vsplit-pane vsplit-left",
                flex: vsplit.split,
                span {
                    "width: {size.read().width}, height: {size.read().height}"
                }
                TabLayoutComponent {
                    layout: vsplit.left.as_ref(),
                    bus: bus.clone()
                }
            }
            div {
                class: "splitter",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        let pos = evt.client_coordinates().x;
                        drag_pos.set(Some(DragPosition {
                            start_split: vsplit.split,
                            start_mousepos: pos,
                            current_mousepos: pos
                        }));
                    }
                },
            }
            div {
                class: "vsplit-pane vsplit-right",
                flex: 1.0 - vsplit.split,
                TabLayoutComponent {
                    layout: vsplit.right.as_ref(),
                    bus: bus.clone()
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabHSplitComponent(cx: Scoped, hsplit: TabHSplit, bus: CommandBus) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "hsplit",
            div {
                class: "hsplit-pane hsplit-top",
                flex: hsplit.split,
                TabLayoutComponent {
                    layout: hsplit.top.as_ref(),
                    bus: bus.clone()
                }
            }
            div {
                class: "splitter"
            }
            div {
                class: "hsplit-pane hsplit-bottom",
                flex: 1.0 - hsplit.split,
                TabLayoutComponent {
                    layout: hsplit.bottom.as_ref(),
                    bus: bus.clone()
                }
            }
        }
    })
}
