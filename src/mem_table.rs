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

    pub fn get_id(&self) -> usize {
        return self.id;
    }

    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        let entry_size: usize = key.len() + value.len();
        if self.curr_size + entry_size > self.size_limit {
            println!("MemTable is full! (TODO: Implement Flush to Disk)");
            return;
        }

        if let Some(ref mut file) = self.wal {
            file.write_all(key).unwrap();
            file.write_all(b" ").unwrap();
            file.write_all(value).unwrap();
            file.write_all(b"\n").unwrap();
        }

        self.map.insert(key.to_vec(), value.to_vec());
        self.curr_size += entry_size;

        println!(
            "Inserted. Current size: {}/{}",
            self.curr_size, self.size_limit
        );
    }
}
