use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MyStruct {
    pub field1: u32,
    pub field2: String,
}

fn main() {
    let original = MyStruct {
        field1: 42,
        field2: String::from("Hello, BORSH!"),
    };

    // Serialize
    let serialized = original.try_to_vec().unwrap();
    println!("Serialized: {:?}", serialized);
    // logs: Serialized: [42, 0, 0, 0, 13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 66, 79, 82, 83, 72, 33]

    // Deserialize
    let deserialized: MyStruct = MyStruct::try_from_slice(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
    // logs: Deserialized: MyStruct { field1: 42, field2: "Hello, BORSH!" }

}
