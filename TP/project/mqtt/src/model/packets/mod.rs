/// CONNACK
pub mod connack;
/// CONNECT
pub mod connect;
/// DISCONNECT
pub mod disconnect;
/// PINGREQ
pub mod pingreq;
/// PINGRESP
pub mod pingresp;
/// PUBACK
pub mod puback;
/// PUBLISH
pub mod publish;
/// SUBACK
pub mod suback;
/// SUBSCRIBE
pub mod subscribe;
/// UNSUBACK
pub mod unsuback;
/// UNSUBSCRIBE
pub mod unsubscribe;

pub const CONNECT_PACKET_TYPE: u8 = 0x1;
pub const CONNACK_PACKET_TYPE: u8 = 0x2;
pub const PUBLISH_PACKET_TYPE: u8 = 0x3;
pub const PUBACK_PACKET_TYPE: u8 = 0x4;
pub const SUBSCRIBE_PACKET_TYPE: u8 = 0x8;
pub const SUBACK_PACKET_TYPE: u8 = 0x9;
pub const UNSUBSCRIBE_PACKET_TYPE: u8 = 0xA;
pub const UNSUBACK_PACKET_TYPE: u8 = 0xB;
pub const PINGREQ_PACKET_TYPE: u8 = 0xC;
pub const PINGRESP_PACKET_TYPE: u8 = 0xD;
pub const DISCONNECT_PACKET_TYPE: u8 = 0xE;

const RESERVED_FIXED_HEADER_FLAGS: u8 = 0x00;

const DEFAULT_VARIABLE_HEADER_LENGTH: usize = 2;
