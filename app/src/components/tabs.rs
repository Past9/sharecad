use std::process::Command;

use crate::{
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use async_channel::Sender;
use dioxus::{html::input_data::MouseButton, prelude::*};
use futures::executor::block_on;

//const FLEX_RESOLUTION: f32 = 10000.0;

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
    fn apply(&self, command: &TabLayoutCommand) -> Self {
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
                split,
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
    fn eq(&self, other: &Self) -> bool {
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
    let layout = (*layout).to_owned();

    let (sender, receiver) = async_channel::unbounded::<TabLayoutCommand>();
    let bus = CommandBus::new(sender);

    let next_command = use_state::<Option<TabLayoutCommand>>(cx, || None);

    let cr = {
        to_owned![next_command];
        use_coroutine(cx, |_rx: UnboundedReceiver<()>| async move {
            log::debug!("new coroutine");
            loop {
                log::debug!("loop {}", receiver.len());
                if let Ok(command) = receiver.recv().await {
                    log::debug!("got command");
                    next_command.set(Some(command));
                }
                log::debug!("past command");
            }

            /*
            loop {
                if let Ok(next) = receiver.recv().await {
                    log::debug!("layout: {:?}", next);
                    on_layout_changed.call(layout.apply(next));
                    /*
                    match next {
                        TabLayoutCommand::Nop => {}
                        TabLayoutCommand::OnAdjustVSplit {
                            layout_id,
                            new_split,
                        } => {
                            //
                        }
                    }
                     */
                }
            }
             */
        })
    };

    if let Some(command) = &**next_command {
        log::debug!("calling on_layout_changed");
        next_command.set(None);
        on_layout_changed.call(layout.apply(command));
        cx.needs_update();
        log::debug!("called on_layout_changed");
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
fn TabLayoutComponent(cx: Scoped, layout: TabLayout, bus: CommandBus) -> Element {
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

#[derive(Clone)]
struct DragPosition {
    start: f64,
    current: f64,
}
impl DragPosition {
    pub fn dist(&self) -> f64 {
        self.current - self.start
    }

    pub fn with_current(self, current: f64) -> Self {
        Self {
            start: self.start,
            current,
        }
    }

    pub fn split_dist(&self, size: f64) -> f64 {
        self.dist() / size
    }

    pub fn adjust_split(&self, size: f64, current_split: f64) -> f64 {
        (current_split + self.split_dist(size)).clamp(0.0, 1.0)
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn TabVSplitComponent(cx: Scoped, vsplit: TabVSplit, bus: CommandBus) -> Element<'a> {
    let size = use_ref(cx, ComponentSize::default);
    let drag_dist = use_state(cx, || -> Option<DragPosition> { None });

    let on_resize = use_state(cx, || {
        to_owned![size];
        OnResize::new(move |new_size: ComponentSize| size.set(new_size))
    });

    {
        to_owned![drag_dist];
        use_window_mouseup(cx, move |evt| {
            drag_dist.set(None);
            if evt.trigger_button() == Some(MouseButton::Primary) {
                drag_dist.set(None);
            }
            log::debug!("stop drag");
        });
    }

    {
        to_owned![drag_dist, size, vsplit, bus];
        use_window_mousemove(cx, move |evt| {
            if evt.held_buttons().contains(MouseButton::Primary) {
                if let Some(ref dist) = *drag_dist.current() {
                    //log::debug!("drag {:#?}", evt);
                    drag_dist.set(Some(dist.clone().with_current(evt.client_coordinates().x)));
                    let new_split = dist.adjust_split(size.read().width, vsplit.split);
                    log::debug!("new split {}", new_split);
                    block_on(bus.send(TabLayoutCommand::OnAdjustVSplit {
                        layout_id: vsplit.layout_id,
                        new_split,
                    }));
                    log::debug!("sent new split");
                    /*
                    on_adjust_vsplit.call(OnAdjustVSplit {
                        layout_id: 0, //vsplit.layout_id,
                        new_split,
                    });
                     */
                    //let x = on_adjust_vsplit;
                }
            } else {
                drag_dist.set(None);
            }
        });
    }

    cx.render(rsx! {
        div {
            class: "vsplit",
            onmounted: move |evt| {
                //log::debug!("onmounted {:#?}", evt.data.get_client_rect().await);
                on_resize.mount(evt);
            },
            div {
                class: "vsplit-pane vsplit-left",
                flex: vsplit.split,
                span {
                    "width: {size.read().width}, height: {size.read().height}"
                }
                TabLayoutComponent {
                    layout: *vsplit.left.clone(),
                    bus: bus.clone()
                }
            }
            div {
                class: "splitter",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        let pos = evt.client_coordinates().x;
                        drag_dist.set(Some(DragPosition {
                            start: pos,
                            current: pos
                        }));
                        //is_dragging.set(true);
                        log::debug!("start drag");
                    }
                },
            }
            div {
                class: "vsplit-pane vsplit-right",
                flex: 1.0 - vsplit.split,
                TabLayoutComponent {
                    layout: *vsplit.right.clone(),
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
                    layout: *hsplit.top.clone(),
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
                    layout: *hsplit.bottom.clone(),
                    bus: bus.clone()
                }
            }
        }
    })
}
