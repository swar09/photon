use std::path::PathBuf;
use photon::persistence::PhotonDB;

fn main() {
    let p = PathBuf::from("/home/eleven/Rust/projects-jan/photon/src/");
    // This returns the Option 
    // let c = p.join("database.pho");
    // println!("{:?}" , c);

    let mut db = PhotonDB::load_or_create(p, 100, 4).unwrap();
   
}
