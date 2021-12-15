use std::collections::BTreeMap;
use super::TextureId;

#[derive(Debug, Default, Clone)]
pub struct TextureSizes(BTreeMap<TextureId, (u32, u32)>);

impl TextureSizes {
    /// Construct using a name -> size map
    pub fn new(
        textures: &BTreeMap<TextureId, String>,
        texture_sizes: BTreeMap<&str, (u32, u32)>,
    ) -> Self {
        let texture_sizes = textures
            .into_iter()
            .flat_map(|(texture_id, texture)| {
                if let Some(texture_size) = texture_sizes.get(texture.as_str()) {
                    Some((*texture_id, *texture_size))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();
        TextureSizes(texture_sizes)
    }

    pub fn get(&self, texture_id: &TextureId) -> Option<&(u32, u32)> {
        self.0.get(texture_id)
    }
}
