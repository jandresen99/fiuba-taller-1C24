use std::fmt::{self, Display, Formatter};

use crate::{MqttError, MqttResult};

const MULTI_LEVEL_WILDCARD: u8 = 0x23;
const SINGLE_LEVEL_WILDCARD: u8 = 0x2B;

/// Part of a TopicFilter, indicates if the topic contains a wildcard.
#[derive(Debug, PartialEq, Clone)]
pub enum TopicLevel {
    Literal(Vec<u8>),
    MultiLevelWildcard,
    SingleLevelWildcard,
}

impl TopicLevel {
    /// Converts a vector of bytes into a TopicLevel.
    pub fn from_bytes(bytes: Vec<u8>) -> MqttResult<TopicLevel> {
        if bytes.len() == 1 {
            return match bytes.first() {
                Some(&MULTI_LEVEL_WILDCARD) => Ok(TopicLevel::MultiLevelWildcard),
                Some(&SINGLE_LEVEL_WILDCARD) => Ok(TopicLevel::SingleLevelWildcard),
                _ => Ok(TopicLevel::Literal(bytes)),
            };
        }

        if bytes.contains(&MULTI_LEVEL_WILDCARD) {
            return Err(MqttError::InvalidWildcard(
                "Multi-level wildcard must be the only character".to_string(),
            ));
        }

        if bytes.contains(&SINGLE_LEVEL_WILDCARD) {
            return Err(MqttError::InvalidWildcard(
                "Single-level wildcard must be the only character".to_string(),
            ));
        }

        Ok(TopicLevel::Literal(bytes))
    }

    /// Converts a TopicLevel into a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TopicLevel::Literal(bytes) => bytes.to_vec(),
            TopicLevel::MultiLevelWildcard => vec![MULTI_LEVEL_WILDCARD],
            TopicLevel::SingleLevelWildcard => vec![SINGLE_LEVEL_WILDCARD],
        }
    }

    /// Returns the length of the TopicLevel.
    pub fn length(&self) -> usize {
        match self {
            TopicLevel::Literal(bytes) => bytes.len(),
            TopicLevel::MultiLevelWildcard => 1,
            TopicLevel::SingleLevelWildcard => 1,
        }
    }
}

impl Display for TopicLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TopicLevel::Literal(bytes) => {
                let string = String::from_utf8_lossy(bytes);
                write!(f, "{}", string)
            }
            TopicLevel::MultiLevelWildcard => write!(f, "{}", MULTI_LEVEL_WILDCARD as char),
            TopicLevel::SingleLevelWildcard => write!(f, "{}", SINGLE_LEVEL_WILDCARD as char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_literal() {
        let bytes = b"home".to_vec();
        let topic_level = TopicLevel::from_bytes(bytes.clone()).unwrap();

        assert_eq!(topic_level, TopicLevel::Literal(bytes));
    }

    #[test]
    fn test_valid_multi_level_wildcard() {
        let bytes = vec![MULTI_LEVEL_WILDCARD];
        let topic_level = TopicLevel::from_bytes(bytes.clone()).unwrap();

        assert_eq!(topic_level, TopicLevel::MultiLevelWildcard);
    }

    #[test]
    fn test_valid_single_level_wildcard() {
        let bytes = vec![SINGLE_LEVEL_WILDCARD];
        let topic_level = TopicLevel::from_bytes(bytes.clone()).unwrap();

        assert_eq!(topic_level, TopicLevel::SingleLevelWildcard);
    }

    #[test]
    fn test_invalid_use_of_wildcards() {
        {
            let bytes = b"home+".to_vec();
            let topic_level = TopicLevel::from_bytes(bytes.clone());

            assert!(topic_level.is_err());
        }

        {
            let bytes = b"+home".to_vec();
            let topic_level = TopicLevel::from_bytes(bytes.clone());

            assert!(topic_level.is_err());
        }

        {
            let bytes = b"home#".to_vec();
            let topic_level = TopicLevel::from_bytes(bytes.clone());

            assert!(topic_level.is_err());
        }

        {
            let bytes = b"#home".to_vec();
            let topic_level = TopicLevel::from_bytes(bytes.clone());

            assert!(topic_level.is_err());
        }

        {
            let bytes = b"#+".to_vec();
            let topic_level = TopicLevel::from_bytes(bytes.clone());

            assert!(topic_level.is_err());
        }
    }
}
