use std::sync::atomic::Ordering;
use std::time::Instant;

use yakui::widgets::{List, Pad};
use yakui::{column, opaque, reflow, spacer, Alignment, CrossAxisAlignment, Dim2, Pivot};

use goryak::{
    blur_bg, button_primary, button_secondary, constrained_viewport, on_primary_container,
    on_secondary_container, padxy, secondary_container, textc, Window,
};
use simulation::economy::Government;
use simulation::Simulation;

use crate::gui::{ExitState, GuiState};
use crate::i18n::I18n;
use crate::inputmap::{InputAction, InputMap};
use crate::uiworld::{SaveLoadState, UiWorld};

pub fn menu_bar(uiworld: &UiWorld, sim: &Simulation) {
    profiling::scope!("hud::menu_bar");
    let i18n = uiworld.read::<I18n>();

    reflow(Alignment::TOP_LEFT, Pivot::TOP_LEFT, Dim2::ZERO, || {
        constrained_viewport(|| {
            column(|| {
                opaque(|| {
                    blur_bg(secondary_container().with_alpha(0.5), 0.0, || {
                        padxy(5.0, 5.0, || {
                            let mut l = List::row();
                            l.item_spacing = 10.0;
                            l.cross_axis_alignment = CrossAxisAlignment::Center;

                            l.show(|| {
                                let mut gui = uiworld.write::<GuiState>();
                                gui.windows.menu(uiworld);
                                save_window(&mut gui, uiworld);
                                textc(
                                    on_primary_container(),
                                    i18n.tr_args(
                                        "ui.menu.money",
                                        &[("value", format!("{}", sim.read::<Government>().money))],
                                    ),
                                );
                            });
                        });
                    });
                });
                spacer(1);
            });
        });
    });
}

fn save_window(gui: &mut GuiState, uiw: &UiWorld) {
    let i18n = uiw.read::<I18n>();
    let mut slstate = uiw.write::<SaveLoadState>();
    if slstate.saving_status.load(Ordering::SeqCst) {
        textc(
            on_secondary_container(),
            i18n.tr("ui.menu.saving").to_string(),
        );
    } else if button_primary(i18n.tr("ui.menu.save")).show().clicked {
        slstate.please_save = true;
        gui.last_save = Instant::now();
        uiw.save_to_disk();
    }

    let mut estate = uiw.write::<ExitState>();

    match *estate {
        ExitState::NoExit => {}
        ExitState::ExitAsk | ExitState::Saving => {
            let mut opened = true;
            Window {
                title: i18n.tr("ui.menu.exit_menu").to_string().into(),
                pad: Pad::all(15.0),
                radius: 10.0,
                opened: &mut opened,
                child_spacing: 5.0,
            }
            .show(|| {
                if let ExitState::Saving = *estate {
                    textc(
                        on_secondary_container(),
                        i18n.tr("ui.menu.saving").to_string(),
                    );
                    if !slstate.please_save && !slstate.saving_status.load(Ordering::SeqCst) {
                        std::process::exit(0);
                    }
                    return;
                }
                if button_secondary(i18n.tr("ui.menu.save_exit")).show().clicked {
                    if let ExitState::ExitAsk = *estate {
                        slstate.please_save = true;
                        *estate = ExitState::Saving;
                    }
                }
                if button_secondary(i18n.tr("ui.menu.exit_no_save")).show().clicked {
                    std::process::exit(0);
                }
                if button_secondary(i18n.tr("ui.menu.cancel")).show().clicked {
                    *estate = ExitState::NoExit;
                }
            });

            if !opened {
                *estate = ExitState::NoExit;
            }

            if uiw
                .read::<InputMap>()
                .just_act
                .contains(&InputAction::Close)
            {
                *estate = ExitState::NoExit;
            }
        }
    }

    match *estate {
        ExitState::NoExit => {
            if button_secondary(i18n.tr("ui.menu.exit")).show().clicked {
                *estate = ExitState::ExitAsk;
            }
        }
        ExitState::ExitAsk => {
            if button_secondary(i18n.tr("ui.menu.save_exit")).show().clicked {
                if let ExitState::ExitAsk = *estate {
                    slstate.please_save = true;
                    *estate = ExitState::Saving;
                }
            }
        }
        ExitState::Saving => {
            textc(
                on_secondary_container(),
                i18n.tr("ui.menu.saving").to_string(),
            );
        }
    }
}
