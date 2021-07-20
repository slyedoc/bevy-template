use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::*;
use spritesheet_generator::sprite_sheet;

use crate::helpers;

#[derive(AssetCollection)]
pub struct DungeonPackAssets {
    #[asset(path = "dungeon_pack_iso.png")]
    pub texture: Handle<Texture>,

    #[asset(path = "dungeon_pack_iso.gen.ron")]
    pub sheet: Handle<spritesheet_generator::sprite_sheet::SpriteSheet>,
}

#[allow(dead_code)]
#[derive(Inspectable)]
pub struct DungeonPackAtlas {
    pub atlas: Handle<TextureAtlas>,

    pub floors: Vec<u32>,
    pub columns: Vec<u32>,
    pub walls: Vec<u32>,
    pub bridges: Vec<u32>,
    pub chests: Vec<u32>,
    pub chairs: Vec<u32>,
    pub barrels: Vec<u32>,
    pub stairs: Vec<u32>,
    pub supports: Vec<u32>,
    pub tables: Vec<u32>,

    #[inspectable(ignore)]
    pub frames: HashMap<u32, String>,
}

impl DungeonPackAtlas {
    
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.frames.len()
    }
}

impl FromWorld for DungeonPackAtlas {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let assets = cell
            .get_resource::<DungeonPackAssets>()
            .expect("DungeonPackAsset not loaded");
        let mut atlases = cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .expect("TextureAtlases not loaded");

        let sheets = cell
            .get_resource::<Assets<sprite_sheet::SpriteSheet>>()
            .expect("TextureAtlases not loaded");

        let sheet = sheets.get(&assets.sheet).unwrap();
        let texture_atlas = helpers::texture::from_sprite_sheet(assets.texture.clone(), sheet);

        let mut pack = DungeonPackAtlas {
            atlas: atlases.add(texture_atlas),
            frames: HashMap::default(),
            columns: Vec::new(),
            floors: Vec::new(),
            walls: Vec::new(),
            bridges: Vec::new(),
            chests: Vec::new(),
            chairs: Vec::new(),
            barrels: Vec::new(),
            stairs: Vec::new(),
            tables: Vec::new(),
            supports: Vec::new(),
        };

        for index in 0u32..sheet.frames.len() as u32 {
            let sheet_frame = sheet.frames.get(index as usize).unwrap();
            let name = sheet_frame.name.to_lowercase();
            // for now add everything to frames
            pack.frames.insert(index, name.to_owned());

            if name.contains("column") {
                pack.columns.push(index);
            } else if name.contains("wall") {
                pack.walls.push(index);
            } else if name.contains("bridge") {
                pack.bridges.push(index);
            } else if name.contains("chair") {
                pack.chairs.push(index);
            } else if name.contains("chest") {
                pack.chests.push(index);
            } else if name.contains_any(&vec!["barrel", "crate", "pile"]) {
                pack.barrels.push(index);
            } else if name.contains("support") {
                pack.supports.push(index);
            } else if name.contains("table") {
                pack.tables.push(index);
            } else if name.contains_any(&vec!["stair", "step"]) {
                pack.stairs.push(index);
            } else {
                // hoping all that are left are floor tiles
                pack.floors.push(index);
            }

        }

        pack
    }
}

trait Contains {
    fn contains_any(&self, vec: &[&str]) -> bool;
}

impl Contains for String {
    fn contains_any(&self, vec: &[&str]) -> bool {
        let mut any = false;
        for s in vec.iter() {
            if self.contains(s) {
                any = true;
            }
        }
        any
    }
}