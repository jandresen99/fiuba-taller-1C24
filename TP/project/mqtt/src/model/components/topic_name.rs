use super::{FORWARD_SLASH, SERVER_RESERVED};
use crate::{EncodedString, MqttError, MqttResult, Read, TopicLevel};
use std::fmt;

/// Represents the name of a topic in MQTT.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct TopicName {
    levels: Vec<Vec<u8>>,
    server_reserved: bool,
}

impl TopicName {
    pub fn new(levels: Vec<Vec<u8>>, server_reserved: bool) -> Self {
        Self {
            levels,
            server_reserved,
        }
    }

    /// Converts a stream of bytes into a TopicName.
    pub fn from_bytes(stream: &mut dyn Read) -> MqttResult<Self> {
        let encoded_string_topic_name = EncodedString::from_bytes(stream)?;
        let bytes = encoded_string_topic_name.content();

        if bytes.is_empty() {
            return Err(MqttError::InvalidTopicName);
        }

        let server_reserved = matches!(bytes.first(), Some(&SERVER_RESERVED));

        let levels_bytes: Vec<Vec<u8>> = bytes
            .split(|&byte| byte == FORWARD_SLASH)
            .map(|slice: &[u8]| slice.to_vec())
            .collect();

        let mut levels = vec![];

        for level in levels_bytes {
            match TopicLevel::from_bytes(level)? {
                TopicLevel::Literal(level) => levels.push(level),
                _ => {
                    return Err(MqttError::InvalidWildcard(
                        "Wildcard not allowed in topic name".to_string(),
                    ))
                }
            }
        }

        Ok(Self {
            levels,
            server_reserved,
        })
    }

    /// Converts the TopicName into a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut topic_bytes = vec![];

        for (i, level) in self.levels.iter().enumerate() {
            topic_bytes.extend(level);

            if i < self.levels.len() - 1 {
                topic_bytes.push(FORWARD_SLASH);
            }
        }

        EncodedString::new(topic_bytes).to_bytes()
    }

    pub fn serialize(&self) -> String {
        let levels = self
            .levels
            .iter()
            .map(|level| String::from_utf8_lossy(level).into_owned())
            .collect::<Vec<String>>()
            .join("/");

        levels
    }

    pub fn deserialize(serialized: &str) -> Result<TopicName, String> {
        let levels = serialized
            .split('/')
            .map(|level| level.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        Ok(TopicName {
            levels,
            server_reserved: false, // Or handle the server_reserved field appropriately if needed
        })
    }

    /// Returns the levels of the topic.
    pub fn levels(&self) -> &Vec<Vec<u8>> {
        &self.levels
    }

    /// Returns the length of the topic.
    pub fn length(&self) -> usize {
        self.to_bytes().len()
    }

    /// Returns whether the topic is reserved by the server.
    pub fn server_reserved(&self) -> bool {
        self.server_reserved
    }
}

impl fmt::Display for TopicName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let levels = self
            .levels
            .iter()
            .map(|level| String::from_utf8_lossy(level).into_owned())
            .collect::<Vec<String>>()
            .join("/");

        write!(f, "{}", levels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[allow(dead_code)]
    fn from_slice(bytes: &[u8]) -> impl Read {
        let encoded_string = EncodedString::new(bytes.to_vec());
        Cursor::new(encoded_string.to_bytes())
    }

    #[test]
    fn test_valid_topic_names() {
        let bytes = &mut from_slice(b"home/livingroom");
        assert!(TopicName::from_bytes(bytes).is_ok());

        let bytes = &mut from_slice(b"/");
        assert!(TopicName::from_bytes(bytes).is_ok());
    }

    #[test]
    fn test_invalid_topic_names() {
        let bytes = &mut from_slice(b"home/+/livingroom");
        assert!(TopicName::from_bytes(bytes).is_err());

        let bytes = &mut from_slice(b"home/livingroom/#");
        assert!(TopicName::from_bytes(bytes).is_err());

        let bytes = &mut from_slice(b"home/livingroom#");
        assert!(TopicName::from_bytes(bytes).is_err());

        let bytes = &mut from_slice(b"+home/livingroom");
        assert!(TopicName::from_bytes(bytes).is_err());
    }

    #[test]
    fn test_length() {
        let bytes = &mut from_slice(b"home/livingroom");
        let topic_name = TopicName::from_bytes(bytes).unwrap();
        assert_eq!(topic_name.length(), 17);

        let bytes = &mut from_slice(b"/");
        let topic_name = TopicName::from_bytes(bytes).unwrap();
        assert_eq!(topic_name.length(), 3);
    }

    #[test]
    fn test_server_reserved() {
        let bytes = &mut from_slice(b"$home/livingroom");
        let topic_name = TopicName::from_bytes(bytes).unwrap();
        assert!(topic_name.server_reserved());

        let bytes = &mut from_slice(b"home/livingroom");
        let topic_name = TopicName::from_bytes(bytes).unwrap();
        assert!(!topic_name.server_reserved());

        let bytes = &mut from_slice(b"home/$livingroom");
        let topic_name = TopicName::from_bytes(bytes).unwrap();
        assert!(!topic_name.server_reserved());
    }
}
