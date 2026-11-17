use std::{rc::Rc, sync::Arc};

use floem::{
    View,
    event::EventListener,
    keyboard::Modifiers,
    reactive::{ReadSignal, RwSignal, Scope, SignalGet, SignalUpdate},
    style::{CursorStyle, Display, Position},
    views::{Decorators, container, label, stack, svg},
};
use lapce_core::{command::FocusCommand, meta::VERSION, mode::Mode};

use crate::{
    command::{CommandExecuted, CommandKind, InternalCommand},
    config::{LapceConfig, color::LapceColor},
    keypress::KeyPressFocus,
    listener::Listener,
    web_link::web_link,
    window_tab::{Focus, WindowTabData},
};

const APP_NAME: &str = "Antares IDE";
const APP_TAGLINE: &str = "Octomind × Lapce — a fast, native code editor";

struct AboutUri;

impl AboutUri {
    const GITHUB: &'static str = "https://github.com/dhapidpad/AntaresIDE";
    const DISCORD: &'static str = "https://discord.gg/n8tGJ6Rn6D";
}

#[derive(Clone, Debug)]
pub struct AboutData {
    pub visible: RwSignal<bool>,
    pub focus: RwSignal<Focus>,
}

impl AboutData {
    pub fn new(cx: Scope, focus: RwSignal<Focus>) -> Self {
        Self {
            visible: cx.create_rw_signal(false),
            focus,
        }
    }

    pub fn open(&self) {
        self.visible.set(true);
        self.focus.set(Focus::AboutPopup);
    }

    pub fn close(&self) {
        self.visible.set(false);
        self.focus.set(Focus::Workbench);
    }
}

impl KeyPressFocus for AboutData {
    fn get_mode(&self) -> Mode {
        Mode::Insert
    }

    fn check_condition(
        &self,
        _condition: crate::keypress::condition::Condition,
    ) -> bool {
        self.visible.get_untracked()
    }

    fn run_command(
        &self,
        command: &crate::command::LapceCommand,
        _count: Option<usize>,
        _mods: Modifiers,
    ) -> CommandExecuted {
        if let CommandKind::Focus(FocusCommand::ModalClose) = &command.kind {
            self.close();
        }
        CommandExecuted::Yes
    }

    fn receive_char(&self, _c: &str) {}

    fn focus_only(&self) -> bool {
        true
    }
}


fn link_item(
    text: &'static str,
    uri: &'static str,
    config: ReadSignal<Arc<LapceConfig>>,
    internal_command: Listener<InternalCommand>,
) -> impl View {
    web_link(
        move || text.to_string(),
        move || uri.to_string(),
        move || config.get().color(LapceColor::EDITOR_LINK),
        internal_command,
    )
    .style(|s| s.margin_horiz(10.0))
}

fn section_title(
    text: &'static str,
    config: ReadSignal<Arc<LapceConfig>>,
) -> impl View {
    label(move || text.to_uppercase()).style(move |s| {
        s.font_bold()
            .font_size(11.0)
            .margin_top(28.0)
            .margin_bottom(10.0)
            .color(config.get().color(LapceColor::EDITOR_DIM))
    })
}

fn divider(config: ReadSignal<Arc<LapceConfig>>) -> impl View {
    container(label(|| String::new())).style(move |s| {
        s.width(220.0)
            .height(1.0)
            .margin_top(24.0)
            .background(config.get().color(LapceColor::LAPCE_BORDER))
    })
}

// ───────────────────────── About popup ─────────────────────────

pub fn about_popup(window_tab_data: Rc<WindowTabData>) -> impl View {
    let about_data = window_tab_data.about_data.clone();
    let config = window_tab_data.common.config;
    let internal_command = window_tab_data.common.internal_command;
    let logo_size = 96.0;

    let close_data = about_data.clone();
    exclusive_popup(
        window_tab_data,
        about_data.visible,
        move || close_data.close(),
        move || {
            stack((
                // Header
                svg(move || config.get().logo_svg()).style(move |s| {
                    s.size(logo_size, logo_size)
                        .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                }),
                label(|| APP_NAME.to_string()).style(move |s| {
                    s.font_bold()
                        .font_size(22.0)
                        .margin_top(14.0)
                        .color(config.get().color(LapceColor::EDITOR_FOREGROUND))
                }),
                label(|| APP_TAGLINE.to_string()).style(move |s| {
                    s.font_size(12.0)
                        .margin_top(4.0)
                        .color(config.get().color(LapceColor::EDITOR_DIM))
                }),
                label(|| format!("v{}", VERSION)).style(move |s| {
                    let config = config.get();
                    s.font_size(11.0)
                        .margin_top(12.0)
                        .padding_vert(2.0)
                        .padding_horiz(10.0)
                        .border(1.0)
                        .border_radius(10.0)
                        .border_color(config.color(LapceColor::LAPCE_BORDER))
                        .color(config.color(LapceColor::EDITOR_DIM))
                }),
                // Project
                divider(config),
                section_title("Project", config),
                link_item("GitHub", AboutUri::GITHUB, config, internal_command),
                // Upstream & community
                section_title("Built on Lapce", config),
                stack((
                    link_item("Website", AboutUri::LAPCE, config, internal_command),
                    link_item("Discord", AboutUri::DISCORD, config, internal_command),
                    link_item("Matrix", AboutUri::MATRIX, config, internal_command),
                ))
                .style(|s| s.flex_row().items_center()),
                // Attributions
                section_title("Attributions", config),
                link_item(
                    "Codicons (CC-BY-4.0)",
                    AboutUri::CODICONS,
                    config,
                    internal_command,
                ),
                // Footer hint
                label(|| "Press Esc to close".to_string()).style(move |s| {
                    s.font_size(11.0)
                        .margin_top(28.0)
                        .color(config.get().color(LapceColor::EDITOR_DIM))
                }),
            ))
            .style(|s| s.flex_col().items_center())
        },
    )
    .debug_name("About Popup")
}

// ───────────────────────── Popup shell ─────────────────────────

fn exclusive_popup<V: View + 'static>(
    window_tab_data: Rc<WindowTabData>,
    visibility: RwSignal<bool>,
    on_close: impl Fn() + 'static,
    content: impl FnOnce() -> V,
) -> impl View {
    let config = window_tab_data.common.config;

    container(
        container(
            container(content())
                .style(move |s| {
                    let config = config.get();
                    s.padding_vert(32.0)
                        .padding_horiz(72.0)
                        .min_width(380.0)
                        .border(1.0)
                        .border_radius(10.0)
                        .border_color(config.color(LapceColor::LAPCE_BORDER))
                        .background(config.color(LapceColor::PANEL_BACKGROUND))
                })
                // Click inside the card must not close the popup.
                .on_event_stop(EventListener::PointerDown, |_| {}),
        )
        .style(|s| {
            s.flex_grow(1.0f32)
                .flex_row()
                .items_center()
                .hover(|s| s.cursor(CursorStyle::Default))
        }),
    )
    // Click on the dimmed backdrop closes the popup.
    .on_event_stop(EventListener::PointerDown, move |_| on_close())
    // Block hover for everything behind the backdrop.
    .on_event_stop(EventListener::PointerMove, |_| {})
    .style(move |s| {
        s.display(if visibility.get() {
            Display::Flex
        } else {
            Display::None
        })
        .position(Position::Absolute)
        .size_pct(100.0, 100.0)
        .flex_col()
        .items_center()
        .background(
            config
                .get()
                .color(LapceColor::LAPCE_DROPDOWN_SHADOW)
                .multiply_alpha(0.6),
        )
    })
}