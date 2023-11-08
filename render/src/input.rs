use space::{point2, vec2, Point2, Vec2};

#[derive(Debug, Clone)]
pub enum InputEvent {
    ModifiersChanged(Modifiers),
    CursorMoved(Point2),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseWheel(Vec2),
    Unhandled,
}
#[cfg(feature = "winit")]
impl From<winit::event::WindowEvent> for InputEvent {
    fn from(event: winit::event::WindowEvent) -> Self {
        match event {
            winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                Self::ModifiersChanged(modifiers.into())
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                Self::CursorMoved(point2(position.x, position.y))
            }
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => vec2(x as f64, y as f64),
                    winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                        x,
                        y,
                    }) => vec2(x, y),
                };
                Self::MouseWheel(delta)
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let evt_button = match button {
                    winit::event::MouseButton::Left => MouseButton::Primary,
                    winit::event::MouseButton::Right => MouseButton::Secondary,
                    winit::event::MouseButton::Middle => MouseButton::Aux,
                    _ => return Self::Unhandled,
                };

                match state.is_pressed() {
                    true => Self::MouseDown(evt_button),
                    false => Self::MouseUp(evt_button),
                }
            }
            _ => Self::Unhandled,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    // Main mouse button, usually left
    Primary,

    // Secondary mouse button, usually left
    Secondary,

    // Auxiliary mouse button, usually middle or mousewheel button
    Aux,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modifiers {
    pub l_ctrl: bool,
    pub r_ctrl: bool,
    pub l_shift: bool,
    pub r_shift: bool,
    pub l_alt: bool,
    pub r_alt: bool,
}
impl Modifiers {
    pub fn ctrl(&self) -> bool {
        self.l_ctrl || self.r_ctrl
    }

    pub fn alt(&self) -> bool {
        self.l_alt || self.r_alt
    }

    pub fn shift(&self) -> bool {
        self.l_shift || self.r_shift
    }
}
#[cfg(feature = "winit")]
impl From<winit::event::Modifiers> for Modifiers {
    fn from(modifiers: winit::event::Modifiers) -> Self {
        Self {
            l_ctrl: modifiers.lcontrol_state() == winit::keyboard::ModifiersKeyState::Pressed,
            r_ctrl: modifiers.rcontrol_state() == winit::keyboard::ModifiersKeyState::Pressed,
            l_shift: modifiers.lshift_state() == winit::keyboard::ModifiersKeyState::Pressed,
            r_shift: modifiers.rshift_state() == winit::keyboard::ModifiersKeyState::Pressed,
            l_alt: modifiers.lalt_state() == winit::keyboard::ModifiersKeyState::Pressed,
            r_alt: modifiers.ralt_state() == winit::keyboard::ModifiersKeyState::Pressed,
        }
    }
}
