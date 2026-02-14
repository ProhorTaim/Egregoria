use yakui::widgets::List;
use yakui::{
    image, reflow, Alignment, Color, CrossAxisAlignment, Dim2, MainAxisAlignment, MainAxisSize,
    Pivot, Vec2,
};

use goryak::{image_button, mincolumn, minrow, padxy, primary};
use simulation::map::LanePatternBuilder;

use crate::gui::hud::toolbox::updown_value;
use crate::gui::roadbuild::{HeightReference, RoadBuildResource, Snapping};
use crate::gui::textures::UiTextures;
use crate::i18n::I18n;
use crate::uiworld::UiWorld;

pub fn roadbuild_properties(uiw: &UiWorld) {
    let i18n = uiw.read::<I18n>();
    let mut state = uiw.write::<RoadBuildResource>();

    padxy(0.0, 10.0, || {
        let mut l = List::row();
        l.main_axis_alignment = MainAxisAlignment::Center;
        l.cross_axis_alignment = CrossAxisAlignment::Center;
        l.item_spacing = 10.0;
        l.show(|| {
            let c = primary().lerp(&Color::WHITE, 0.3);
            let active = (c, c.with_alpha(0.7));
            let default = (Color::WHITE.with_alpha(0.3), Color::WHITE.with_alpha(0.5));

            mincolumn(4.0, || {
                minrow(2.0, || {
                    let (snapping_none, snapping_grid, snapping_angel) = match state.snapping {
                        Snapping::None => (active, default, default),
                        Snapping::SnapToGrid => (default, active, default),
                        Snapping::SnapToAngle => (default, default, active),
                    };
                    if image_button(
                        uiw.read::<UiTextures>().get("snap_notting"),
                        Vec2::new(30.0, 30.0),
                        snapping_none.0,
                        snapping_none.1,
                        primary(),
                        i18n.tr("ui.roadbuild.snap.none").to_string(),
                    )
                    .clicked
                    {
                        state.snapping = Snapping::None;
                    }
                    if image_button(
                        uiw.read::<UiTextures>().get("snap_grid"),
                        Vec2::new(30.0, 30.0),
                        snapping_grid.0,
                        snapping_grid.1,
                        primary(),
                        i18n.tr("ui.roadbuild.snap.grid").to_string(),
                    )
                    .clicked
                    {
                        state.snapping = Snapping::SnapToGrid;
                    }
                    if image_button(
                        uiw.read::<UiTextures>().get("snap_angle"),
                        Vec2::new(30.0, 30.0),
                        snapping_angel.0,
                        snapping_angel.1,
                        primary(),
                        i18n.tr("ui.roadbuild.snap.angle").to_string(),
                    )
                    .clicked
                    {
                        state.snapping = Snapping::SnapToAngle;
                    }
                });

                minrow(2.0, || {
                    let (hos_ground, hos_start, hos_incline, hos_decline) =
                        match state.height_reference {
                            HeightReference::Ground => (active, default, default, default),
                            HeightReference::Start => (default, active, default, default),
                            HeightReference::MaxIncline => (default, default, active, default),
                            HeightReference::MaxDecline => (default, default, default, active),
                        };
                    if image_button(
                        uiw.read::<UiTextures>().get("height_reference_ground"),
                        Vec2::new(30.0, 30.0),
                        hos_ground.0,
                        hos_ground.1,
                        primary(),
                        i18n.tr("ui.roadbuild.height.ground").to_string(),
                    )
                    .clicked
                    {
                        state.height_reference = HeightReference::Ground;
                    }
                    if image_button(
                        uiw.read::<UiTextures>().get("height_reference_start"),
                        Vec2::new(30.0, 30.0),
                        hos_start.0,
                        hos_start.1,
                        primary(),
                        i18n.tr("ui.roadbuild.height.start").to_string(),
                    )
                    .clicked
                    {
                        state.height_reference = HeightReference::Start;
                    }
                    if image_button(
                        uiw.read::<UiTextures>().get("height_reference_incline"),
                        Vec2::new(30.0, 30.0),
                        hos_incline.0,
                        hos_incline.1,
                        primary(),
                        i18n.tr("ui.roadbuild.height.max_incline").to_string(),
                    )
                    .clicked
                    {
                        state.height_reference = HeightReference::MaxIncline;
                    }
                    if image_button(
                        uiw.read::<UiTextures>().get("height_reference_decline"),
                        Vec2::new(30.0, 30.0),
                        hos_decline.0,
                        hos_decline.1,
                        primary(),
                        i18n.tr("ui.roadbuild.height.max_decline").to_string(),
                    )
                    .clicked
                    {
                        state.height_reference = HeightReference::MaxDecline;
                    }
                });
            });
            // Road elevation
            updown_value(&mut state.height_offset, 2.0, "m");

            // image name, label, builder
            let builders: &[(&str, &str, LanePatternBuilder)] = &[
                (
                    "roadtypes_street",
                    "ui.roadtype.street",
                    LanePatternBuilder::new(),
                ),
                (
                    "roadtypes_street_1way",
                    "ui.roadtype.street_one_way",
                    LanePatternBuilder::new().one_way(true),
                ),
                (
                    "roadtypes_avenue",
                    "ui.roadtype.avenue",
                    LanePatternBuilder::new().n_lanes(2).speed_limit(13.0),
                ),
                (
                    "roadtypes_avenue_1way",
                    "ui.roadtype.avenue_one_way",
                    LanePatternBuilder::new()
                        .n_lanes(2)
                        .one_way(true)
                        .speed_limit(13.0),
                ),
                (
                    "roadtypes_drive",
                    "ui.roadtype.drive",
                    LanePatternBuilder::new()
                        .parking(false)
                        .sidewalks(false)
                        .speed_limit(13.0),
                ),
                (
                    "roadtypes_drive_1way",
                    "ui.roadtype.drive_one_way",
                    LanePatternBuilder::new()
                        .parking(false)
                        .sidewalks(false)
                        .one_way(true)
                        .speed_limit(13.0),
                ),
                (
                    "roadtypes_highway",
                    "ui.roadtype.highway",
                    LanePatternBuilder::new()
                        .n_lanes(3)
                        .speed_limit(25.0)
                        .parking(false)
                        .sidewalks(false),
                ),
                (
                    "roadtypes_highway_1way",
                    "ui.roadtype.highway_one_way",
                    LanePatternBuilder::new()
                        .n_lanes(3)
                        .speed_limit(25.0)
                        .parking(false)
                        .sidewalks(false)
                        .one_way(true),
                ),
                (
                    "roadtypes_rail",
                    "ui.roadtype.rail",
                    LanePatternBuilder::new().rail(true),
                ),
                (
                    "roadtypes_rail_1way",
                    "ui.roadtype.rail_one_way",
                    LanePatternBuilder::new().rail(true).one_way(true),
                ),
            ];

            for (icon, label, builder) in builders {
                let mut l = List::column();
                l.main_axis_size = MainAxisSize::Min;
                l.show(|| {
                    let is_active = &state.pattern_builder == builder;
                    let (default_col, hover_col) = if is_active {
                        let c = Color::WHITE.adjust(0.5);
                        (c, c)
                    } else {
                        (Color::WHITE, Color::WHITE.with_alpha(0.7))
                    };
                    if image_button(
                        uiw.read::<UiTextures>().get(icon),
                        Vec2::new(64.0, 64.0),
                        default_col,
                        hover_col,
                        primary(),
                        i18n.tr(label).to_string(),
                    )
                    .clicked
                    {
                        state.pattern_builder = *builder;
                    }

                    if is_active {
                        reflow(
                            Alignment::CENTER_LEFT,
                            Pivot::TOP_LEFT,
                            Dim2::pixels(0.0, 32.0),
                            || {
                                image(
                                    uiw.read::<UiTextures>().get("select_triangle_under"),
                                    Vec2::new(64.0, 10.0),
                                );
                            },
                        );
                    }
                });
            }
        });
    });
}
