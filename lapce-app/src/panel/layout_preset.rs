use serde::{Deserialize, Serialize};

use super::{
    data::{PanelOrder, PanelSize, default_panel_order},
    kind::PanelKind,
    position::PanelPosition,
    style::PanelStyle,
};

/// Predefined workbench layout configurations per PRODUCT-SPEC §3.2 and §10.2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutPreset {
    /// Standard coding layout: Primary Sidebar (FileExplorer) open, Bottom & Secondary hidden.
    Coding,
    /// Agentic pairing layout: Explorer on Left, Agents panel on Secondary Sidebar (Right), Terminal on Bottom.
    Agentic,
    /// Code review layout: Source Control on Left, Problem / Diagnostics on Bottom.
    Review,
    /// Debugging layout: Debug panel on Left, Terminal / Debug Console on Bottom.
    Debug,
    /// Zen / distraction-free layout: All sidebars and bottom panels hidden, 100% editor focus.
    Minimal,
    /// Clean presentation layout: All panels collapsed for demos and screen sharing.
    Presentation,
}

impl LayoutPreset {
    pub const ALL: [Self; 6] = [
        Self::Coding,
        Self::Agentic,
        Self::Review,
        Self::Debug,
        Self::Minimal,
        Self::Presentation,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Coding => "Coding",
            Self::Agentic => "Agentic",
            Self::Review => "Review",
            Self::Debug => "Debug",
            Self::Minimal => "Minimal (Zen Mode)",
            Self::Presentation => "Presentation",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Coding => "Primary file explorer open with full editor center",
            Self::Agentic => "AI agent paired on secondary sidebar with active terminal",
            Self::Review => "Source control and git changes with diagnostic panel",
            Self::Debug => "Debug controls and terminal session",
            Self::Minimal => "Distraction-free zen view with all panels collapsed",
            Self::Presentation => "Clean presentation view optimized for screen sharing",
        }
    }

    /// Pure function applying this layout preset to the panel order, styles, and sizes.
    pub fn apply(
        &self,
        order: &PanelOrder,
        styles: &mut im::HashMap<PanelPosition, PanelStyle>,
        size: &mut PanelSize,
    ) {
        match self {
            Self::Coding => {
                size.left = 280.0;
                size.bottom = 240.0;
                size.right = 340.0;

                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftTop,
                    true,
                    Some(PanelKind::FileExplorer),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftBottom,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomLeft,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomRight,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightTop,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightBottom,
                    false,
                    None,
                );
            }
            Self::Agentic => {
                size.left = 260.0;
                size.bottom = 220.0;
                size.right = 380.0;

                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftTop,
                    true,
                    Some(PanelKind::FileExplorer),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftBottom,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomLeft,
                    true,
                    Some(PanelKind::Terminal),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomRight,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightTop,
                    true,
                    Some(PanelKind::Antares),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightBottom,
                    false,
                    None,
                );
            }
            Self::Review => {
                size.left = 300.0;
                size.bottom = 240.0;
                size.right = 340.0;

                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftTop,
                    true,
                    Some(PanelKind::SourceControl),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftBottom,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomLeft,
                    true,
                    Some(PanelKind::Problem),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomRight,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightTop,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightBottom,
                    false,
                    None,
                );
            }
            Self::Debug => {
                size.left = 300.0;
                size.bottom = 260.0;
                size.right = 340.0;

                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftTop,
                    true,
                    Some(PanelKind::Debug),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::LeftBottom,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomLeft,
                    true,
                    Some(PanelKind::Terminal),
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::BottomRight,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightTop,
                    false,
                    None,
                );
                set_position_state(
                    order,
                    styles,
                    PanelPosition::RightBottom,
                    false,
                    None,
                );
            }
            Self::Minimal | Self::Presentation => {
                size.left = 280.0;
                size.bottom = 240.0;
                size.right = 340.0;

                for pos in [
                    PanelPosition::LeftTop,
                    PanelPosition::LeftBottom,
                    PanelPosition::BottomLeft,
                    PanelPosition::BottomRight,
                    PanelPosition::RightTop,
                    PanelPosition::RightBottom,
                ] {
                    set_position_state(order, styles, pos, false, None);
                }
            }
        }
    }
}

/// Restores default panel positions, styles, and sizes per PRODUCT-SPEC §10.2.
pub fn reset_ui_layout_state(
    order: &mut PanelOrder,
    styles: &mut im::HashMap<PanelPosition, PanelStyle>,
    size: &mut PanelSize,
) {
    *order = default_panel_order();
    styles.clear();

    styles.insert(
        PanelPosition::LeftTop,
        PanelStyle {
            active: 0,
            shown: true,
            maximized: false,
        },
    );
    styles.insert(
        PanelPosition::LeftBottom,
        PanelStyle {
            active: 0,
            shown: false,
            maximized: false,
        },
    );
    styles.insert(
        PanelPosition::BottomLeft,
        PanelStyle {
            active: 0,
            shown: true,
            maximized: false,
        },
    );
    styles.insert(
        PanelPosition::BottomRight,
        PanelStyle {
            active: 0,
            shown: false,
            maximized: false,
        },
    );
    styles.insert(
        PanelPosition::RightTop,
        PanelStyle {
            active: 1, // Antares is at index 1
            shown: false,
            maximized: false,
        },
    );
    styles.insert(
        PanelPosition::RightBottom,
        PanelStyle {
            active: 0,
            shown: false,
            maximized: false,
        },
    );

    *size = PanelSize::default();
}

