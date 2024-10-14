mod tile_repository;
mod defs;
mod mvt;

use std::io::{Bytes, Read};
use std::fs;
use flate2::read::GzDecoder;
use geozero::mvt::Message;
use geozero::mvt::tile::{Feature, GeomType};
use reqwest::blocking;
use crate::mvt::{TmpFeature, TmpGeomType, TmpLayer, TmpTile, TmpValue};
use crate::tile_repository::MBTilesTileRepository;

/*
 * Workflow vector tile rendering
 * -> goal render a single tile in the browser
 * -> request tile from MapsStudio
 * -> decode tile into Vec<Feature>
 * -> draw lines
 */

pub fn request_tile(repo: MBTilesTileRepository) {
    // let body = blocking::get("http://lx003360.rsint.net:8080/api/data/osm/5/16/10.pbf")
    // let body = blocking::get("http://lx003360.rsint.net:8080/api/data/contours/9/274/179.pbf")
    //     .unwrap()
    //     .bytes()
    //     .unwrap();

    let body = repo
        .get_tile(0, 0, 0)
        .expect("Could not get tile")
        .tile_data;
    
    let mut uncompressed = GzDecoder::<&[u8]>::new(body.as_slice());
    let mut tile_data: Vec<u8>  = vec![];
    
    uncompressed.read_to_end(tile_data.as_mut()).expect("Could not decode GZip");
    
    let tile_data = tile_data.as_ref();

    let tile = geozero::mvt::Tile::decode(tile_data)
        .expect("Could not decode MVT Tile");
    
    let mut tmp_tile = TmpTile { layers:Vec::new() };
    
    for layer in tile.layers {
        let mut tmp_layer = TmpLayer {
            name: layer.name.clone(),
            version: layer.version.clone(),
            features: Vec::new(),
            extent: layer.extent,
            keys: layer.keys,
            values: layer.values.iter().map(|f| f.into()).collect(),
        };
        
        for feature in layer.features {
            let typ = match feature.r#type {
                Some(t) => Some(TmpGeomType::from_int(t)),
                None => None,
            };
            let tmp_feature = TmpFeature {
                id: feature.id.clone(),
                tags: feature.tags.clone(),
                r#type: typ,
                geometry: feature.geometry.clone(),
            };
            
            tmp_layer.features.push(tmp_feature);
        }
        
        tmp_tile.layers.push(tmp_layer);    
    }
    
    fs::write("output3.txt", format_args!("{:#?}", tmp_tile).to_string()).unwrap();
}

// All Keys: [
//     "admin_level",
//     "disputed",
//     "maritime",
//     "subclass",
//     "class",
//     "name_int",
//     "name_de",
//     "name",
//     "rank",
//     "name:latin",
//     "name_en",
//     "iso_a2",
//     "name:nonlatin",
//     "id",
//     "intermittent",
// ]


fn main() {
    let repo = MBTilesTileRepository::from_file("F:\\Maps\\europe2.mbtiles")
        .expect("Could not open file");

    // println!("{:#?}", repo.get_metadata());

    request_tile(repo);
}
