use std::collections::{HashMap, HashSet};

use {Protocol, Transport, Processor, Result, Error};
use protocol::{Encode, Decode, Type, MessageType};

use self::ProtocolAction::*;
use self::SerAction::*;
use self::Primitive::*;
use self::Action::*;

#[derive(Debug, Default, Clone)]
pub struct MockProtocol {
    log: Vec<ProtocolAction>
}

pub type MockTransport = Vec<u8>;

impl MockProtocol {
    pub fn new() -> Self { Default::default() }
    pub fn log(&self) -> &[ProtocolAction] { &self.log }

    fn log_action(&mut self, action: ProtocolAction) -> Result<()> {
        self.log.push(action);
        Ok(())
    }
}

// omg
impl Protocol for MockProtocol {
    fn write_message_begin<T: Transport>(&mut self, transport: &mut T, name: &str,
                                         message_type: MessageType, sequence_id: i32) -> Result<()> {
        self.log_action(Write(Message(Begin((String::from(name), message_type, sequence_id)))))
    }

    fn write_message_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(Message(End)))
    }

    fn write_struct_begin<T: Transport>(&mut self, transport: &mut T, name: &str) -> Result<()> {
         self.log_action(Write(Struct(Begin(String::from(name)))))
    }

    fn write_struct_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(Struct(End)))
    }

    fn write_field_begin<T: Transport>(&mut self, transport: &mut T, name: &str, field_type: Type, field_id: i16) -> Result<()> {
        self.log_action(Write(Field(Begin((String::from(name), field_type, field_id)))))
    }

    fn write_field_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(Field(End)))
    }

    fn write_field_stop<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(Field(Stop)))
    }

    fn write_map_begin<T: Transport>(&mut self, transport: &mut T, key_type: Type,
                                     value_type: Type, size: usize) -> Result<()> {
         self.log_action(Write(Map(Begin((key_type, value_type, size)))))
    }

    fn write_map_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(Map(End)))
    }

    fn write_list_begin<T: Transport>(&mut self, transport: &mut T, elem_type: Type, size: usize) -> Result<()> {
        self.log_action(Write(List(Begin((elem_type, size)))))
    }

    fn write_list_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(List(End)))
    }

    fn write_set_begin<T: Transport>(&mut self, transport: &mut T, elem_type: Type, size: usize) -> Result<()> {
        self.log_action(Write(Set(Begin((elem_type, size)))))
    }

    fn write_set_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        self.log_action(Write(Set(End)))
    }

    fn write_bool<T: Transport>(&mut self, transport: &mut T, value: bool) -> Result<()> {
        self.log_action(Write(Bool(value)))
    }

    fn write_byte<T: Transport>(&mut self, transport: &mut T, value: i8) -> Result<()> {
        self.log_action(Write(Byte(value)))
    }

    fn write_i16<T: Transport>(&mut self, transport: &mut T, value: i16) -> Result<()> {
        self.log_action(Write(I16(value)))
    }

    fn write_i32<T: Transport>(&mut self, transport: &mut T, value: i32) -> Result<()> {
        self.log_action(Write(I32(value)))
    }

    fn write_i64<T: Transport>(&mut self, transport: &mut T, value: i64) -> Result<()> {
        self.log_action(Write(I64(value)))
    }

    fn write_double<T: Transport>(&mut self, transport: &mut T, value: f64) -> Result<()> {
        self.log_action(Write(Double(value)))
    }

    fn write_str<T: Transport>(&mut self, transport: &mut T, value: &str) -> Result<()> {
        self.log_action(Write(PString(String::from(value))))
    }

    fn write_string<T: Transport>(&mut self, transport: &mut T, value: &String) -> Result<()> {
        self.log_action(Write(PString(String::from(value))))
    }

    fn write_binary<T: Transport>(&mut self, transport: &mut T, value: &[u8]) -> Result<()> {
        self.log_action(Write(Binary(Vec::from(value))))
    }

    fn read_message_begin<T: Transport>(&mut self, transport: &mut T) -> Result<(String, MessageType, i32)> {
        read!(self, Message(Begin((name, type_, id))), (name, type_, id))
    }

    fn read_message_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        read!(self, Message(End))
    }

    fn read_struct_begin<T: Transport>(&mut self, transport: &mut T) -> Result<String> {
        read!(self, Struct(Begin(name)), name)
    }

    fn read_struct_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        read!(self, Struct(End))
    }

    fn read_field_begin<T: Transport>(&mut self, transport: &mut T) -> Result<(String, Type, i16)> {
        read!(self, Field(Begin((name, type_, id))), (name, type_, id))
    }

    fn read_field_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        read!(self, Field(End))
    }

    fn read_map_begin<T: Transport>(&mut self, transport: &mut T) -> Result<(Type, Type, i32)> {
        read!(self, Map(Begin((keyt, valuet, len))), (keyt, valuet, len as i32))
    }

    fn read_map_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
        read!(self, Map(End))
    }

    fn read_list_begin<T: Transport>(&mut self, transport: &mut T) -> Result<(Type, i32)> {
        read!(self, List(Begin((ty, len))), (ty, len as i32))
    }
    fn read_list_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
         read!(self, List(End))
    }

    fn read_set_begin<T: Transport>(&mut self, transport: &mut T) -> Result<(Type, i32)> {
        read!(self, Set(Begin((ty, len))), (ty, len as i32))
    }

    fn read_set_end<T: Transport>(&mut self, transport: &mut T) -> Result<()> {
         read!(self, Set(End))
    }

    fn read_bool<T: Transport>(&mut self, transport: &mut T) -> Result<bool> { read!(self, Bool(val), val) }
    fn read_byte<T: Transport>(&mut self, transport: &mut T) -> Result<i8> { read!(self, Byte(val), val) }
    fn read_i16<T: Transport>(&mut self, transport: &mut T) -> Result<i16> { read!(self, I16(val), val) }
    fn read_i32<T: Transport>(&mut self, transport: &mut T) -> Result<i32> { read!(self, I32(val), val) }
    fn read_i64<T: Transport>(&mut self, transport: &mut T) -> Result<i64> { read!(self, I64(val), val) }
    fn read_double<T: Transport>(&mut self, transport: &mut T) -> Result<f64> { read!(self, Double(val), val) }
    fn read_string<T: Transport>(&mut self, transport: &mut T) -> Result<String> { read!(self, PString(string), string) }
    fn read_binary<T: Transport>(&mut self, transport: &mut T) -> Result<Vec<u8>> { read!(self, Binary(val), val) }

    fn skip<T: Transport>(&mut self, transport: &mut T, type_: Type) -> Result<()> {
        // TODO: Implement *checked* skipping
        if self.log.len() != 0 { self.log.pop(); }
        Ok(())
    }
}

macro_rules! read {
    ($selff:expr, $expected:pat, $body:expr) => {
        match $selff.log.pop() {
             Some(Write($expected)) => Ok($body),
             Some(other) => {
                 panic!(concat!("Unexpected read. Expected ", stringify!($expected),
                        ", encountered {:?}. Log was: {:?}"), &other, &$selff.log)
             },
             None => {
                 panic!(concat!("Unexpected read on empty log. Expected ", stringify!($expected)))
             }
        }
    };
    ($selff:expr, $expected:pat) => { read!($selff, $expected, ()) }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProtocolAction {
    Read(SerAction),
    Write(SerAction),
    Skip
}

#[derive(Debug, PartialEq, Clone)]
pub enum SerAction {
    Message(Action<(String, MessageType, i32)>),
    Struct(Action<String>),
    Field(Action<(String, Type, i16)>),
    Map(Action<(Type, Type, usize)>),
    List(Action<Type, usize>),
    Set(Action<Type, usize>),
    Primitive(Primitive)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action<B> { Begin(B), End, Stop }

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Primitive {
    Bool(bool),
    Double(f64),
    Byte(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    PString(String),
    Binary(Vec<u8>)
}