fn set_position_state(
    order: &PanelOrder,
    styles: &mut im::HashMap<PanelPosition, PanelStyle>,
    pos: PanelPosition,
    shown: bool,
    preferred_kind: Option<PanelKind>,
) {
    let style = styles.entry(pos).or_default();
    style.shown = shown;
    style.maximized = false;
    if shown {
        if let Some(kind) = preferred_kind {
            if let Some(panels) = order.get(&pos) {
                if let Some(idx) = panels.iter().position(|k| *k == kind) {
                    style.active = idx;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_layout_presets_apply_cleanly() {
        let order = default_panel_order();
        let mut styles = im::HashMap::new();
        let mut size = PanelSize::default();

        for preset in LayoutPreset::ALL {
            preset.apply(&order, &mut styles, &mut size);
            assert!(!preset.name().is_empty());
            assert!(!preset.description().is_empty());

            match preset {
                LayoutPreset::Coding => {
                    assert!(styles.get(&PanelPosition::LeftTop).unwrap().shown);
                    assert!(!styles.get(&PanelPosition::BottomLeft).unwrap().shown);
                    assert!(!styles.get(&PanelPosition::RightTop).unwrap().shown);
                    assert_eq!(size.left, 280.0);
                }
                LayoutPreset::Agentic => {
                    assert!(styles.get(&PanelPosition::LeftTop).unwrap().shown);
                    assert!(styles.get(&PanelPosition::BottomLeft).unwrap().shown);
                    assert!(styles.get(&PanelPosition::RightTop).unwrap().shown);
                    assert_eq!(size.right, 380.0);
                }
                LayoutPreset::Review => {
                    assert!(styles.get(&PanelPosition::LeftTop).unwrap().shown);
                    assert!(styles.get(&PanelPosition::BottomLeft).unwrap().shown);
                    assert!(!styles.get(&PanelPosition::RightTop).unwrap().shown);
                    assert_eq!(size.left, 300.0);
                }
                LayoutPreset::Debug => {
                    assert!(styles.get(&PanelPosition::LeftTop).unwrap().shown);
                    assert!(styles.get(&PanelPosition::BottomLeft).unwrap().shown);
                    assert!(!styles.get(&PanelPosition::RightTop).unwrap().shown);
                    assert_eq!(size.bottom, 260.0);
                }
                LayoutPreset::Minimal | LayoutPreset::Presentation => {
                    assert!(!styles.get(&PanelPosition::LeftTop).unwrap().shown);
                    assert!(!styles.get(&PanelPosition::BottomLeft).unwrap().shown);
                    assert!(!styles.get(&PanelPosition::RightTop).unwrap().shown);
                }
            }
        }
    }

    #[test]
    fn test_reset_ui_layout_restores_defaults() {
        let mut order = PanelOrder::new();
        let mut styles = im::HashMap::new();
        let mut size = PanelSize {
            left: 50.0,
            left_split: 0.1,
            bottom: 50.0,
            bottom_split: 0.1,
            right: 50.0,
            right_split: 0.1,
        };

        reset_ui_layout_state(&mut order, &mut styles, &mut size);

        assert_eq!(order, default_panel_order());
        assert_eq!(size.left, 280.0);
        assert_eq!(size.bottom, 240.0);
        assert_eq!(size.right, 340.0);

        assert!(styles.get(&PanelPosition::LeftTop).unwrap().shown);
        assert!(styles.get(&PanelPosition::BottomLeft).unwrap().shown);
        assert!(!styles.get(&PanelPosition::RightTop).unwrap().shown);
    }

    #[test]
    fn test_layout_preset_serde_roundtrip() {
        for preset in LayoutPreset::ALL {
            let json = serde_json::to_string(&preset).unwrap();
            let parsed: LayoutPreset = serde_json::from_str(&json).unwrap();
            assert_eq!(preset, parsed);
        }
    }

    #[test]
    fn test_workbench_layout_commands_serialize() {
        use std::str::FromStr;
        use crate::command::LapceWorkbenchCommand;

        let commands = [
            ("apply_layout_coding", LapceWorkbenchCommand::ApplyLayoutCoding),
            ("apply_layout_agentic", LapceWorkbenchCommand::ApplyLayoutAgentic),
            ("apply_layout_review", LapceWorkbenchCommand::ApplyLayoutReview),
            ("apply_layout_debug", LapceWorkbenchCommand::ApplyLayoutDebug),
            ("apply_layout_minimal", LapceWorkbenchCommand::ApplyLayoutMinimal),
            ("apply_layout_presentation", LapceWorkbenchCommand::ApplyLayoutPresentation),
            ("reset_ui_layout", LapceWorkbenchCommand::ResetUiLayout),
            ("toggle_antares_agent_focus", LapceWorkbenchCommand::ToggleAntaresAgentFocus),
            ("toggle_antares_agent_visual", LapceWorkbenchCommand::ToggleAntaresAgentVisual),
        ];

        for (serialized, cmd) in commands {
            assert_eq!(cmd.to_string(), serialized);
            assert_eq!(LapceWorkbenchCommand::from_str(serialized).unwrap(), cmd);
        }
    }
}
