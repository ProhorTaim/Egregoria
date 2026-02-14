use std::time::{Duration, Instant};

use yakui::widgets::{CountGrid, List, Pad};
use yakui::{
    constrained, divider, Constraints, CrossAxisAlignment, MainAxisAlignItems, MainAxisSize, Vec2,
};

use common::saveload::Encoder;
use engine::GfxSettings;
use engine::ShadowQuality;
use goryak::{
    button_primary, button_secondary, checkbox_value, combo_box, dragvalue, icon_button, minrow,
    on_secondary_container, outline, padx, padxy, textc, VertScrollSize, Window,
};
use serde::{Deserialize, Serialize};
use simulation::Simulation;

use crate::game_loop::Timings;
use crate::gui::keybinds::{KeybindState, KeybindStateInner};
use crate::i18n::{I18n, Language};
use crate::inputmap::{Bindings, InputMap};
use crate::uiworld::UiWorld;

const BINDINGS_SAVE_NAME: &str = "bindings";

const SETTINGS_SAVE_NAME: &str = "settings";

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub camera_border_move: bool,
    pub camera_smooth: bool,
    pub camera_smooth_tightness: f32,
    pub camera_fov: f32,

    pub gfx: GfxSettings,

    pub gui_scale: f32,
    pub language: Language,
    pub low_dpi_mode: bool,

    pub master_volume_percent: f32,
    pub music_volume_percent: f32,
    pub effects_volume_percent: f32,
    pub ui_volume_percent: f32,

    #[serde(skip)]
    pub time_warp: u32,
    pub auto_save_every: AutoSaveEvery,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_border_move: false,
            camera_smooth: true,
            master_volume_percent: 100.0,
            music_volume_percent: 100.0,
            effects_volume_percent: 100.0,
            ui_volume_percent: 100.0,
            time_warp: 1,
            auto_save_every: AutoSaveEvery::FiveMinutes,
            camera_smooth_tightness: 1.0,
            camera_fov: 60.0,
            gui_scale: 1.0,
            gfx: GfxSettings::default(),
            language: Language::English,
            low_dpi_mode: false,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoSaveEvery {
    Never = 0,
    OneMinute = 1,
    FiveMinutes = 2,
}

impl From<AutoSaveEvery> for Option<Duration> {
    fn from(x: AutoSaveEvery) -> Option<Duration> {
        match x {
            AutoSaveEvery::Never => None,
            AutoSaveEvery::OneMinute => Some(Duration::from_secs(60)),
            AutoSaveEvery::FiveMinutes => Some(Duration::from_secs(5 * 60)),
        }
    }
}

impl From<u8> for AutoSaveEvery {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Never,
            1 => Self::OneMinute,
            2 => Self::FiveMinutes,
            _ => Self::Never,
        }
    }
}

impl AsRef<str> for AutoSaveEvery {
    fn as_ref(&self) -> &str {
        match self {
            AutoSaveEvery::Never => "Never",
            AutoSaveEvery::OneMinute => "Minute",
            AutoSaveEvery::FiveMinutes => "Five Minutes",
        }
    }
}

pub struct SettingsState {
    fps: f32,
    ms: f32,
    instant: Instant,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            fps: 0.0,
            ms: 0.0,
            instant: Instant::now(),
        }
    }
}

