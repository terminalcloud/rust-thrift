// service! {
//     trait_name = SharedService,
//     processor_name = SharedServiceProcessor,
//     client_name = SharedServiceClient,
//     methods = [
//         SharedServiceGetStructArgs -> SharedServiceGetStructResult = shared.getStruct(key: i32 => 1) -> SharedStruct
//     ],
//     generics = <S>,
//     bounds = <S: SharedService>,
//     fields = [shared: S]
// }

#[macro_export]
macro_rules! service {
    (trait_name = $trait_name:ident,
     processor_name = $processor_name:ident,
     client_name = $client_name:ident,
     methods = $methods:tt,
     generics = $generics:tt,
     bounds = $bounds:tt) => {
        service_trait! {
            trait_name = $trait_name,
            methods = $methods
        }

        service_processor! {
            processor_name = $processor_name,
            methods = $methods,
            generics = $generics,
            bounds = $bounds
        }

        service_client! {
            client_name = $client_name,
            methods = $methods
        }
    }
}

macro_rules! service_trait {
    (trait_name = $name:ident,
     methods = $methods:tt) => {
        pub trait $name {
            service_trait_methods! { $methods }
        }
    }
}

macro_rules! service_trait_methods {
    ([$($iname:ident -> $oname:ident = $fname.$mname:ident($($aname:ident: $aty:ty => $aid:expr),*) -> $rty:ty),+]) => {
        $(fn $mname(&mut self, $($aname: $aty)*) -> $rty;)+
    }
}

macro_rules! service_processor {
    (processor_name = $name:ident,
     methods = $methods:tt,
     generics = $generics:tt,
     bounds = $bounds:tt,
     fields = [$fields:tt]) => {
        pub struct $name $bounds {
            $fields
        }

        impl $bounds $name $generics {
            service_processor_constructor! {
                name = $name,
                fields = $fields
            }
            service_processor_methods! { $methods }
        }

        service_processor_processor_impl! {
            name = $name,
            bounds = $bounds,
            generics = $generics
        }
    }
}

macro_rules! service_processor_fields { ([$fields:tt]) => { $fields } }

macro_rules! service_processor_constructor {
    (name = $name:ident,
     fields = $($fname:ident: $fty: ty)+) => {
        pub fn new($($fname: $fty)+) -> Self {
            $name { $($fname: $fname,)+ }
        }
    }
}

macro_rules! service_processor_methods {
    ([$($iname:ident -> $oname:ident = $fname.$mname:ident($($aname:ident: $aty:ty => $aid:expr),*) -> $rty:ty),+]) => {
        pub fn dispatch<P: $crate::Protocol, T: $crate::Transport>(&mut self, prot: &mut P, transport: &mut T,
                                                                   name: &str, ty: $crate::protocol::MessageType, id: i32) -> Result<()> {
            match name {
                $(stringify!($mname) => self.$mname(prot, transport, ty, id))+,
                _ => panic!() // TODO: Use an error
            }
        }

        $(fn $mname<P: $crate::Protocol, T: $crate::Transport>(&mut self, prot: &mut P, transport: &mut T,
                                                               ty: $crate::protocol::MessageType, id: i32) -> Result<()> {
            static mname: &'static str = stringify!($mname);

            strukt! {
                name = $iname,
                fields = {
                    $($aname: $aty => $aid,)*
                }
            }

            strukt! {
                name = $oname,
                fields = {
succes: $rty => 0
                }
            }

            let mut args = $iname::default();
            try!(protocol::helpers::receive_body(prot, transport, mname, &mut args, mname, ty, id));

            let mut result = $oname::default();
            result.success = self.$fname.$mname($(args.$aname)*);

            try!(protocol::helpers::send(prot, transport, mname,
                                         $crate::protocol::MessageType::Reply, &result));

            Ok(())
        })+
    }
}

macro_rules! service_processor_processor_impl {
    (name = $name:ident,
     bounds = <$($boundty:ty: $bound:ident),*>,
     generics = $generics:tt) => {
        impl<P: $crate::Protocol, T: Tranport, $($boundty: $bound,)*> Processor for $name $generics {
            fn process(&mut self, protocol: &mut P, transport: &mut T) -> Result<()> {
                let (name, ty, id) = try!(protocol.read_message_begin(transport));
                self.dispatch(protocol, transport, name, ty, id)
            }
        }
    }
}

