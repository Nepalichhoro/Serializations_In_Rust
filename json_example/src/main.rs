use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    pub field1: u32,
    pub field2: String,
}

fn main() {
    let original = MyStruct {
        field1: 42,
        field2: String::from("Hello, Serde!"),
    };

    // Serialize
    let serialized = serde_json::to_string(&original).unwrap();
    println!("Serialized: {:?}", serialized);

    // Deserialize
    let deserialized: MyStruct = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
