use serde::Serialize;
use serde_json::Error;

/// Represents a generic structur that can be serialized to a JSON string.
pub trait Serializable
where
    Self::Data: Serialize,
{
    /// Type of data to be serialized
    type Data;

    /// Return data to be serialized
    fn data_to_be_serialized(&self) -> &Self::Data;

    /// Serialize data to JSON string
    fn serialize(&self) -> Result<String, Error> {
        serde_json::to_string(self.data_to_be_serialized())
    }
}
