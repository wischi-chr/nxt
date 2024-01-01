//! Types and functionality related to motor functions

use crate::{Error, Result};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ops::BitOr;

pub const RUN_FOREVER: u32 = 0;

// supported ports are 0, 1, 2 == A, B, C
// 3 == AB, 4 == AC, 5 == BC, 6 == ABC
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum OutPort {
    A = 0,
    B = 1,
    C = 2,
    AB = 3,
    AC = 4,
    BC = 5,
    ABC = 6,
    //0xFF is protocol defined to mean "all ports".
    All = 0xff,
}

impl TryFrom<u8> for OutPort {
    type Error = Error;
    fn try_from(code: u8) -> Result<Self> {
        Self::from_u8(code).ok_or(Error::Parse("Invalid OutPort"))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub struct OutMode(pub(crate) u8);
impl OutMode {
    pub const IDLE: Self = Self(0x00);
    pub const ON: Self = Self(0x01);
    pub const BRAKE: Self = Self(0x02);
    pub const REGULATED: Self = Self(0x04);
}

impl From<u8> for OutMode {
    fn from(code: u8) -> Self {
        Self(code)
    }
}

impl BitOr<OutMode> for OutMode {
    type Output = Self;
    fn bitor(self, other: OutMode) -> Self {
        Self(self.0 | other.0)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum RegulationMode {
    #[default]
    Idle = 0,
    Speed = 1,
    Sync = 2,
}

impl TryFrom<u8> for RegulationMode {
    type Error = Error;
    fn try_from(code: u8) -> Result<Self> {
        Self::from_u8(code).ok_or(Error::Parse("Invalid RegulationMode"))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum RunState {
    Idle = 0x00,
    RampUp = 0x10,
    Running = 0x20,
    RampDown = 0x40,
}

impl TryFrom<u8> for RunState {
    type Error = Error;
    fn try_from(code: u8) -> Result<Self> {
        Self::from_u8(code).ok_or(Error::Parse("Invalid RunState"))
    }
}

#[derive(Debug)]
pub struct OutputState {
    pub port: OutPort,
    pub power: i8,
    pub mode: OutMode,
    pub regulation_mode: RegulationMode,
    pub turn_ratio: i8,
    pub run_state: RunState,
    pub tacho_limit: u32,
    pub tacho_count: i32,
    pub block_tacho_count: i32,
    pub rotation_count: i32,
}
