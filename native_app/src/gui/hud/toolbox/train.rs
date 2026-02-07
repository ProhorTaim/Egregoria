use goryak::{mincolumn, minrow, outline, padxy};
use prototypes::{prototypes_iter, RollingStockID, RollingStockPrototype};
use yakui::widgets::List;
use yakui::{button, divider, label, CrossAxisAlignment, MainAxisAlignment};

use crate::gui::addtrain::TrainSpawnResource;
use crate::i18n::I18n;
use crate::uiworld::UiWorld;

pub fn train_properties(uiw: &UiWorld) {
    let i18n = uiw.read::<I18n>();
    let mut state = uiw.write::<TrainSpawnResource>();

    padxy(0.0, 0.0, || {
        let mut l = List::row();
        l.main_axis_alignment = MainAxisAlignment::Start;
        l.cross_axis_alignment = CrossAxisAlignment::Center;
        l.item_spacing = 10.0;
        l.show(|| {
            mincolumn(0.1, || {
                if button(i18n.tr("ui.train.remove")).clicked {
                    state.wagons.clear();
                    state.set_zero();
                }
                label(i18n.tr_args(
                    "ui.train.acceleration",
                    &[("value", format!("{:.1}", state.acceleration))],
                ));
                label(i18n.tr_args(
                    "ui.train.deceleration",
                    &[("value", format!("{:.1}", state.deceleration))],
                ));
                label(i18n.tr_args(
                    "ui.train.total_length",
                    &[("value", format!("{}", state.total_lenght.ceil()))],
                ));
            });

            mincolumn(0.5, || {
                minrow(0.0, || {
                    let mut remove: Option<usize> = None;
                    for (i, rs) in state
                        .wagons
                        .iter()
                        .map(|id| RollingStockID::prototype(*id))
                        .enumerate()
                    {
                        let label =
                            i18n.proto_label("rolling_stock", &rs.name, &rs.label);
                        if button(label).clicked {
                            remove = Some(i);
                        }
                    }
                    if let Some(i) = remove {
                        state.wagons.remove(i);
                        state.calculate();
                    }
                });

                divider(outline(), 10.0, 1.0);

                minrow(0.0, || {
                    for rolling_stock in prototypes_iter::<RollingStockPrototype>() {
                        let label = i18n.proto_label(
                            "rolling_stock",
                            &rolling_stock.name,
                            &rolling_stock.label,
                        );
                        let resp = button(label);
                        if resp.clicked {
                            state.wagons.push(rolling_stock.id);
                            state.calculate();
                        }
                    }
                });
            });
        });
    });
}

/*
if ui.button(freightstation).clicked() {
   *uiworld.write::<Tool>() = Tool::SpecialBuilding;

   uiworld.write::<SpecialBuildingResource>().opt = Some(SpecialBuildKind {
       make: Box::new(move |args| {
           let obb = args.obb;
           let c = obb.center().z(args.mpos.z + 0.3);

           let [offx, offy] = obb.axis().map(|x| x.normalize().z(0.0));

           let pat =
               LanePatternBuilder::new().rail(true).one_way(true).build();

           let mut commands = Vec::with_capacity(5);

           commands.push(WorldCommand::MapMakeConnection {
               from: MapProject::ground(c - offx * 45.0 - offy * 100.0),
               to: MapProject::ground(c - offx * 45.0 + offy * 100.0),
               inter: None,
               pat,
           });

           commands.push(WorldCommand::MapBuildSpecialBuilding {
               pos: args.obb,
               kind: BuildingKind::RailFreightStation(proto.id),
               gen: BuildingGen::NoWalkway {
                   door_pos: Vec2::ZERO,
               },
               zone: None,
               connected_road: args.connected_road,
           });
           commands
       }),
       size: proto.size,
       asset: proto.asset.clone(),
       road_snap: false,
   });
}
*/
