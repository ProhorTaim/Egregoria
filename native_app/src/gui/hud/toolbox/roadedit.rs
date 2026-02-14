use yakui::widgets::List;
use yakui::{
    column, image, reflow, Alignment, CrossAxisAlignment, Dim2, MainAxisAlignment, Pivot, Vec2,
};

use goryak::{padxy, primary_image_button};
use simulation::map::LightPolicy;

use crate::gui::hud::toolbox;
use crate::gui::hud::toolbox::select_triangle;
use crate::gui::roadeditor::RoadEditorResource;
use crate::gui::textures::UiTextures;
use crate::i18n::I18n;
use crate::uiworld::UiWorld;

pub fn roadedit_properties(uiw: &UiWorld) {
    let i18n = uiw.read::<I18n>();
    let state = &mut *uiw.write::<RoadEditorResource>();
    let Some(ref mut v) = state.inspect else {
        return;
    };

    padxy(0.0, 10.0, || {
        let mut l = List::row();
        l.main_axis_alignment = MainAxisAlignment::Center;
        l.cross_axis_alignment = CrossAxisAlignment::Center;
        l.item_spacing = 10.0;
        l.show(|| {
            let texs = uiw.read::<UiTextures>();

            let light_policy_choices = &[
                (
                    LightPolicy::NoLights,
                    "ui.roadedit.no_lights",
                    "roadedit_no_light",
                ),
                (
                    LightPolicy::Lights,
                    "ui.roadedit.traffic_lights",
                    "roadedit_light",
                ),
                (
                    LightPolicy::StopSigns,
                    "ui.roadedit.stop_signs",
                    "roadedit_stop_sign",
                ),
                (LightPolicy::Auto, "ui.roadedit.auto", "roadedit_auto"),
            ];

            for (policy, label, icon) in light_policy_choices {
                column(|| {
                    let enabled = v.light_policy == *policy;
                    if primary_image_button(
                        texs.get(icon),
                        Vec2::new(64.0, 64.0),
                        enabled,
                        i18n.tr(label).to_string(),
                    )
                    .clicked
                    {
                        v.light_policy = *policy;
                        state.dirty = true;
                    }

                    if enabled {
                        select_triangle(uiw);
                    }
                });
            }

            let mut has_roundabout = v.turn_policy.roundabout.is_some();

            let turn_policies = [
                (
                    &mut v.turn_policy.left_turns,
                    i18n.tr("ui.roadedit.left_turns"),
                    "roadedit_left_turn",
                ),
                (
                    &mut v.turn_policy.back_turns,
                    i18n.tr("ui.roadedit.back_turns"),
                    "roadedit_back_turn",
                ),
                (
                    &mut v.turn_policy.crosswalks,
                    i18n.tr("ui.roadedit.crosswalks"),
                    "roadedit_crosswalk",
                ),
                (
                    &mut has_roundabout,
                    i18n.tr("ui.roadedit.roundabout"),
                    "roadedit_roundabout",
                ),
            ];

            for (enabled, label, icon) in turn_policies {
                column(|| {
                    if primary_image_button(
                        texs.get(icon),
                        Vec2::new(64.0, 64.0),
                        *enabled,
                        label.to_string(),
                    )
                    .clicked
                    {
                        *enabled = !*enabled;
                        state.dirty = true;
                    }

                    if !*enabled {
                        reflow(
                            Alignment::TOP_LEFT,
                            Pivot::TOP_LEFT,
                            Dim2::pixels(0.0, 0.0),
                            || {
                                image(texs.get("roadedit_forbidden"), Vec2::new(64.0, 64.0));
                            },
                        );
                    }
                });
            }

            if has_roundabout != v.turn_policy.roundabout.is_some() {
                v.turn_policy.roundabout = if has_roundabout {
                    Some(Default::default())
                } else {
                    None
                };
                state.dirty = true;
            }

            if let Some(ref mut roundabout) = v.turn_policy.roundabout {
                state.dirty |= toolbox::updown_value(&mut roundabout.radius, 2.0, "m");
            }
        });
    });
}
