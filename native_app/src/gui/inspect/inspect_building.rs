use goryak::{
    dragvalue, fixed_spacer, minrow, on_secondary_container, primary, textc, ProgressBar, Window,
};
use prototypes::{ItemID, Recipe};
use simulation::economy::Market;
use simulation::map::{Building, BuildingID, BuildingKind, Zone, MAX_ZONE_AREA};
use simulation::map_dynamic::{BuildingInfos, ElectricityFlow};
use simulation::souls::freight_station::FreightTrainState;
use simulation::world_command::WorldCommand;
use simulation::{Simulation, SoulID};
use yakui::widgets::Pad;
use yakui::Vec2;

use crate::gui::inspect::entity_link;
use crate::gui::item_icon_yakui;
use crate::i18n::I18n;
use crate::uiworld::UiWorld;

fn label(x: impl Into<String>) {
    textc(on_secondary_container(), x.into());
}

/// Inspect a specific building, showing useful information about it
pub fn inspect_building(uiworld: &UiWorld, sim: &Simulation, id: BuildingID) -> bool {
    let i18n = uiworld.read::<I18n>();
    let map = sim.map();
    let Some(building) = map.buildings().get(id) else {
        return false;
    };

    let title = match building.kind {
        BuildingKind::House => i18n.tr("ui.inspect.house").to_string(),
        BuildingKind::GoodsCompany(id) => i18n.proto_label(
            "goods_company",
            &id.prototype().name,
            &id.prototype().label,
        ),
        BuildingKind::RailFreightStation(id) => i18n.proto_label(
            "freight_station",
            &id.prototype().name,
            &id.prototype().label,
        ),
        BuildingKind::TrainStation => i18n.tr("ui.inspect.train_station").to_string(),
        BuildingKind::ExternalTrading => i18n.tr("ui.inspect.external_trading").to_string(),
    };

    let mut is_open = true;
    Window {
        title: title.into(),
        pad: Pad::all(10.0),
        radius: 10.0,
        opened: &mut is_open,
        child_spacing: 5.0,
    }
    .show(|| {
        if cfg!(debug_assertions) {
            label(format!("{:?}", building.id));
        }

        match building.kind {
            BuildingKind::House => render_house(uiworld, sim, building),
            BuildingKind::GoodsCompany(_) => {
                render_goodscompany(uiworld, sim, building);
            }
            BuildingKind::RailFreightStation(_) => {
                render_freightstation(uiworld, sim, building);
            }
            BuildingKind::TrainStation => {}
            BuildingKind::ExternalTrading => {}
        };

        if let Some(ref zone) = building.zone {
            let mut cpy = zone.filldir;
            minrow(5.0, || {
                let mut ang = cpy.angle_cossin().to_degrees();

                if dragvalue().min(-180.0).max(180.0).show(&mut ang.0) {
                    cpy = ang.to_radians().vec2();
                    uiworld.commands().push(WorldCommand::UpdateZone {
                        building: id,
                        zone: Zone {
                            filldir: cpy,
                            ..zone.clone()
                        },
                    })
                }

                label(i18n.tr("ui.inspect.fill_angle"));
            });

            ProgressBar {
                value: zone.area / MAX_ZONE_AREA,
                size: Vec2::new(200.0, 25.0),
                color: primary().adjust(0.7),
            }
            .show_children(|| {
                label(i18n.tr_args(
                    "ui.inspect.zone_area",
                    &[
                        ("value", format!("{:.0}", zone.area)),
                        ("max", format!("{:.0}", MAX_ZONE_AREA)),
                    ],
                ));
            });
        }
    });

    is_open
}

fn render_house(uiworld: &UiWorld, sim: &Simulation, b: &Building) {
    let i18n = uiworld.read::<I18n>();
    let binfos = sim.read::<BuildingInfos>();
    let Some(info) = binfos.get(b.id) else {
        return;
    };
    let Some(SoulID::Human(owner)) = info.owner else {
        return;
    };

    minrow(5.0, || {
        label(i18n.tr("ui.inspect.owner"));
        entity_link(uiworld, sim, owner);
    });

    label(i18n.tr("ui.inspect.currently_in_house"));
    for &soul in info.inside.iter() {
        let SoulID::Human(soul) = soul else {
            continue;
        };
        entity_link(uiworld, sim, soul);
    }
}

fn render_freightstation(uiworld: &UiWorld, sim: &Simulation, b: &Building) {
    let i18n = uiworld.read::<I18n>();
    let Some(SoulID::FreightStation(owner)) = sim.read::<BuildingInfos>().owner(b.id) else {
        return;
    };
    let Some(freight) = sim.world().get(owner) else {
        return;
    };

    label(i18n.tr_args(
        "ui.inspect.waiting_cargo",
        &[("value", format!("{}", freight.f.waiting_cargo))],
    ));
    label(i18n.tr_args(
        "ui.inspect.wanted_cargo",
        &[("value", format!("{}", freight.f.wanted_cargo))],
    ));

    fixed_spacer((0.0, 10.0));
    label(i18n.tr("ui.inspect.trains"));
    for (tid, state) in &freight.f.trains {
        minrow(5.0, || {
            entity_link(uiworld, sim, *tid);
            match state {
                FreightTrainState::Arriving => {
                    label(i18n.tr("ui.inspect.train_arriving"));
                }
                FreightTrainState::Loading => {
                    label(i18n.tr("ui.inspect.train_loading"));
                }
                FreightTrainState::Moving => {
                    label(i18n.tr("ui.inspect.train_moving"));
                }
            }
        });
    }
}

