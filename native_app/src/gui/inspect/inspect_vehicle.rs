use crate::gui::inspect::{entity_link, follow_button};
use crate::i18n::I18n;
use crate::uiworld::UiWorld;
use goryak::{minrow, on_secondary_container, textc, Window};
use simulation::transportation::VehicleState;
use simulation::{Simulation, VehicleID};
use yakui::widgets::Pad;

pub fn inspect_vehicle(uiworld: &UiWorld, sim: &Simulation, id: VehicleID) -> bool {
    let i18n = uiworld.read::<I18n>();
    let Some(v) = sim.get(id) else {
        return false;
    };

    let name = format!("{:?}", v.vehicle.kind);

    let mut is_open = true;
    Window {
        title: name.into(),
        pad: Pad::all(10.0),
        radius: 10.0,
        opened: &mut is_open,
        child_spacing: 5.0,
    }
    .show(|| {
        if cfg!(debug_assertions) {
            textc(on_secondary_container(), format!("{:?}", id));
        }

        match v.vehicle.state {
            VehicleState::Parked(_) => {
                textc(
                    on_secondary_container(),
                    i18n.tr("ui.inspect.vehicle.parked").to_string(),
                );
            }
            VehicleState::Driving => {
                textc(
                    on_secondary_container(),
                    i18n.tr_args(
                        "ui.inspect.vehicle.driving_speed",
                        &[("value", format!("{:.0}", v.speed.0 * 3.6))],
                    ),
                );
            }
            VehicleState::Panicking(_) => {
                textc(
                    on_secondary_container(),
                    i18n.tr("ui.inspect.vehicle.panicking").to_string(),
                );
            }
            VehicleState::RoadToPark(_, _, _) => {
                textc(
                    on_secondary_container(),
                    i18n.tr("ui.inspect.vehicle.parking").to_string(),
                );
            }
        }

        for (human_id, human) in &sim.world().humans {
            if human.router.personal_car == Some(id) {
                minrow(5.0, || {
                    textc(
                        on_secondary_container(),
                        i18n.tr("ui.inspect.vehicle.owned_by").to_string(),
                    );
                    entity_link(uiworld, sim, human_id);
                });
            }
        }

        follow_button(uiworld, id);
    });

    is_open
}
