// As of now implementing the Ram only solution and i will implement complete disk zero copy persistance when completed
// Os internals , how database works ?? watch some tuts .

use crate::HNSW;
use memmap2::*;
use rkyv::rancor::Error;
use rkyv::Archive;
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;
use std::{fs::File, path::PathBuf};
use std::io::Read;

const db_name: &str= "main_hnsw_database.pho";

#[derive(Debug)]
pub struct PhotonDB {
    pub hnsw: HNSW,
    pub dim: usize,
    pub path: PathBuf,
}

impl PhotonDB {
    fn save(&self) -> Result<bool, String>{
        let mut file = File::create(&self.path).expect("Failed to create db file");
        file = File::open(&self.path).unwrap();
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&self.hnsw).unwrap();
        file.write_all(&bytes).expect("Error in writing");
        Ok(true)

    }

    fn load(path: PathBuf, dim: usize) -> Result<PhotonDB, String> {
        let dir_path = path.parent().unwrap();
        let db_path = dir_path.join(db_name);
        if db_path.exists() {
            
            let mut file = File::open(&db_path).unwrap();
            // let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&player).unwrap();
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).expect("Failed to read file");

            let hnsw = rkyv::from_bytes::<HNSW, Error>(&bytes).unwrap();

            Ok(PhotonDB {
                hnsw,
                dim,
                path: db_path,
            })
          


        } else {
            // println!("Error: {:?}", db_path);
            Err("Database Curpted".to_string())
        }
    }

    fn create(path: PathBuf, max_elements: usize, dim: usize) -> Result<PhotonDB, String> {
        let dir_path = path.parent().unwrap();
        let db_path = dir_path.join(db_name);


        Ok(PhotonDB {
            hnsw: HNSW::new(max_elements, dim),
            dim,
            path: db_path
        })
        
    }
}
