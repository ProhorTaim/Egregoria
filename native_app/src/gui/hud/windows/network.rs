use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use yakui::divider;
use yakui::widgets::Pad;

use common::saveload::Encoder;
use goryak::{
    button_primary, checkbox_value, error, on_secondary_container, outline, text_edit, textc,
    Window,
};
use simulation::Simulation;

use crate::i18n::I18n;
use crate::network::NetworkState;
use crate::uiworld::UiWorld;

#[derive(Default, Serialize, Deserialize)]
pub struct NetworkConnectionInfo {
    pub name: String,
    pub ip: String,
    #[serde(skip)]
    pub error: String,
    #[serde(skip)]
    show_hashes: bool,
    #[serde(skip)]
    hashes: BTreeMap<String, u64>,
    #[serde(skip)]
    hashes_tick: u64,
}

fn label(x: impl Into<String>) {
    textc(on_secondary_container(), x.into());
}

/// Network window
/// Allows to connect to a server or start a server
pub fn network(uiworld: &UiWorld, sim: &Simulation, opened: &mut bool) {
    let i18n = uiworld.read::<I18n>();
    Window {
        title: i18n.tr("ui.network.title").into(),
        opened,
        pad: Pad::all(10.0),
        radius: 10.0,
        child_spacing: 10.0,
    }
    .show(|| {
        let mut state = uiworld.write::<NetworkState>();
        let mut info = uiworld.write::<NetworkConnectionInfo>();
        common::saveload::JSONPretty::save_silent(&*info, "netinfo");

        match *state {
            NetworkState::Singleplayer(_) => {
                if !info.error.is_empty() {
                    textc(error(), info.error.clone());
                    divider(outline(), 5.0, 1.0);
                }

                text_edit(200.0, &mut info.name, i18n.tr("ui.network.name"));

                if info.name.is_empty() {
                    label(i18n.tr("ui.network.enter_name"));
                    return;
                }

                if button_primary(i18n.tr("ui.network.start_server"))
                    .show()
                    .clicked
                {
                    if let Some(server) = crate::network::start_server(&mut info, sim) {
                        *state = NetworkState::Server(server);
                    }
                }

                divider(outline(), 5.0, 1.0);

                text_edit(200.0, &mut info.ip, i18n.tr("ui.network.ip"));

                if button_primary(i18n.tr("ui.network.connect")).show().clicked {
                    if let Some(c) = crate::network::start_client(&mut info) {
                        *state = NetworkState::Client(c);
                    }
                }
            }
            NetworkState::Client(ref client) => {
                label(client.lock().unwrap().describe());
                show_hashes(sim, &mut info, &i18n);
            }
            NetworkState::Server(ref server) => {
                label(i18n.tr("ui.network.running_server"));
                label(server.lock().unwrap().describe());
                show_hashes(sim, &mut info, &i18n);
            }
        }
    });
}

fn show_hashes(sim: &Simulation, info: &mut NetworkConnectionInfo, i18n: &I18n) {
    checkbox_value(
        &mut info.show_hashes,
        on_secondary_container(),
        i18n.tr("ui.network.show_hashes").to_string(),
    );
    if !info.show_hashes {
        return;
    }

    if sim.get_tick() % 100 == 0 || info.hashes.is_empty() {
        info.hashes = sim.hashes();
        info.hashes_tick = sim.get_tick();
    }

    label(i18n.tr_args(
        "ui.network.hashes_for_tick",
        &[("value", format!("{}", info.hashes_tick))],
    ));
    for (name, hash) in &info.hashes {
        label(format!("{name}: {hash}"));
    }
}
