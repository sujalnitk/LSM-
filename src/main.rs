mod mem_table;
use crate::mem_table::MemTable;
pub fn main() {
    println!("hello");

    let mut table = MemTable::new(1, 1024);
    println!("{}", table.get_size_limit());
    table.insert(b"sujal", b"user");
    table.insert(b"sujal", b"user1");
    table.insert(b"key", b"value");

    println!("{:?}", table.get_map());
}
