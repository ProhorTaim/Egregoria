use common::FastMap;
use engine::{FrameContext, GfxContext, InstancedMeshBuilder, MeshInstance, SpriteBatchBuilder};
use geom::{LinearColor, Vec3, V3};
use prototypes::{RenderAsset, RollingStockID, RollingStockPrototype};
use simulation::transportation::{Location, VehicleKind};
use simulation::Simulation;

/// Render all entities using instanced rendering for performance
pub struct InstancedRender {
    pub path_not_found: SpriteBatchBuilder<true>,
    pub rolling_stock: FastMap<RollingStockID, InstancedMeshBuilder<true>>,
    pub cars: Option<InstancedMeshBuilder<true>>,
    // pub locomotives: InstancedMeshBuilder<true>,
    // pub wagons_passenger: InstancedMeshBuilder<true>,
    // pub wagons_freight: InstancedMeshBuilder<true>,
    pub trucks: Option<InstancedMeshBuilder<true>>,
    pub pedestrians: Option<InstancedMeshBuilder<true>>,
}

impl InstancedRender {
    pub fn new(gfx: &mut GfxContext) -> Self {
        defer!(log::info!("finished init of instanced render"));

        let mut rolling_stock = FastMap::default();
        RollingStockPrototype::iter()
            .map(|rail_wagon_proto| (&rail_wagon_proto.asset, rail_wagon_proto.id))
            .filter_map(|(asset, id)| {
                let RenderAsset::Mesh { path } = asset else {
                    None?
                };
                match gfx.mesh(path) {
                    Err(e) => {
                        log::error!("Failed to load mesh {}: {:?}", asset, e);
                        None
                    }
                    Ok(m) => Some((id, m)),
                }
            })
            .for_each(|(id, mesh)| {
                rolling_stock.insert(id, InstancedMeshBuilder::new_ref(&mesh));
            });

        // Try to load optional models - if they all fail, we'll skip rendering them
        let car = match gfx.mesh("simple_car.glb".as_ref()) {
            Ok(m) => Some(m),
            Err(e) => {
                log::warn!("Failed to load car model: {:?}, continuing without it", e);
                None
            }
        };

        let truck = match gfx.mesh("truck.glb".as_ref()) {
            Ok(m) => Some(m),
            Err(e) => {
                log::warn!("Failed to load truck model: {:?}, continuing without it", e);
                None
            }
        };

        let pedestrian = match gfx.mesh("pedestrian.glb".as_ref()) {
            Ok(m) => Some(m),
            Err(e) => {
                log::warn!(
                    "Failed to load pedestrian model: {:?}, continuing without it",
                    e
                );
                None
            }
        };

        // If we have at least one model, use it; otherwise create with None
        let some_model = if let Some(m) = car.as_ref() {
            Some(m)
        } else if let Some(m) = truck.as_ref() {
            Some(m)
        } else if let Some(m) = pedestrian.as_ref() {
            Some(m)
        } else {
            log::warn!("No vehicle models loaded! Vehicles will not be rendered.");
            None
        };

        // Create InstancedMeshBuilders for loaded models, None for missing ones
        let (cars_builder, trucks_builder, pedestrians_builder) = if let Some(fallback) = some_model
        {
            (
                car.as_ref().map(|m| InstancedMeshBuilder::new_ref(m)),
                truck.as_ref().map(|m| InstancedMeshBuilder::new_ref(m)),
                pedestrian
                    .as_ref()
                    .map(|m| InstancedMeshBuilder::new_ref(m)),
            )
        } else {
            (None, None, None)
        };

        InstancedRender {
            path_not_found: SpriteBatchBuilder::new(
                &gfx.texture("assets/sprites/path_not_found.png", "path_not_found"),
                gfx,
            ),

            rolling_stock,

            cars: cars_builder,
            trucks: trucks_builder,
            pedestrians: pedestrians_builder,
        }
    }

    pub fn render(&mut self, sim: &Simulation, fctx: &mut FrameContext<'_>) {
        profiling::scope!("entity_render::render");
        if let Some(cars) = self.cars.as_mut() {
            cars.instances.clear();
        }
        if let Some(trucks) = self.trucks.as_mut() {
            trucks.instances.clear();
        }
        if let Some(pedestrians) = self.pedestrians.as_mut() {
            pedestrians.instances.clear();
        }

        for v in sim.world().vehicles.values() {
            let trans = &v.trans;
            let instance = MeshInstance {
                pos: trans.pos,
                dir: trans.dir,
                tint: v.vehicle.tint.into(),
            };

            match v.vehicle.kind {
                VehicleKind::Car => {
                    if let Some(cars) = self.cars.as_mut() {
                        cars.instances.push(instance)
                    }
                }
                VehicleKind::Truck => {
                    if let Some(trucks) = self.trucks.as_mut() {
                        trucks.instances.push(instance)
                    }
                }
                _ => {}
            }
        }

        self.rolling_stock.iter_mut().for_each(|(_, m)| {
            m.instances.clear();
        });
        for wagon in sim.world().wagons.values() {
            let trans = &wagon.trans;
            let instance = MeshInstance {
                pos: trans.pos,
                dir: trans.dir,
                tint: LinearColor::WHITE,
            };

            if let Some(mesh) = self.rolling_stock.get_mut(&wagon.wagon.rolling_stock) {
                mesh.instances.push(instance);
            }
        }

        for p in sim.world().humans.values() {
            if matches!(p.location, Location::Outside) {
                if let Some(pedestrians) = self.pedestrians.as_mut() {
                    pedestrians.instances.push(MeshInstance {
                        pos: p.trans.pos.up(0.5 + 0.4 * p.pedestrian.walk_anim.cos()),
                        dir: p.trans.dir.xy().z0(),
                        tint: LinearColor::WHITE,
                    });
                }
            }
        }

        self.path_not_found.clear();
        for (_, (trans, itin)) in sim.world().query_trans_itin() {
            let Some(wait) = itin.is_wait_for_reroute() else {
                continue;
            };
            if wait == 0 {
                continue;
            }

            let r = wait as f32 / 200.0;
            let off = 1.0 - r;

            let s = 7.0;
            self.path_not_found.push(
                trans.pos + off * 3.0 * V3::Y + 3.0 * V3::Z,
                Vec3::X,
                LinearColor::RED.a(r),
                (s, s),
            );
        }

        if let Some(x) = self.path_not_found.build(fctx.gfx) {
            fctx.objs.push(Box::new(x));
        }
        if let Some(cars) = self.cars.as_mut() {
            if let Some(x) = cars.build(fctx.gfx) {
                fctx.objs.push(Box::new(x));
            }
        }
        if let Some(trucks) = self.trucks.as_mut() {
            if let Some(x) = trucks.build(fctx.gfx) {
                fctx.objs.push(Box::new(x));
            }
        }
        if let Some(pedestrians) = self.pedestrians.as_mut() {
            if let Some(x) = pedestrians.build(fctx.gfx) {
                fctx.objs.push(Box::new(x));
            }
        }

        self.rolling_stock.iter_mut().for_each(|(_, imb)| {
            if let Some(x) = imb.build(fctx.gfx) {
                fctx.objs.push(Box::new(x));
            }
        });
    }
}
