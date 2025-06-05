use crate::prelude::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::collections::VecDeque;

use avian2d::{parry::shape::Compound, prelude::*};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use geo::{BooleanOps, Coord, CoordsIter, LineString, MultiPolygon, Vector2DOps};
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::loading::LoadResource;

#[derive(Resource, Asset, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct MachinePartConfigByType(pub HashMap<String, MachinePartConfig>);

pub struct MachinePartConfigByTypePlugin;

impl Plugin for MachinePartConfigByTypePlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(MachinePartConfigByTypeLoader);
        app.load_resource_from_path::<MachinePartConfigByType>("machine_parts.ron");
    }
}

#[derive(Default)]
struct MachinePartConfigByTypeLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum MachinePartConfigByTypeLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl AssetLoader for MachinePartConfigByTypeLoader {
    type Asset = MachinePartConfigByType;
    type Settings = ();
    type Error = MachinePartConfigByTypeLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let mut library = ron::de::from_bytes::<MachinePartConfigByType>(&bytes)?;
        for MachinePartConfig { subassemblies, .. } in library.0.values_mut() {
            for subassembly in subassemblies {
                match subassembly {
                    SubAssembly::Collider {
                        mesh_image_path,
                        colliders,
                        ..
                    } => {
                        let loaded_image = load_context
                            .loader()
                            .immediate()
                            .with_static_type()
                            .load::<Image>(mesh_image_path.clone())
                            .await?;
                        *colliders = colliders_from_image(loaded_image.get());
                    }
                    SubAssembly::Sprite {
                        sprite,
                        sprite_asset_path,
                        ..
                    } => {
                        *sprite = load_context
                            .loader()
                            .with_static_type()
                            .load::<Image>(sprite_asset_path.clone());
                    }
                }
            }
        }

        Ok(library)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

fn colliders_from_image(mesh_image: &Image) -> Vec<Compound> {
    let mut pixels = vec![0.0; (mesh_image.width() * mesh_image.height()) as usize];
    for x in 0..mesh_image.width() {
        for y in 0..mesh_image.height() {
            if mesh_image.get_color_at(x, y).unwrap().is_fully_opaque() {
                let i = y * mesh_image.width() + x;
                pixels[i as usize] = 1.0;
            }
        }
    }
    let c = contour::ContourBuilder::new(
        mesh_image.width() as usize,
        mesh_image.height() as usize,
        false,
    );
    let contour = c.contours(&pixels, &[0.5]).unwrap();
    let polygons: MultiPolygon<f64> = contour[0].clone().into_inner().0;
    let polys = divide_reduce::<MultiPolygon<f64>>(vec![polygons], |a, b| a.union(&b))
        .unwrap_or(MultiPolygon::new(vec![]));

    let polys: Vec<_> = polys
        .into_iter()
        .map(|p| {
            geo::Polygon::new(
                removed_in_line(p.exterior()),
                p.interiors().iter().map(removed_in_line).collect(),
            ) //.simplify(&50.0)
        })
        .collect();

    polys
        .into_iter()
        .map(|poly| {
            let exterior: Vec<Vec2> = poly
                .exterior()
                .into_iter()
                .map(|p| {
                    Vec2::new(
                        p.x as f32 - mesh_image.width() as f32 / 2.0,
                        -p.y as f32 + mesh_image.height() as f32 / 2.0,
                    )
                })
                .collect();
            let indices = (0..exterior.len() as u32).collect_vec();
            let collider = Collider::convex_decomposition_with_config(
                exterior,
                indices.windows(2).map(|i| [i[0], i[1]]).collect_vec(),
                &VhacdParameters {
                    //concavity: 0.01,
                    ..Default::default()
                },
            );
            let shape = collider.shape().as_compound().unwrap();
            shape.clone()
        })
        .collect()
}

fn divide_reduce<T>(list: Vec<T>, mut reduction: impl FnMut(T, T) -> T) -> Option<T> {
    let mut queue = VecDeque::from(list);

    while queue.len() > 1 {
        for _ in 0..(queue.len() / 2) {
            let (one, two) = (queue.pop_front().unwrap(), queue.pop_front().unwrap());
            queue.push_back(reduction(one, two));
        }
    }

    queue.pop_back()
}
fn removed_in_line(line: &geo::LineString<f64>) -> geo::LineString<f64> {
    if line.coords_count() < 3 {
        return line.clone();
    }

    let mut points = line.coords().copied();
    let (mut a, mut b) = points.next_tuple().unwrap();
    let mut kept = vec![];

    fn along_path(a: Coord<f64>, b: Coord<f64>, c: Coord<f64>) -> bool {
        let dir_1 = (c - b).try_normalize().unwrap_or_default();
        let dir_2 = (b - a).try_normalize().unwrap_or_default();
        dir_1.dot_product(dir_2) >= 0.9995
    }
    // maybe add first one
    {
        let mut coords = line.coords().copied();
        let b = coords.next().unwrap();
        let c = coords.next().unwrap();
        let _ = coords.next_back().unwrap(); // last coord is a duplicate
        let a = coords.next_back().unwrap();
        if !along_path(a, b, c) {
            kept.push(b);
        }
    }

    for p in points {
        if !along_path(a, b, p) {
            kept.push(b);
        }
        (a, b) = (b, p);
    }
    LineString::<f64>::new(kept)
}