pub fn settings(uiw: &UiWorld, _: &Simulation, opened: &mut bool) {
    Window {
        title: uiw.read::<I18n>().tr("ui.settings.title").into(),
        pad: Pad::all(10.0),
        radius: 10.0,
        opened,
        child_spacing: 0.0,
    }
    .show(|| {
        profiling::scope!("gui::window::settings");

        VertScrollSize::Percent(0.8).show(|| {
            let mut l = List::column();
            l.item_spacing = 5.0;
            l.main_axis_size = MainAxisSize::Min;
            l.show(|| {
                let mut settings = uiw.write::<Settings>();
                let mut state = uiw.write::<SettingsState>();
                let before = *settings;
                let i18n = uiw.read::<I18n>();

                textc(on_secondary_container(), i18n.tr("ui.settings.gameplay"));
                minrow(5.0, || {
                    textc(on_secondary_container(), i18n.tr("ui.settings.autosave"));
                    let mut id = settings.auto_save_every as u8 as usize;
                    let auto_never = i18n.tr("ui.settings.autosave.never");
                    let auto_one = i18n.tr("ui.settings.autosave.one_min");
                    let auto_five = i18n.tr("ui.settings.autosave.five_min");
                    let items = [auto_never.as_str(), auto_one.as_str(), auto_five.as_str()];
                    if combo_box(&mut id, &items, 200.0) {
                        settings.auto_save_every = AutoSaveEvery::from(id as u8);
                    }
                });

                divider(outline(), 10.0, 1.0);
                textc(on_secondary_container(), i18n.tr("ui.settings.input"));
                checkbox_value(
                    &mut settings.camera_border_move,
                    on_secondary_container(),
                    i18n.tr("ui.settings.camera_border_move"),
                );
                checkbox_value(
                    &mut settings.camera_smooth,
                    on_secondary_container(),
                    i18n.tr("ui.settings.camera_smooth"),
                );

                if settings.camera_smooth {
                    minrow(5.0, || {
                        dragvalue()
                            .min(0.1)
                            .max(2.0)
                            .step(0.1)
                            .show(&mut settings.camera_smooth_tightness);
                        textc(
                            on_secondary_container(),
                            i18n.tr("ui.settings.camera_smooth_tightness"),
                        );
                    });
                }

                minrow(5.0, || {
                    dragvalue()
                        .min(2.0)
                        .max(179.0)
                        .step(1.0)
                        .show(&mut settings.camera_fov);
                    textc(on_secondary_container(), i18n.tr("ui.settings.camera_fov"));
                });

                if state.fps == 0.0 || state.instant.elapsed() > Duration::from_millis(300) {
                    state.ms = uiw.read::<Timings>().all.avg();
                    state.fps = 1.0 / state.ms;
                    state.instant = Instant::now();
                }

                divider(outline(), 10.0, 1.0);
                #[cfg(debug_assertions)]
                textc(
                    on_secondary_container(),
                    i18n.tr("ui.settings.debug_fps_warning"),
                );
                textc(
                    on_secondary_container(),
                    i18n.tr_args(
                        "ui.settings.graphics_stats",
                        &[
                            ("fps", format!("{:.1}", state.fps)),
                            ("ms", format!("{:.1}", 1000.0 * state.ms)),
                        ],
                    ),
                );
                checkbox_value(
                    &mut settings.gfx.fullscreen,
                    on_secondary_container(),
                    i18n.tr("ui.settings.fullscreen"),
                );
                checkbox_value(
                    &mut settings.low_dpi_mode,
                    on_secondary_container(),
                    "Low resolution mode (macOS, restart required)",
                );
                checkbox_value(
                    &mut settings.gfx.terrain_grid,
                    on_secondary_container(),
                    i18n.tr("ui.settings.terrain_grid"),
                );
                checkbox_value(
                    &mut settings.gfx.fog,
                    on_secondary_container(),
                    i18n.tr("ui.settings.fog"),
                );
                checkbox_value(
                    &mut settings.gfx.ssao,
                    on_secondary_container(),
                    i18n.tr("ui.settings.ssao"),
                );
                checkbox_value(
                    &mut settings.gfx.msaa,
                    on_secondary_container(),
                    i18n.tr("ui.settings.msaa"),
                );
                checkbox_value(
                    &mut settings.gfx.vsync,
                    on_secondary_container(),
                    i18n.tr("ui.settings.vsync"),
                );
                checkbox_value(
                    &mut settings.gfx.parallel_render,
                    on_secondary_container(),
                    i18n.tr("ui.settings.threaded_render"),
                );

                minrow(5.0, || {
                    let mut id = settings.gfx.shadows as u8 as usize;
                    let s_none = i18n.tr("ui.settings.shadow.none");
                    let s_low = i18n.tr("ui.settings.shadow.low");
                    let s_medium = i18n.tr("ui.settings.shadow.medium");
                    let s_high = i18n.tr("ui.settings.shadow.high");
                    let s_ultra = i18n.tr("ui.settings.shadow.ultra");
                    let items = [
                        s_none.as_str(),
                        s_low.as_str(),
                        s_medium.as_str(),
                        s_high.as_str(),
                        s_ultra.as_str(),
                    ];
                    if combo_box(&mut id, &items, 200.0) {
                        settings.gfx.shadows = ShadowQuality::from(id as u8);
                    }
                    textc(
                        on_secondary_container(),
                        i18n.tr("ui.settings.shadow_quality"),
                    );
                });

                divider(outline(), 10.0, 1.0);
                textc(on_secondary_container(), i18n.tr("ui.settings.gui"));
                minrow(5.0, || {
                    dragvalue().min(0.5).max(2.0).show(&mut settings.gui_scale);
                    textc(on_secondary_container(), i18n.tr("ui.settings.gui_scale"));
                });

                divider(outline(), 10.0, 1.0);
                textc(on_secondary_container(), i18n.tr("ui.settings.audio"));
                minrow(5.0, || {
                    dragvalue()
                        .min(0.0)
                        .max(100.0)
                        .step(1.0)
                        .show(&mut settings.master_volume_percent);
                    textc(
                        on_secondary_container(),
                        i18n.tr("ui.settings.master_volume"),
                    );
                });

                minrow(5.0, || {
                    dragvalue()
                        .min(0.0)
                        .max(100.0)
                        .step(1.0)
                        .show(&mut settings.music_volume_percent);
                    textc(
                        on_secondary_container(),
                        i18n.tr("ui.settings.music_volume"),
                    );
                });

                minrow(5.0, || {
                    dragvalue()
                        .min(0.0)
                        .max(100.0)
                        .step(1.0)
                        .show(&mut settings.effects_volume_percent);
                    textc(
                        on_secondary_container(),
                        i18n.tr("ui.settings.effects_volume"),
                    );
                });

                minrow(5.0, || {
                    dragvalue()
                        .min(0.0)
                        .max(100.0)
                        .step(1.0)
                        .show(&mut settings.ui_volume_percent);
                    textc(on_secondary_container(), i18n.tr("ui.settings.ui_volume"));
                });

                divider(outline(), 10.0, 1.0);
                textc(on_secondary_container(), i18n.tr("ui.settings.keybinds"));
                let mut bindings = uiw.write::<Bindings>();
                if button_primary(i18n.tr("ui.settings.reset")).show().clicked {
                    *bindings = Bindings::default();
                    uiw.write::<InputMap>().build_input_tree(&mut bindings);

                    common::saveload::JSONPretty::save_silent(&*bindings, BINDINGS_SAVE_NAME);
                }

                let mut sorted_inps = bindings.0.keys().cloned().collect::<Vec<_>>();
                sorted_inps.sort();

                constrained(
                    Constraints::loose(Vec2::new(f32::INFINITY, 100000.0)),
                    || {
                        CountGrid::col(4)
                            .main_axis_size(MainAxisSize::Min)
                            .cross_axis_aligment(CrossAxisAlignment::Start)
                            .main_axis_align_items(MainAxisAlignItems::Center)
                            .show(|| {
                                for action in &sorted_inps {
                                    padx(2.0, || {
                                        textc(on_secondary_container(), action.to_string());
                                    });

                                    padx(2.0, || {
                                        minrow(0.0, || {
                                            let label = {
                                                let comb = bindings.0.get(action).unwrap();
                                                if comb.0.len() > 0 {
                                                    format!("{}", comb.0[0])
                                                } else {
                                                    i18n.tr("ui.settings.empty").to_string()
                                                }
                                            };
                                            let resp = button_primary(label).show();
                                            if resp.clicked {
                                                let mut state = uiw.write::<KeybindState>();
                                                state.enabled = Some(KeybindStateInner {
                                                    to_bind_to: action.clone(),
                                                    cur: Default::default(),
                                                    bind_index: 0,
                                                });
                                            }
                                        });
                                    });

                                    padx(2.0, || {
                                        minrow(0.0, || {
                                            let label = {
                                                let comb = bindings.0.get(action).unwrap();
                                                if comb.0.len() > 1 {
                                                    format!("{}", comb.0[1])
                                                } else {
                                                    i18n.tr("ui.settings.empty").to_string()
                                                }
                                            };
                                            let resp = button_primary(label).show();
                                            if resp.clicked {
                                                let mut state = uiw.write::<KeybindState>();
                                                state.enabled = Some(KeybindStateInner {
                                                    to_bind_to: action.clone(),
                                                    cur: Default::default(),
                                                    bind_index: 1,
                                                });
                                            }
                                        });
                                    });

                                    padxy(8.0, 2.0, || {
                                        minrow(0.0, || {
                                            if icon_button(button_primary("arrows-rotate")).show().clicked {
                                                if let Some(comb_mut) = bindings.0.get_mut(action) {
                                                    comb_mut.0 = Bindings::default().0.remove(action).unwrap().0;
                                                }
                                                uiw.write::<InputMap>().build_input_tree(&mut bindings);
                                                common::saveload::JSONPretty::save_silent(&*bindings, BINDINGS_SAVE_NAME);
                                            }
                                        });
                                    });
                                }
                            });
                    },
                );
                drop(bindings);

                divider(outline(), 10.0, 1.0);
                textc(on_secondary_container(), i18n.tr("ui.settings.language"));
                minrow(5.0, || {
                    let lang_en = i18n.tr("ui.settings.language.english");
                    let lang_ru = i18n.tr("ui.settings.language.russian");

                    if button_secondary(lang_en).show().clicked {
                        settings.language = Language::English;
                    }

                    if button_secondary(lang_ru).show().clicked {
                        settings.language = Language::Russian;
                    }
                });
                drop(i18n);

                let settings_changed = *settings != before;
                if settings_changed {
                    common::saveload::JSONPretty::save_silent(&*settings, SETTINGS_SAVE_NAME);
                }
                drop(state);
                drop(settings);
            });
        });
    })
}

pub fn manage_settings(ctx: &mut engine::Context, settings: &Settings) {
    ctx.gfx.update_settings(settings.gfx);
    if (ctx.yakui.zoom_factor - settings.gui_scale).abs() > 0.001 {
        ctx.yakui.zoom_factor = settings.gui_scale;
    }
    if (ctx.egui.zoom_factor - settings.gui_scale).abs() > 0.001 {
        ctx.egui.zoom_factor = settings.gui_scale;
    }

    ctx.audio.set_settings(
        settings.master_volume_percent,
        settings.ui_volume_percent,
        settings.music_volume_percent,
        settings.effects_volume_percent,
    );
}
