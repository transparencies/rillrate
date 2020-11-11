use anyhow::Error;
use derive_more::{From, FromStr, Into};
use meio_connect::{Protocol, ProtocolCodec, ProtocolData};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;

pub const PORT: u16 = 1636;

#[derive(Debug)]
pub struct RillProviderProtocol;

impl Protocol for RillProviderProtocol {
    type ToServer = RillToServer;
    type ToClient = RillToProvider;
    type Codec = JsonCodec;
}

pub struct JsonCodec;

impl ProtocolCodec for JsonCodec {
    fn decode<T: ProtocolData>(data: &[u8]) -> Result<T, Error> {
        serde_json::from_slice(data).map_err(Error::from)
    }

    fn encode<T: ProtocolData>(value: &T) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(value).map_err(Error::from)
    }
}

/// An identifier in a hierarchy of the node/metadata/stream.
#[derive(Serialize, Deserialize, FromStr, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(String);

impl AsRef<str> for EntryId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for EntryId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EntryId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for EntryId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Display for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, From, Into, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path(Vec<EntryId>);

impl Path {
    pub fn concat(&self, other: &[EntryId]) -> Path {
        self.0
            .iter()
            .chain(other.iter())
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }
}

impl AsRef<[EntryId]> for Path {
    fn as_ref(&self) -> &[EntryId] {
        &self.0
    }
}

/*
impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.join(".")
    }
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RillToServer {
    Declare { entry_id: EntryId },
    Entries { entries: Vec<EntryId> },
    Data { direct_id: DirectId, data: RillData },
}

pub type Timestamp = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RillData {
    LogRecord {
        timestamp: Timestamp,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RillToProvider {
    ListOf {
        path: Path,
    },
    // TODO: Use `Path` insead of `EntryId`.
    ControlStream {
        entry_id: EntryId,
        direct_id: DirectId,
        active: bool,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DirectId(pub u64);
