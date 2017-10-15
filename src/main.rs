extern crate byteorder;

use byteorder::{NetworkEndian, WriteBytesExt};

fn main() {
    let mut writer = Vec::new();
    writer.write_u8(10);
    writer.write_string(String::from("Trangar"));
    writer.write_string(String::from("Password"));

    let bin = writer;
    println!("{:?}", std::str::from_utf8(&bin));
    println!("{:?}", parse(&bin));
}

#[derive(Debug)]
pub enum Network {
    Connected {
        version: String
    },
    Login {
        username: String,
        password: String,
    },
    Move {
        entity_id: u64,
        x: f64,
        y: f64
    },
    __UNUSED
}

#[derive(Debug)]
pub enum ParseResult<T, E> {
    Ok(T),
    NotEnoughBytes,
    Err(E)
}

#[derive(Debug)]
pub enum ParseError {
    InvalidNetworkType
}

pub mod inner {
    use ::{ParseResult, ParseError};
    use std::{mem, ptr};
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum TYPE {
        Connected,
        Login,
        Move,
        __UNUSED
    }

    impl TYPE {
        pub fn size() -> usize { mem::size_of::<TYPE>() }
        pub fn parse(data: &[u8]) -> ParseResult<TYPE, ParseError> {
            println!("data len: {}, TYPE::size: {}", data.len(), TYPE::size());
            if data.len() < TYPE::size() { return ParseResult::NotEnoughBytes; }
            let mut inner_type: TYPE = unsafe { mem::uninitialized() };
            unsafe { ptr::copy(data as *const _ as *const TYPE, &mut inner_type as *mut _, TYPE::size()) };
            if inner_type == TYPE::__UNUSED {
                ParseResult::Err(ParseError::InvalidNetworkType)
            } else {
                ParseResult::Ok(inner_type)
            }
        }
    }

    pub struct Connected {
        pub version: String,
    }

    pub struct Login {
        pub username: String,
        pub password: String,
    }

    pub struct Move {
        pub entity_id: u64,
        pub x: f64,
        pub y: f64
    }
}

fn parse(data: &[u8]) -> ParseResult<Network, ParseError> {
    let inner_type = match inner::TYPE::parse(data) {
        ParseResult::Ok(t) => t,
        ParseResult::Err(e) => return ParseResult::Err(e),
        ParseResult::NotEnoughBytes => return ParseResult::NotEnoughBytes
    };

    println!("INner type: {:?}", inner_type);

    ParseResult::NotEnoughBytes
}
