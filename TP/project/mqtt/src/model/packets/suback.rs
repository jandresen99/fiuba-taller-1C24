use super::{DEFAULT_VARIABLE_HEADER_LENGTH, RESERVED_FIXED_HEADER_FLAGS, SUBACK_PACKET_TYPE};
use crate::{
    encrypt, errors::error::MqttResult, FixedHeader, MqttError, Read, RemainingLength,
    SubackReturnCode,
};

/// Represents a SUBACK packet of MQTT. The server uses it to confirm the subscription to one or more topics.
#[derive(Debug)]
pub struct Suback {
    packet_identifier: u16,
    suback_return_codes: Vec<SubackReturnCode>,
}

impl Suback {
    pub fn new(packet_identifier: u16, suback_return_codes: Vec<SubackReturnCode>) -> Self {
        Self {
            packet_identifier,
            suback_return_codes,
        }
    }

    /// Converts a stream of bytes into a Suback.
    pub fn from_bytes(fixed_header: FixedHeader, stream: &mut dyn Read) -> MqttResult<Self> {
        // Fixed Header
        let fixed_header_flags = fixed_header.first_byte() & 0b0000_1111;

        if fixed_header_flags != RESERVED_FIXED_HEADER_FLAGS {
            return Err(MqttError::InvalidFixedHeaderFlags);
        }

        let remaining_length = fixed_header.remaining_length().value();

        // Variable Header
        let mut variable_header_buffer = [0; DEFAULT_VARIABLE_HEADER_LENGTH];
        stream.read_exact(&mut variable_header_buffer)?;

        let packet_identifier =
            u16::from_be_bytes([variable_header_buffer[0], variable_header_buffer[1]]);

        let mut return_codes = vec![];

        // Payload
        let mut payload_buffer = vec![0; remaining_length - DEFAULT_VARIABLE_HEADER_LENGTH];
        stream.read_exact(&mut payload_buffer)?;

        for &return_code_byte in payload_buffer.iter() {
            let return_code = SubackReturnCode::from_byte(return_code_byte)?;
            return_codes.push(return_code);
        }

        Ok(Suback::new(packet_identifier, return_codes))
    }

    /// Converts the Suback into a vector of bytes.
    pub fn to_bytes(&self, key: &[u8]) -> Vec<u8> {
        // Variable Header
        let mut variable_header_bytes = self.packet_identifier.to_be_bytes().to_vec();

        // Payload
        for return_code in &self.suback_return_codes {
            variable_header_bytes.push(return_code.to_byte());
        }

        // Fixed Header
        let mut fixed_header_bytes = vec![SUBACK_PACKET_TYPE << 4 | RESERVED_FIXED_HEADER_FLAGS];

        let remaining_length_value = variable_header_bytes.len() as u32;
        let remaining_length_bytes = RemainingLength::new(remaining_length_value).to_bytes();
        fixed_header_bytes.extend(remaining_length_bytes);

        let encrypted_bytes = match encrypt(variable_header_bytes, key) {
            Ok(bytes) => bytes,
            Err(_) => return vec![],
        };

        let mut packet_bytes = vec![];

        packet_bytes.extend(fixed_header_bytes);
        packet_bytes.extend(encrypted_bytes);

        packet_bytes
    }

    /// Returns the packet identifier.
    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier
    }

    /// Returns the Suback return codes.
    pub fn suback_return_codes(&self) -> &Vec<SubackReturnCode> {
        &self.suback_return_codes
    }
}

#[cfg(test)]
mod tests {
    use crate::encryptation::encryping_tool::decrypt;

    use super::*;

    const KEY: &[u8; 32] = &[0; 32];

    #[test]
    fn test_suback_to_bytes() {
        let suback = Suback::new(
            42,
            vec![
                SubackReturnCode::SuccessMaximumQoS0,
                SubackReturnCode::SuccessMaximumQoS1,
                SubackReturnCode::SuccessMaximumQoS2,
            ],
        );

        let expected_bytes: Vec<u8> = vec![144_u8, 5_u8, 0_u8, 42_u8, 0x00_u8, 0x01_u8, 0x02_u8];

        let encrypted_bytes = suback.to_bytes(KEY);
        let fixed_header_bytes = &encrypted_bytes[0..2];
        let decrypted_bytes = decrypt(&encrypted_bytes[2..], KEY).unwrap();
        let suback_bytes = [fixed_header_bytes, &decrypted_bytes[..]].concat();

        assert_eq!(suback_bytes, expected_bytes);
    }

    #[test]
    fn test_suback_from_bytes() {
        let bytes: Vec<u8> = vec![0_u8, 42_u8, 0x00_u8, 0x01_u8, 0x02_u8];

        let mut stream = &bytes[..];

        let fixed_header = FixedHeader::new(0b1001_0000, RemainingLength::new(5));
        let suback = Suback::from_bytes(fixed_header, &mut stream).unwrap();

        let return_codes = suback.suback_return_codes();
        assert_eq!(suback.packet_identifier(), 42);
        assert_eq!(return_codes.len(), 3);
        assert_eq!(return_codes[0], SubackReturnCode::SuccessMaximumQoS0);
        assert_eq!(return_codes[1], SubackReturnCode::SuccessMaximumQoS1);
        assert_eq!(return_codes[2], SubackReturnCode::SuccessMaximumQoS2);
    }
}