macro_rules! service_client {
    (client_name = $client_name:ident,
     methods = [$($iname:ident -> $oname:ident = $fname.$mname:ident($($aname:ident: $aty:ty => $aid:expr),*) -> $rty:ty),+]) => {
        pub struct $client_name<P: $crate::Protocol, T: $crate::Transport> {
            pub protocol: P,
            pub transport: T
        }

        impl<P: $crate::Protocol, T: $crate::Transport> $client_name<P, T> {
            pub fn new(protocol: P, transport: T) -> Self {
                $client_name {
protocol: protocol,
transport: transport
                }
            }

            $(pub fn $mname(&mut self, $($aname: $aty,)*) -> Result<$rty> {
                static mname: &'static str = stringify!($mname);

                let args = $iname { $($aname: $aname,)* };
                try!(protocol::helpers::send(&mut self.protocol, &mut self.transport,
                                             mname, $crate::protocol::MessageType::Call, &args));

                let mut result = $oname::default();
                try!(protocol::helpers::receive(&mut self.protocol, &mut self.transport,
                                                mname, &mut result));

                Ok(result.success)
            })+
        }
    }
}

#[macro_export]
macro_rules! strukt {
    (name = $name:ident,
     fields = { $($fname:ident: $fty:ty => $id:expr),* }) => {
        struct $name {
            $($fname: $fty,)*
        }

        impl Encode for $name {
            fn encode<P, T>(&self, protocol: &mut P, transport: &mut T) -> Result<()>
                where P: $crate::Protocol, T: $crate::Transport {
                    strukt_encode_impl! {
                        name = $name,
                        selff = self,
                        protocol = protocol,
                        transport = transport,
                        fields = { $($fname: $fty => $id),* }
                    }
                }
        }

        impl Decode for $name {
            fn decode<P, T>(&mut self, protocol: &mut P, transport: &mut T) -> Result<()>
                where P: $crate::Protocol, T: $crate::Transport {
                    strukt_decode_impl! {
                        name = $name,
                        selff = self,
                        protocol = protocol,
                        transport = transport,
                        fields = { $($fname: $fty => $id),* }
                    }
                }
        }
    };
    (pub $rest:tt) => { pub strukt! { $rest } }
}

macro_rules! strukt_encode_impl {
    (name = $name:ident,
     selff = $selff:ident,
     protocol = $protocol:ident,
     transport = $transport:ident,
     fields = { $($fname:ident: $fty:ty => $fid:expr),* }) => {{
        static name: &'static str = stringify!($name);

        try!(protocol.write_struct_begin(transport, name));

        { $(strukt_encode_field!($fname, $selff.$fname, $fty, $fid, $protocol, $transport);)* };

        try!(oprot.write_field_stop(transport));
        try!(protocol.write_struct_end(transport));
        Ok(())
    }}
}

macro_rules! strukt_encode_field {
    ($name:ident, $target:expr, $ty:ty, $id:expr, $protocol:ident, $transport:ident) => {{
        macro_rules! strukt_encode_field_prim {
            ($wmethod:ident) => {
                try!($protocol.write_field_begin($transport, stringify!($name), ty_to_type!($ty), $id));
                try!($protocol.$wmethod($transport, $target));
                try!($protocol.write_field_end($transport));
            }
        }

        macro_rules! encode_match_ty {
            (bool) => strukt_encode_field_prim!(write_bool),
            (u8) => strukt_encode_field_prim!(write_byte),
            (i16) => strukt_encode_field_prim!(write_i16),
            (i32) => strukt_encode_field_prim!(write_i32),
            (i64) => strukt_encode_field_prim!(write_i64),
            (String) => strukt_encode_field_prim!(write_string),
            (HashMap<$key:ty, $value:ty>) => {
                try!($protocol.write_map_begin($transport,
                                               ty_to_type!($key),
                                               ty_to_type!($value),
                                               $target.len()));

                for (key, value) in &$target {
                    strukt_encode_field!($name, key, $key, $id, $protocol, $transport);
                    strukt_encode_field!($name, value, $value, $id, $protocol, $transport);
                }

                try!($protocol.write_map_end($transport));
            }
            (HashSet<$v:ty>) => {
                try!($protocol.write_set_begin($transport, ty_to_type!($v), $target.len()));

                for el in &$target {
                    strukt_encode_field!($name, el, $v, $id, $protocol, $transport);
                }

                try!($protocol.write_set_end($transport));
            }
            (Vec<$v:ty>) => {
                try!($protocol.write_list_begin($transport, ty_to_type!($v), $target.len()));

                for el in &$target {
                    strukt_encode_field!($name, el, $v, $id, $protocol, $transport);
                }

                try!($protocol.write_list_end($transport));
            }
            (Option<$v:ty>) => {
                if let Some(ref x) = $target {
                    strukt_encode_field!($name, x, $v, $id, $protocol, $transport)
                }
            }
            ($ty:ty) => {
                try!($target.encode($protocol, $transport));
            }
        }

        encode_match_ty!($ty);
    }}
}

