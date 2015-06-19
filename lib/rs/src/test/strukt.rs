use protocol::{Type, Decode};

use test::*;
use mock::*;
use test::generated::*;

#[test]
fn test_simple_struct() {
    let instance = Simple { key: String::from("Hello World!") };
    let mut protocol = encode(instance.clone());

    assert_eq!(protocol.log(), &[
        Struct(Begin(String::from("Simple"))),
        Field(Begin((String::from("key"), Type::String, 16))),
        Prim(PString(String::from("Hello World!"))),
        Field(End),
        field_end(),
        Struct(End)
    ]);

    let mut second = Simple::default();
    second.decode(&mut protocol, &mut MockTransport::new(vec![])).unwrap();

    assert_eq!(instance.key, second.key);
}