fn render_goodscompany(uiworld: &UiWorld, sim: &Simulation, b: &Building) {
    let i18n = uiworld.read::<I18n>();
    let owner = sim.read::<BuildingInfos>().owner(b.id);

    let Some(SoulID::GoodsCompany(c_id)) = owner else {
        return;
    };
    let Some(c) = sim.world().companies.get(c_id) else {
        return;
    };
    let goods = &c.comp;
    let workers = &c.workers;
    let proto = c.comp.proto.prototype();

    let market = &*sim.read::<Market>();
    let map = &*sim.map();
    let elec_flow = &*sim.read::<ElectricityFlow>();

    let max_workers = goods.max_workers;
    ProgressBar {
        value: workers.0.len() as f32 / max_workers as f32,
        size: Vec2::new(200.0, 25.0),
        color: primary().adjust(0.7),
    }
    .show_children(|| {
        label(i18n.tr_args(
            "ui.inspect.workers",
            &[
                ("value", format!("{}", workers.0.len())),
                ("max", format!("{}", max_workers)),
            ],
        ));
    });

    if let Some(driver) = goods.driver {
        minrow(5.0, || {
            label(i18n.tr("ui.inspect.driver_is"));
            entity_link(uiworld, sim, driver);
        });
    }
    let productivity = c.productivity(proto, b.zone.as_ref(), map, elec_flow);
    if productivity < 1.0 {
        ProgressBar {
            value: productivity,
            size: Vec2::new(200.0, 25.0),
            color: primary().adjust(0.7),
        }
        .show_children(|| {
            label(i18n.tr_args(
                "ui.inspect.productivity",
                &[(
                    "value",
                    format!("{:.0}", (productivity * 100.0).round()),
                )],
            ));
        });
    }

    if let Some(ref r) = proto.recipe {
        render_recipe(uiworld, r);
    }

    if let Some(net_id) = map.electricity.net_id(b.id) {
        let blackout = elec_flow.blackout(net_id);

        if let Some(power_c) = proto.power_consumption {
            ProgressBar {
                value: productivity,
                size: Vec2::new(200.0, 25.0),
                color: primary().adjust(0.7),
            }
            .show_children(|| {
                label(i18n.tr_args(
                    "ui.inspect.power",
                    &[
                        ("value", format!("{}", productivity as f64 * power_c)),
                        ("max", format!("{}", power_c)),
                    ],
                ));
            });
        }

        if let Some(power_prod) = proto.power_production {
            label(i18n.tr_args(
                "ui.inspect.power_producing",
                &[("value", format!("{}", power_prod * productivity as f64))],
            ));

            let stats = elec_flow.network_stats(net_id);

            ProgressBar {
                value: if blackout { 0.0 } else { 1.0 },
                size: Vec2::new(200.0, 25.0),
                color: primary().adjust(0.7),
            }
            .show_children(|| {
                label(i18n.tr_args(
                    "ui.inspect.network_health",
                    &[
                        ("prod", format!("{}", stats.produced_power)),
                        ("cons", format!("{}", stats.consumed_power)),
                        (
                            "pct",
                            format!(
                                "{:.0}",
                                (100 * stats.produced_power.0) / stats.consumed_power.0.max(1)
                            ),
                        ),
                    ],
                ));
            });
        }
    }

    ProgressBar {
        value: goods.progress,
        size: Vec2::new(200.0, 25.0),
        color: primary().adjust(0.7),
    }
    .show_children(|| {
        label(i18n.tr_args(
            "ui.inspect.progress_pct",
            &[("value", format!("{:.0}", goods.progress * 100.0))],
        ));
    });

    fixed_spacer((0.0, 10.0));
    label(i18n.tr("ui.inspect.storage"));

    let jobopening = ItemID::new("job-opening");
    for (&id, m) in market.iter() {
        let Some(v) = m.capital(c_id.into()) else {
            continue;
        };
        if id == jobopening && v == 0 {
            continue;
        }

        item_icon_yakui(uiworld, id, v);
    }
}

fn render_recipe(uiworld: &UiWorld, recipe: &Recipe) {
    let i18n = uiworld.read::<I18n>();
    if recipe.consumption.is_empty() {
        label(i18n.tr("ui.inspect.no_inputs"));
    } else {
        label(if recipe.consumption.len() == 1 {
            i18n.tr("ui.inspect.input")
        } else {
            i18n.tr("ui.inspect.inputs")
        });
        minrow(5.0, || {
            for item in recipe.consumption.iter() {
                item_icon_yakui(uiworld, item.id, item.amount);
            }
        });
    }

    if recipe.production.is_empty() {
        label(i18n.tr("ui.inspect.no_outputs"));
    } else {
        label(if recipe.production.len() == 1 {
            i18n.tr("ui.inspect.output")
        } else {
            i18n.tr("ui.inspect.outputs")
        });
        minrow(5.0, || {
            for item in recipe.production.iter() {
                item_icon_yakui(uiworld, item.id, item.amount);
            }
        });
    }
}
