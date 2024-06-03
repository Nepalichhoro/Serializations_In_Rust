use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    pub field1: u32,
    pub field2: String,
}

fn main() {
    let original = MyStruct {
        field1: 42,
        field2: String::from("Hello, bincode!"),
    };

    // Serialize
    let serialized = bincode::serialize(&original).unwrap();
    println!("Serialized: {:?}", serialized);
    // logs: Serialized: [42, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 98, 105, 110, 99, 111, 100, 101, 33]

    // Deserialize
    let deserialized: MyStruct = bincode::deserialize(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
    // logs: Deserialized: MyStruct { field1: 42, field2: "Hello, bincode!" }
}
