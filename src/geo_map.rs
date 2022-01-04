use std::collections::BTreeMap;

use shalrath::repr::{Brush, BrushPlane, Entity, Extension, Properties, TextureOffset, Triangle};

use crate::{brush::BrushId, entity::EntityId, face::FaceId, texture::TextureId, Vector2};

pub type Entities = Vec<EntityId>;
pub type Brushes = Vec<BrushId>;
pub type Faces = Vec<FaceId>;

pub type PointEntities = Vec<EntityId>;

pub type EntityProperties = BTreeMap<EntityId, Properties>;
pub type EntityBrushes = BTreeMap<EntityId, Vec<BrushId>>;

pub type BrushFaces = BTreeMap<BrushId, Vec<FaceId>>;

pub type FacePlanes = BTreeMap<FaceId, Triangle>;
pub type FaceTextures = BTreeMap<FaceId, TextureId>;
pub type FaceOffsets = BTreeMap<FaceId, TextureOffset>;
pub type FaceAngles = BTreeMap<FaceId, f32>;
pub type FaceScales = BTreeMap<FaceId, Vector2>;
pub type FaceExtensions = BTreeMap<FaceId, Extension>;

pub type Textures = BTreeMap<TextureId, String>;

/// Struct-of-arrays representation of a [`shalrath::repr::Map`]
#[derive(Debug, Default, Clone)]
pub struct GeoMap {
    pub entities: Entities,
    pub brushes: Brushes,
    pub faces: Faces,

    pub textures: Textures,

    pub entity_properties: EntityProperties,
    pub entity_brushes: EntityBrushes,
    pub point_entities: PointEntities,

    pub brush_faces: BrushFaces,

    pub face_planes: FacePlanes,
    pub face_textures: FaceTextures,
    pub face_offsets: FaceOffsets,
    pub face_angles: FaceAngles,
    pub face_scales: FaceScales,
    pub face_extensions: FaceExtensions,
}

impl GeoMap {
    pub fn new(shalrath::repr::Map(map): shalrath::repr::Map) -> Self {
        let mut entity_head = 0;
        let mut brush_head = 0;
        let mut plane_head = 0;
        let mut texture_head = 0;

        let mut entities = Vec::<EntityId>::default();
        let mut brushes = Vec::<BrushId>::default();
        let mut faces = Vec::<FaceId>::default();

        let mut entity_properties = BTreeMap::<EntityId, Properties>::default();
        let mut entity_brushes = BTreeMap::<EntityId, Vec<BrushId>>::default();

        let mut brush_faces = BTreeMap::<BrushId, Vec<FaceId>>::default();

        let mut face_planes = BTreeMap::<FaceId, Triangle>::default();
        let mut face_textures = BTreeMap::<FaceId, TextureId>::default();
        let mut face_offsets = BTreeMap::<FaceId, TextureOffset>::default();
        let mut face_angles = BTreeMap::<FaceId, f32>::default();
        let mut face_scales = BTreeMap::<FaceId, Vector2>::default();
        let mut face_extensions = BTreeMap::<FaceId, Extension>::default();

        let mut textures = BTreeMap::<String, TextureId>::new();

        for Entity {
            properties,
            brushes: shalrath::repr::Brushes(bs),
        } in map.into_iter()
        {
            let entity_id = EntityId(entity_head);
            entity_head += 1;

            entities.push(entity_id);
            entity_properties.insert(entity_id, properties);

            for Brush(ps) in bs {
                let brush_id = BrushId(brush_head);
                brush_head += 1;

                brushes.push(brush_id);
                entity_brushes.entry(entity_id).or_default().push(brush_id);

                for BrushPlane {
                    plane,
                    texture,
                    texture_offset,
                    angle,
                    scale_x,
                    scale_y,
                    extension,
                } in ps
                {
                    let plane_id = FaceId(plane_head);
                    plane_head += 1;

                    faces.push(plane_id);
                    face_planes.insert(plane_id, plane);

                    let texture_id = if let Some(texture_id) = textures.get(&texture) {
                        *texture_id
                    } else {
                        let texture_id = TextureId(texture_head);
                        textures.insert(texture, texture_id);
                        texture_head += 1;
                        texture_id
                    };

                    face_textures.insert(plane_id, texture_id);

                    face_offsets.insert(plane_id, texture_offset);
                    face_angles.insert(plane_id, angle);
                    face_scales.insert(plane_id, nalgebra::vector![scale_x, scale_y]);
                    face_extensions.insert(plane_id, extension);
                    brush_faces.entry(brush_id).or_default().push(plane_id);
                }
            }
        }

        let point_entities: Vec<EntityId> = entities
            .iter()
            .filter(|entity_id| !entity_brushes.contains_key(entity_id))
            .copied()
            .collect();

        let textures = textures
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<BTreeMap<_, _>>();

        GeoMap {
            entities,
            brushes,
            faces,
            textures,
            entity_properties,
            entity_brushes,
            point_entities,
            brush_faces,
            face_planes,
            face_textures,
            face_offsets,
            face_angles,
            face_scales,
            face_extensions,
        }
    }
}

impl From<shalrath::repr::Map> for GeoMap {
    fn from(map: shalrath::repr::Map) -> Self {
        GeoMap::new(map)
    }
}
