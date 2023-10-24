use eframe::{
    egui::{
        self,
        style::{Selection, WidgetVisuals, Widgets},
    },
    epaint::{Color32, Rounding, Shadow, Stroke},
};

pub const VISUALS: egui::Visuals = egui::Visuals {
    dark_mode: true,
    override_text_color: None,
    widgets: Widgets {
        noninteractive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(27),
            bg_fill: Color32::from_gray(27),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32::from_gray(60),
            }, // separators, indentation lines
            fg_stroke: Stroke {
                width: 1.0,
                color: Color32::from_gray(140),
            }, // normal text color
            rounding: Rounding::ZERO,
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(60), // button background
            bg_fill: Color32::from_gray(60),      // checkbox background
            bg_stroke: Stroke {
                width: 0.0,
                color: Color32::from_rgb_additive(0, 0, 0),
            },
            fg_stroke: Stroke {
                width: 1.0,
                color: Color32::from_gray(180),
            }, // button text
            rounding: Rounding::ZERO,
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(70),
            bg_fill: Color32::from_gray(70),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32::from_gray(150),
            }, // e.g. hover over window edge or button
            fg_stroke: Stroke {
                width: 1.5,
                color: Color32::from_gray(240),
            },
            rounding: Rounding::ZERO,
            expansion: 1.0,
        },
        active: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(55),
            bg_fill: Color32::from_gray(55),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32::WHITE,
            },
            fg_stroke: Stroke {
                width: 2.0,
                color: Color32::WHITE,
            },
            rounding: Rounding::ZERO,
            expansion: 1.0,
        },
        open: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(27),
            bg_fill: Color32::from_gray(27),
            bg_stroke: Stroke {
                width: 1.0,
                color: Color32::from_gray(60),
            },
            fg_stroke: Stroke {
                width: 1.0,
                color: Color32::from_gray(210),
            },
            rounding: Rounding::ZERO,
            expansion: 0.0,
        },
    },
    selection: Selection {
        bg_fill: Color32::from_rgb(0, 92, 128),
        stroke: Stroke {
            width: 1.0,
            color: Color32::from_rgb(192, 222, 255),
        },
    },
    hyperlink_color: Color32::from_rgb(90, 170, 255),
    faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
    extreme_bg_color: Color32::from_gray(10),            // e.g. TextEdit background
    code_bg_color: Color32::from_gray(64),
    warn_fg_color: Color32::from_rgb(255, 143, 0), // orange
    error_fg_color: Color32::from_rgb(255, 0, 0),  // red

    window_rounding: Rounding::ZERO,
    window_shadow: Shadow {
        extrusion: 0.0,
        color: Color32::TRANSPARENT,
    },
    window_fill: Color32::from_gray(27),
    window_stroke: Stroke {
        width: 0.0,
        color: Color32::TRANSPARENT,
    },

    menu_rounding: Rounding::ZERO,
    panel_fill: Color32::from_gray(27),

    popup_shadow: Shadow {
        extrusion: 5.0,
        color: Color32::from_black_alpha(64),
    },
    resize_corner_size: 12.0,
    text_cursor: Stroke {
        width: 2.0,
        color: Color32::from_rgb(128, 128, 128),
    },
    text_cursor_preview: false,
    clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
    button_frame: true,
    collapsing_header_frame: false,
    indent_has_left_vline: true,

    striped: false,

    slider_trailing_fill: false,

    interact_cursor: None,

    image_loading_spinners: true,
};
