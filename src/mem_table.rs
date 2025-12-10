use std::collections::Hashmap;
use std::fs::File;
use std::io::Write;

pub struct MemTable {
    map: Hashmap<Vec<u8>, Vec<u8>>,
    WAL: Option<File>,
    id: usize,
    size: usize,
}

impl MemTable {
    pub fn new(id: usize, size: usize) -> Self {
        let wal_file = format!("wal_{}.log", id);
        let file = File::create(&wal_file).expect("Could not create wal file");

        Self {
            map: Hashmap::new(),
            WAL: Some(file),
            id,
            size,
        }
    }
}

fn main {
    let table = MemTable::new();
}
