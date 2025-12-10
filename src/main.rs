mod mem_table;
use crate::mem_table::MemTable;
pub fn main() {
    println!("hello");

    let table = MemTable::new(1, 1024);
    println!("{}", table.get_size_limit())
}
