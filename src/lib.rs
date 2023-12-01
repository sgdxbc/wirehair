#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fmt::Display, ptr::null_mut, sync::OnceLock};

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidInput,
    BadDenseSeed,
    BadPeelSeed,
    BadInputSmallN,
    BadInputLargeN,
    ExtraInsufficient,
    Error,
    OutOfMemory,
    UnsupportedPlatform,
}

impl From<bindings::WirehairResult> for Error {
    fn from(value: bindings::WirehairResult) -> Self {
        use crate::Error::*;
        use bindings::*;
        match value {
            WirehairResult_t_Wirehair_InvalidInput => InvalidInput,
            WirehairResult_t_Wirehair_BadDenseSeed => BadDenseSeed,
            WirehairResult_t_Wirehair_BadPeelSeed => BadPeelSeed,
            WirehairResult_t_Wirehair_BadInput_SmallN => BadInputSmallN,
            WirehairResult_t_Wirehair_BadInput_LargeN => BadInputLargeN,
            WirehairResult_t_Wirehair_ExtraInsufficient => ExtraInsufficient,
            WirehairResult_t_Wirehair_Error => Error,
            WirehairResult_t_Wirehair_OOM => OutOfMemory,
            WirehairResult_t_Wirehair_UnsupportedPlatform => UnsupportedPlatform,
            _ => unreachable!(),
        }
    }
}

fn to_result(value: bindings::WirehairResult) -> Result<(), Error> {
    if value == bindings::WirehairResult_t_Wirehair_Success {
        Ok(())
    } else {
        Err(Error::from(value))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

fn init() -> Result<(), Error> {
    to_result(unsafe { bindings::wirehair_init_(bindings::WIREHAIR_VERSION as _) })
}

static INIT: OnceLock<Result<(), Error>> = OnceLock::new();

#[derive(Debug)]
pub struct Encoder {
    codec: bindings::WirehairCodec,
    block_bytes: u32,
}

unsafe impl Send for Encoder {}
unsafe impl Sync for Encoder {}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { bindings::wirehair_free(self.codec) }
    }
}

impl Encoder {
    pub fn new(message: &[u8], block_bytes: u32) -> Result<Self, Error> {
        (*INIT.get_or_init(init))?;
        let codec = unsafe {
            bindings::wirehair_encoder_create(
                null_mut(),
                message.as_ptr() as _,
                message.len() as _,
                block_bytes,
            )
        };
        if !codec.is_null() {
            Ok(Self { codec, block_bytes })
        } else {
            Err(Error::Error)
        }
    }

    pub fn encode(&self, block_id: u32) -> Result<Vec<u8>, Error> {
        let mut block_data = vec![0; self.block_bytes as _];
        let mut data_bytes = 0u32;
        to_result(unsafe {
            bindings::wirehair_encode(
                self.codec,
                block_id,
                block_data.as_mut_ptr() as _,
                self.block_bytes,
                (&mut data_bytes) as _,
            )
        })?;
        block_data.truncate(data_bytes as _);
        Ok(block_data)
    }
}

#[derive(Debug)]
pub struct Decoder {
    codec: bindings::WirehairCodec,
    message_bytes: u64,
    block_bytes: u32,
}

unsafe impl Send for Decoder {}
unsafe impl Sync for Decoder {}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { bindings::wirehair_free(self.codec) }
    }
}

impl Decoder {
    pub fn new(message_bytes: u64, block_bytes: u32) -> Result<Self, Error> {
        (*INIT.get_or_init(init))?;
        let codec =
            unsafe { bindings::wirehair_decoder_create(null_mut(), message_bytes, block_bytes) };
        if !codec.is_null() {
            Ok(Self {
                codec,
                message_bytes,
                block_bytes,
            })
        } else {
            Err(Error::Error)
        }
    }

    pub fn decode(&mut self, block_id: u32, block_data: &[u8]) -> Result<bool, Error> {
        let result = unsafe {
            bindings::wirehair_decode(
                self.codec,
                block_id,
                block_data.as_ptr() as _,
                block_data.len() as _,
            )
        };
        if result == bindings::WirehairResult_t_Wirehair_NeedMore {
            Ok(false)
        } else {
            to_result(result).map(|()| true)
        }
    }

    pub fn recover(&self) -> Result<Vec<u8>, Error> {
        let mut message = vec![0; self.message_bytes as _];
        to_result(unsafe {
            bindings::wirehair_recover(self.codec, message.as_mut_ptr() as _, self.message_bytes)
        })?;
        Ok(message)
    }
}

impl TryFrom<Decoder> for Encoder {
    type Error = Error;

    fn try_from(value: Decoder) -> Result<Self, Self::Error> {
        to_result(unsafe { bindings::wirehair_decoder_becomes_encoder(value.codec) })?;
        Ok(Self {
            codec: value.codec,
            block_bytes: value.block_bytes,
        })
    }
}
