use std::path::PathBuf;
use photon::persistence::{self, PhotonDB};

fn main() {
    let p = PathBuf::from("/home/eleven/Rust/projects-jan/photon/src/");
    // This returns the Option 
    // let c = p.join("database.pho");
    // println!("{:?}" , c);

    let mut db = PhotonDB::load_or_create(p, 100, 4).unwrap();
    println!("{:?}", db); // Created empty Database
      let v1 = vec![1.0, 1.0, 1.0, 1.0];
        let v2 = vec![2.0, 2.0, 2.0, 2.0];
        let v3 = vec![1.1, 1.1, 1.1, 1.1]; 

       db.add(v1);
    db.save();

}
