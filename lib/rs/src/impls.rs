use protocol::{Encode, Decode, Type};
use {Protocol, Transport};

use std::{any, mem};

macro_rules! prim_encode {
    ($($T:ty => $method:ident),*) => {
        $(impl Encode for $T {
            fn encode<P, T>(&self, protocol: &mut P, transport: &mut T) -> Result<()>
            where P: Protocol, T: Transport {
                try!(protocol.$method(transport));
                Ok(())
            }
        })*
    }
}

macro_rules! prim_decode {
    ($($T:ty => $method:ident),*) => {
        $(impl Decode for $T {
            fn decode<P, T>(&mut self, protocol: &mut P, transport: &mut T) -> Result<()>
            where P: Protocol, T: Transport {
                self = try!(protocol.$method(transport));
                Ok(())
            }
        })*
    }
}

prim_encode! {
    bool => write_bool,
    i8 => write_i8,
    i16 => write_i16,
    i32 => write_i32,
    i64 => write_i64,
    String => write_string
}

prim_decode! {
    bool => read_bool,
    i8 => read_i8,
    i16 => read_i16,
    i32 => read_i32,
    i64 => read_i64,
    String => read_string
}

macro_rules! match_generic_type {
    ($m:expr, $o:ident, $T:ty, $t1:ty => $e1:expr) => {{
        if any::TypeId::of::<$T>() == any::TypeId::of::<$t1>() {
            let mut $o = unsafe { mem::transmute::<$T, $t1>() }
            $e1
        }
    }};

    ($m:expr, $T:ty, $t1:ty => $e1:expr, $($t:ty => $e:expr),+) => {{
        if any::TypeId::of::<$T>() == any::TypeId::of::<$t1>() {
            let mut $o = unsafe { mem::transmute::<$T, $t1>() }
            $e1
        } else {
            match_generic_type!($m, $T, $($t => $e),*)
        }
    }}
}

impl<E> Encode for Vec<E> where E: Any {
    fn encode<P, T>(&self, protocol: &mut P, transport: &mut T) -> Result<()>
    where P: Protocol, T: Transport {
        #[inline]
        fn encode_list<P, T, E>(protocol: &mut P, transport: &mut T, buf: &[E], kind: Type) -> Result<()> {
            try!(protocol.write_list_begin(transport, kind, buf.len()));
            for el in buf { try!(el.encode(protocol, transport)); }
            try!(protocol.write_list_end(transport));
        }

        match_generic_type! { &self, this, Vec<T>,
            Vec<u8> => try!(protocol.write_binary(this)),
            Vec<bool> => try!(encode_list(protocol, transport, this, Type::Bool)),
            Vec<i16> => try!(encode_list(protocol, transport, this, Type::I16)),
            Vec<i32> => try!(encode_list(protocol, transport, this, Type::I32)),
            Vec<i64> => try!(encode_list(protocol, transport, this, Type::I64)),
            Vec<String> => try!(encode_list(protocol, transport, this, Type::String)),
            Vec<E> => try!(encode_list(protocol, transport, this, Type::Struct)),
        }

        Ok(())
    }
}

