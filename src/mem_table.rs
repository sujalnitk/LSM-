use std::collections::Hashmap;
use std::fs::File;

pub struct MemTable {
    map: Hashmap,
    WAL: Option<File>,
    id: usize,
    size: usize,
}
