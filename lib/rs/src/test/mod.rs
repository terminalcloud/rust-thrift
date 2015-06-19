use {Protocol, Transport, Processor, Result, Error};
use protocol::{Encode, Decode, Type, MessageType};

use mock::*;

mod generated {
    use std::collections::{HashMap, HashSet};

    strukt! {
        name = Simple,
        fields = {
            key: String => 16,
        }
    }

    strukt! {
        name = Empty,
        fields = {}
    }

    strukt! {
        name = Nested,
        fields = {
            nested: Vec<Vec<Vec<Simple>>> => 32,
        }
    }

    strukt! {
        name = Recursive,
        fields = {
            recurse: Vec<Recursive> => 0,
        }
    }
}

fn encode<T: Encode>() -> MockProtocol {
    let mut protocol = MockProtocol::new();
    T::default().encode(&mut protocol, &mut &[]).unwrap();
    protocol
}

