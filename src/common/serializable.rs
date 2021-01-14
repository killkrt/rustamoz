use serde::Serialize;

/// Represents a generic structur that can be serialized to a JSON string.
pub trait Serializable
where
    Self::Data: Serialize,
{
    /// Type of data to be serialized
    type Data;

    /// Return data to be serialized
    fn data_to_be_serialized(&self) -> &Self::Data;
}
