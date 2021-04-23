use uuid::Uuid;
use sled::{Db, Tree};

use std::env;
use std::path::PathBuf;

use crate::{ Description, Interval };

pub struct Database {
    db: Db,
    cycles: Tree
}

impl Database {

    pub fn new() -> Database {
        let db = or_panic!(Res sled::open(&file_path()));
        let cycles = or_panic!(Res db.open_tree("cycles"));
        Database { db, cycles }
    }

    pub fn append(&self, description: Description) -> Uuid {
        let id = Uuid::new_v4();
        self.update(id, description);
        id
    }

    pub fn update(&self, id: Uuid, description: Description) {
        let data = or_panic!(Res serde_cbor::to_vec(&description));
        or_panic!(Res self.cycles.insert(id.as_bytes(), data));
    }

    pub fn remove(&self, id: Uuid) {
        or_panic!(Res self.cycles.remove(id.as_bytes()));
    }

    pub fn entries(&self) -> impl Iterator<Item=(Uuid, Description)> {
        self.cycles.iter().map(|x|x.map(|(key, value)| {
            (
                or_panic!(Res Uuid::from_slice(&key)),
                or_panic!(Res serde_cbor::from_slice(&value))
            )
        })).flatten()
    }

    pub fn export_to_writer<W: std::io::Write>(&self, w: W) {
        let entries: Vec<(Uuid, Description)> = self.entries().collect();
        or_panic!(Res serde_cbor::to_writer(w, &entries));
    }

    pub fn import_from_reader<R: std::io::Read>(&self, r: R) -> Vec<(Uuid, Description)> {
        let data = or_panic!(Res serde_cbor::from_reader::<Vec<(Uuid, Description)>, R>(r));
        for (key, value) in &data {
            self.update(key.clone(), value.clone());
        }
        data
    }
}

fn file_path() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .map(|mut x|{x.push(".local/share/cycles/database");x})
        .expect("HOME Environment Variable not set")
}
