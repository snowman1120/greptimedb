///! Common actions for manifest
use serde::{Deserialize, Serialize};

pub type ProtocolVersion = u16;

/// Current reader and writer versions
/// TODO(dennis): configurable
const READER_VERSION: ProtocolVersion = 0;
const WRITER_VERSION: ProtocolVersion = 0;

/// The maximum protocol version we are currently allowed to use,
/// TODO(dennis): reading from configuration.
pub fn supported_protocol_version() -> (ProtocolVersion, ProtocolVersion) {
    (READER_VERSION, WRITER_VERSION)
}

/// Protocol action that used to block older clients from reading or writing the log when backwards
/// incompatible changes are made to the protocol. clients should be tolerant of messages and
/// fields that they do not understand.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolAction {
    pub min_reader_version: ProtocolVersion,
    pub min_writer_version: ProtocolVersion,
}

impl Default for ProtocolAction {
    fn default() -> Self {
        let (min_reader_version, min_writer_version) = supported_protocol_version();
        Self {
            min_reader_version,
            min_writer_version,
        }
    }
}

impl ProtocolAction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_readable(&self, reader_version: ProtocolVersion) -> bool {
        reader_version >= self.min_reader_version
    }

    pub fn is_writable(&self, writer_version: ProtocolVersion) -> bool {
        writer_version >= self.min_writer_version
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;

    use super::*;

    #[test]
    fn test_protocol_action() {
        let mut action = ProtocolAction::new();

        assert!(action.is_readable(0));
        assert!(action.is_writable(0));
        action.min_reader_version = 2;
        action.min_writer_version = 3;
        assert!(!action.is_readable(0));
        assert!(!action.is_writable(0));
        assert!(action.is_readable(2));
        assert!(action.is_writable(3));
        assert!(action.is_readable(3));
        assert!(action.is_writable(4));

        let s = json::to_string(&action).unwrap();
        assert_eq!("{\"min_reader_version\":2,\"min_writer_version\":3}", s);

        let action_decoded: ProtocolAction = json::from_str(&s).unwrap();
        assert!(!action_decoded.is_readable(0));
        assert!(!action_decoded.is_writable(0));
        assert!(action_decoded.is_readable(2));
        assert!(action_decoded.is_writable(3));
        assert!(action_decoded.is_readable(3));
        assert!(action_decoded.is_writable(4));
    }
}
