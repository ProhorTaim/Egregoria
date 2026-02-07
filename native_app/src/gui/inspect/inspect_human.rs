use goryak::{dragvalue, fixed_spacer, minrow, on_secondary_container, textc, Window};
use prototypes::ItemID;
use yakui::widgets::Pad;

use simulation::economy::Market;
use simulation::map_dynamic::Destination;
use simulation::souls::desire::WorkKind;
use simulation::transportation::Location;
use simulation::{HumanID, Simulation};

use crate::gui::inspect::{building_link, follow_button};
use crate::gui::item_icon_yakui;
use crate::i18n::I18n;
use crate::uiworld::UiWorld;

/// Inspect a specific building, showing useful information about it
pub fn inspect_human(uiworld: &UiWorld, sim: &Simulation, id: HumanID) -> bool {
    let i18n = uiworld.read::<I18n>();
    let Some(human) = sim.get(id) else {
        return false;
    };

    let pinfo = &human.personal_info;
    let title = format!("{}{:?} • {}", pinfo.age, pinfo.gender, pinfo.name);

    let mut is_open = true;

    fn label(x: impl Into<String>) {
        textc(on_secondary_container(), x.into());
    }

    Window {
        title: title.into(),
        pad: Pad::all(10.0),
        radius: 10.0,
        opened: &mut is_open,
        child_spacing: 5.0,
    }
    .show(|| {
        if cfg!(debug_assertions) {
            label(format!("{:?}", id));
        }

        match human.location {
            Location::Outside => {}
            Location::Vehicle(_) => {
                label(i18n.tr("ui.inspect.human.in_vehicle"));
            }
            Location::Building(x) => {
                minrow(5.0, || {
                    label(i18n.tr("ui.inspect.human.in_building"));
                    building_link(uiworld, sim, x);
                });
            }
        }

        if let Some(ref dest) = human.router.target_dest {
            match dest {
                Destination::Outside(pos) => {
                    label(i18n.tr_args(
                        "ui.inspect.human.going_to",
                        &[("value", format!("{}", pos))],
                    ));
                }
                Destination::Building(b) => {
                    minrow(5.0, || {
                        label(i18n.tr("ui.inspect.human.going_to_building"));
                        building_link(uiworld, sim, *b);
                    });
                }
            }
        }

        minrow(5.0, || {
            label(i18n.tr("ui.inspect.human.house_is"));
            building_link(uiworld, sim, human.home.house);
        });

        label(i18n.tr_args(
            "ui.inspect.human.last_ate",
            &[("value", format!("{}", human.food.last_ate))],
        ));

        if let Some(ref x) = human.work {
            minrow(5.0, || {
                label(i18n.tr("ui.inspect.human.working_at"));
                building_link(uiworld, sim, x.workplace);
                match x.kind {
                    WorkKind::Driver { .. } => {
                        label(i18n.tr("ui.inspect.human.as_driver"));
                    }
                    WorkKind::Worker => {
                        label(i18n.tr("ui.inspect.human.as_worker"));
                    }
                }
            });
        }

        fixed_spacer((0.0, 10.0));
        label(i18n.tr("ui.inspect.human.desires"));
        minrow(5.0, || {
            let mut score = human.food.last_score;
            dragvalue().show(&mut score);
            label(i18n.tr("ui.inspect.human.food"));
        });
        minrow(5.0, || {
            let mut score = human.home.last_score;
            dragvalue().show(&mut score);
            label(i18n.tr("ui.inspect.human.home"));
        });
        minrow(5.0, || {
            let mut score = human.work.as_ref().map(|x| x.last_score).unwrap_or(0.0);
            dragvalue().show(&mut score);
            label(i18n.tr("ui.inspect.human.work"));
        });

        let market = sim.read::<Market>();

        fixed_spacer((0.0, 10.0));

        let jobopening = ItemID::new("job-opening");
        for (&item_id, m) in market.iter() {
            let Some(v) = m.capital(id.into()) else {
                continue;
            };
            if item_id == jobopening {
                continue;
            }

            item_icon_yakui(uiworld, item_id, v);
        }

        follow_button(uiworld, id);
    });
    is_open
}