macro_rules! ty_to_type {
    (bool) => { $crate::protocol::Type::Bool };
    (u8) => { $crate::protocol::Type::Byte };
    (i16) => { $crate::protocol::Type::I16 };
    (i32) => { $crate::protocol::Type::I32 };
    (i64) => { $crate::protocol::Type::I64 };
    (String) => { $crate::protocol::Type::String };
    (HashMap<$k:ty, $v:ty>) => { $crate::protocol::Type::Map };
    (HashSet<$v:ty>) => { $crate::protocol::Type::Set };
    (Vec<$v:ty>) => { $crate::protocol::Type::List };
    ($ty:ty) => { $crate::protocol::Type::Struct };
}

macro_rules! strukt_decode_impl {
    (name = $name:ident,
     selff = $selff:ident,
     protocol = $protocol:ident,
     transport = $transport:ident,
     fields = { $($fname:ident: $fty:ty => $fid:expr),* }) => {{
        let mut has_result = false;
        try!($protocol.read_struct_begin($transport));

        loop {
            match try!($protocol.read_field_begin($transport)) {
                (_, $crate::protocol::Type::TStop, _) => {
                    try!($protocol.read_field_end($transport));
                    break;
                },
                $((_, ty_to_type!($fty), $fid) =>
                    strukt_decode_block!($fname, $selff.$fname, $fty, $fid, $protocol, $transport, has_result),)*
                (_, ftype, _) => {
                    try!($protocol.skip($transport, ftype));
                }
            }

            try!($protocol.read_field_end($transport));
        }

        try!($protocol.read_struct_end($transport));

        if has_result {
            Ok(())
        } else {
            Err($crate::Error::from($crate::protocol::Error::ProtocolViolation))
        }
    }}
}

macro_rules! strukt_decode_block {
    ($name:ident, $target:expr, $ty:ty, $id:expr, $protocol:ident, $transport:ident, $result:ident) => {{
        macro_rules! strukt_decode_block_prim {
            ($rmethod:ident) => {
                $target = try!($protocol.$rmethod(transport));
                $result = true;
            },
        }

        macro_rules! decode_match_ty {
            (bool) => strukt_decode_block_prim!(read_bool),
            (u8) => strukt_decode_block_prim!(read_byte),
            (i16) => strukt_decode_block_prim!(read_i16),
            (i32) => strukt_decode_block_prim!(read_i32),
            (i64) => strukt_decode_block_prim!(read_i64),
            (String) => strukt_decode_block_prim!(read_string),
            (HashMap<$key:ty, $value:ty>) => {
                if let (ty_to_type!($key), ty_to_type!($value), len) = try!($protocol.read_set_begin($transport)) {
                    let mut map = ::std::collections::HashMap::with_capacity(len);

                    for _ in (0..len) {
                        let key = $key::default();
                        let value = $value::default();

                        try!(key.decode($protocol, $transport));
                        try!(value.decode($protocol, $transport));

                        map.insert(key, value);
                    }

                    $target = map;
                } else {
                    return Err($crate::Error::from($crate::protocol::Error::ProtocolViolation));
                }
            },
            (HashSet<$v:ty>) => {
                if let (ty_to_type!($v), len) = try!($protocol.read_set_begin($transport)) {
                    let mut set = ::std::collections::HashSet::with_capacity(len);

                    for _ in (0..len) {
                        let x = $v::default();
                        try!(x.decode($protocol, $transport));
                        set.insert(x);
                    }

                    $target = set;
                } else {
                    return Err($crate::Error::from($crate::protocol::Error::ProtocolViolation));
                }
            },
            (Vec<$v:ty>) => {
                if let (ty_to_type!($v), len) = try!($protocol.read_list_begin($transport)) {
                    let mut list = Vec::with_capacity(len);

                    for _ in (0..len) {
                        let x = $v::default();
                        try!(x.decode($protocol, $transport));
                        list.push(x);
                    }

                    $target = list;
                } else {
                    return Err($crate::Error::from($crate::protocol::Error::ProtocolViolation));
                }
            },
            (Option<$v:ty>) => {
                $target = Some(try!($protocol.$rmethod(transport)));
                $result = true;
            },
            ($v:ty) => {
                try!($target.decode($protocol, $transport));
                $result = true;
            }
        }

        decode_match_ty!($ty)
    }}
}

