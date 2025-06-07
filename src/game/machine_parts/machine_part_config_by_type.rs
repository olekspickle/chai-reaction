use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::{platform::collections::HashMap, render::render_resource::Extent3d};
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
        for MachinePartConfig {
            subassemblies,
            texture_info,
            ..
        } in library.0.values_mut()
        {
            for subassembly in subassemblies {
                match subassembly {
                    SubAssembly::FluidFilter {
                        mesh_image_path,
                        colliders,
                        ..
                    } |
                    SubAssembly::ConveyorBelt{
                        mesh_image_path,
                        colliders,
                        ..
                    } |
                    SubAssembly::FluidFilterButton {
                        mesh_image_path,
                        colliders,
                        ..
                    } |
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
                        let image = loaded_image.get();
                        let size = image.size();
                        let rotations = texture_info.rotations;

                        let row_height = size.y / rotations;
                        let mut rotation_colliders = Vec::new();

                        for rot in 0..rotations {
                            let offset = UVec2::new(0, row_height * rot);
                            let region_size = UVec2::new(size.x, row_height);
                            let new_colliders =
                                colliders_from_image_region(&image, offset, region_size);
                            rotation_colliders.push(new_colliders);
                        }

                        *colliders = rotation_colliders;
                    }
                    SubAssembly::Sprite {
                        sprite,
                        sprite_asset_path,
                        ..
                    } => {
                        let loaded_texture = load_context
                            .loader()
                            .immediate()
                            .with_static_type()
                            .load::<Image>(sprite_asset_path.clone())
                            .await?;

                        let size = loaded_texture.get().size().as_vec2();
                        sprite.image = load_context
                            .add_loaded_labeled_asset(sprite_asset_path.clone(), loaded_texture);

                        let rotations = texture_info.rotations;
                        let frames = texture_info.frames.frames();
                        if rotations != 1 || frames != 1 {
                            let pixel_size = size.as_uvec2();
                            let tile_size =
                                UVec2::new(pixel_size.x / frames, pixel_size.y / rotations);
                            let columns = frames;
                            let rows = rotations;
                            // TextureAtlasLayout::from_grid expects tile_size: Vec2, columns: usize, rows: usize, padding: Option<Vec2>, offset: Option<Vec2>
                            let layout =
                                TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
                            // Add the layout as an asset and get a handle
                            let layout_handle = load_context.add_loaded_labeled_asset(
                                format!("{sprite_asset_path}_layout"),
                                layout.into(),
                            );
                            sprite.layout = Some(layout_handle);
                        }
                    }
                    SubAssembly::ParticleVessel { 
                        texture_path, 
                        image, 
                        ..
                    } => {
                        let loaded_image = load_context
                            .loader()
                            .immediate()
                            .with_static_type()
                            .load::<Image>(texture_path.clone())
                            .await?;

                        *image = load_context.add_loaded_labeled_asset(
                            texture_path.clone(),
                            loaded_image,
                        );
                    }
                    SubAssembly::FlowField {
                        flow_texture_path,
                        flow_texture,
                        collider,
                    } => {
                        let loaded_flow_texture = load_context
                            .loader()
                            .immediate()
                            .with_static_type()
                            .load::<Image>(flow_texture_path.clone())
                            .await?;

                        let size = loaded_flow_texture.get().size().as_vec2();
                        flow_texture.image = load_context.add_loaded_labeled_asset(
                            flow_texture_path.clone(),
                            loaded_flow_texture,
                        );

                        let rotations = texture_info.rotations;
                        if rotations != 1 {
                            let pixel_size = size.as_uvec2();
                            let tile_size = UVec2::new(pixel_size.x, pixel_size.y / rotations);
                            let columns = 1;
                            let rows = rotations;
                            let layout =
                                TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
                            let layout_handle = load_context.add_loaded_labeled_asset(
                                format!("{flow_texture_path}_layout"),
                                layout.into(),
                            );
                            flow_texture.layout = Some(layout_handle);
                        }

                        *collider = Collider::rectangle(size.x, size.y / rotations as f32);
                    }
                    _ => (),
                }
            }
        }

        Ok(library)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

fn colliders_from_image_region(mesh_image: &Image, offset: UVec2, size: UVec2) -> Vec<Compound> {
    let mut pixels = vec![0.0; (size.x * size.y) as usize];
    for x in 0..size.x {
        for y in 0..size.y {
            let img_x = offset.x + x;
            let img_y = offset.y + y;
            if let Ok(color) = mesh_image.get_color_at(img_x, img_y) {
                if color.is_fully_opaque() {
                    let i = y * size.x + x;
                    pixels[i as usize] = 1.0;
                }
            }
        }
    }
    let c = contour::ContourBuilder::new(size.x as usize, size.y as usize, false);
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
            )
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
                        p.x as f32 - size.x as f32 / 2.0,
                        -p.y as f32 + size.y as f32 / 2.0,
                    )
                })
                .collect();
            let indices = (0..exterior.len() as u32).collect_vec();
            let collider = Collider::convex_decomposition_with_config(
                exterior,
                indices.windows(2).map(|i| [i[0], i[1]]).collect_vec(),
                &VhacdParameters {
                    ..Default::default()
                },
            );
            let shape = collider.shape().as_compound().unwrap();
            shape.clone()
        })
        .collect()
}

fn colliders_from_image(mesh_image: &Image) -> Vec<Compound> {
    let size = UVec2::new(mesh_image.width(), mesh_image.height());
    colliders_from_image_region(mesh_image, UVec2::ZERO, size)
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
