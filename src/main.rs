mod mem_table;
use crate::mem_table::MemTable;
pub fn main() {
    println!("hello");

    let table = MemTable::new(1, 1024);
}
