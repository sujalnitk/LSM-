use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct MemTable {
    map: HashMap<Vec<u8>, Vec<u8>>,
    wal: Option<File>,
    id: usize,
    curr_size: usize,
    size_limit: usize,
}

impl MemTable {
    pub fn new(id: usize, size_limit: usize) -> Self {
        let wal_file = format!("wal_{}.log", id);
        let file = File::create(&wal_file).expect("Could not create wal file");

        Self {
            map: HashMap::new(),
            wal: Some(file),
            id,
            curr_size: 0,
            size_limit,
        }
    }

    pub fn get_size_limit(&self) -> usize {
        return self.size_limit;
    }
}
