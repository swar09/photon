use crate::HNSW;
use memmap2::*;
use pyo3::ffi::newfunc;
use rkyv::rancor::Error;
use std::fs::{self, exists};
use std::io;
use std::path::Path;
use std::{fs::File, path::PathBuf};

#[derive(Debug)]
pub struct PhotonDB {
    pub hnsw: HNSW,
    pub dim: usize,
    pub path: PathBuf,
}

impl PhotonDB {
    // fn new() -> Self {todo!()}

    pub fn load_or_create(path: PathBuf, max_elements: usize, dim: usize) -> io::Result<PhotonDB> {
        let parent_dir = path.parent().unwrap_or(Path::new("."));
        let p = parent_dir.join("hnsw_database_main.pho");
        if p.exists() {
            // Print some text here for the user entertainment
            // May be try to add loading screens

            let file = File::open(&p).unwrap();

            let mmap = unsafe { Mmap::map(&file)? };

            let bytes = &mmap[..];

            let hnsw = rkyv::from_bytes::<HNSW, Error>(&bytes).unwrap();
            // Just load the bytes in the variable and desereialize it and pass forward
            return Ok(PhotonDB { hnsw, dim, path: p });
        } else {
            return Ok(PhotonDB {
                hnsw: HNSW::new(max_elements, dim),
                dim,
                path: p,
            });
        }
    }

    pub fn save(&self) -> io::Result<bool> {
        let bytes = rkyv::to_bytes::<Error>(&self.hnsw).unwrap();
        // let  temp_path = self.path.join("hnsw_database_temp.pho"); // THis is creating a folder
        let db_path = Path::new(&self.path);
        let parent_dir = db_path.parent().unwrap_or(Path::new("."));
        let temp_path = parent_dir.join("hnsw_database_temp.pho");

        let mut file = File::create(&temp_path).expect("Failed to create temp db file ");
        // file = File::open(&temp_path).unwrap();

        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        mmap[..].copy_from_slice(&bytes);
        
        // let res = mmap.wri
        
        mmap.flush()?;


        let res = fs::rename(temp_path, &self.path).expect("FILE RENAME ERROR");

        // fs::write(&self.path, bytes).expect("Error in creating db file ");
        return Ok((true));
    }
    
    pub fn add(&mut self, vec: Vec<f32>) {
         if vec.len() != self.hnsw.vectors.dim {
            panic!("Vector dimension mismatch");
        }
        
    
        let id = self.hnsw.vectors.insert(&vec);

        
        let m = self.hnsw.m;
        let m_max = m; 
        let ef_construction = self.hnsw.ef_construction;
        let m_l = 1.0 / (m as f32).ln();

        self.hnsw.insert(id, m, m_max, ef_construction, m_l);
    }
}

// Log impl

struct Transaction {
    query: u8,
}
pub struct Log {
    time: usize,
    transactions: Transaction,
    path: PathBuf, // self.path for log file
}

impl Log {
    fn append(&self, T: Transaction) -> io::Result<bool> {
        let mut file = File::create(&self.path).expect("Failed to create temp db file ");
        file = File::open(&self.path).unwrap();

        // fs::ap(&self.path, T).expect("Error in creating db file ");

        // file dot append T
        // Here i will get the log of all the transactions which happednon the vector db
        // Insert delete add edge etc , not that search queries only modification and deletion etc

        Ok(true)
    }

    fn read(path: PathBuf) {
        // Read that file parse it
        // And pass appropriate data such that
        // other function can revert the things where went wrong
        // Basically a backup
    }
}
pub struct Buffer {
    db: PhotonDB,
    path: PathBuf,
}

impl Buffer {
    // This will be a copy of the main data base and this will keep comiting the changes to main db
    // all the current chANGES ARE OCCURUNG HERE in this file
    // After some creteria we can commit the changes
    fn commit(&self) {}
}
