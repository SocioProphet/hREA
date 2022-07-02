/**
 * Shared structs used by both semantic index "host" zome and client APIs
 * in order to communicate across the WASM API boundary.
 *
 * @package hdk_semantic_indexes
 * @since   2021-10-01
 */
use chrono::{DateTime, Utc};
use holochain_serialized_bytes::prelude::*;
pub use hdk_uuid_types::{DnaAddressable, EntryHash, HeaderHash};
pub use hdk_rpc_errors::{OtherCellResult, CrossCellError};

//--------------- API I/O STRUCTS ----------------

/// Query / modify entries by revision / `HeaderHash`
#[derive(Debug, Serialize, Deserialize)]
pub struct ByHeader {
    pub address: HeaderHash,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByRevision {
    pub revision_id: HeaderHash,
}
/// Shared parameter struct that all related record storage endpoints must implement
#[derive(Debug, Serialize, Deserialize)]
pub struct ByAddress<T> {
    pub address: T,
}

/// Shared parameter struct for indexing endpoints to respond to record creation
#[derive(Debug, Serialize, Deserialize)]
pub struct AppendAddress<T> {
    pub address: T,
    pub timestamp: DateTime<Utc>,
}

/// Common request format (zome trait) for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteEntryLinkRequest<A, B>
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    pub remote_entry: A,
    pub target_entries: Vec<B>,
    pub removed_entries: Vec<B>,
}

impl<A, B> TryFrom<&RemoteEntryLinkRequest<A, B>> for SerializedBytes
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(t: &RemoteEntryLinkRequest<A, B>) -> Result<SerializedBytes, SerializedBytesError> {
        encode(t).map(|v|
            SerializedBytes::from(UnsafeBytes::from(v))
        )
    }
}

impl<A, B> TryFrom<RemoteEntryLinkRequest<A, B>> for SerializedBytes
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(t: RemoteEntryLinkRequest<A, B>) -> Result<SerializedBytes, SerializedBytesError> {
        SerializedBytes::try_from(&t)
    }
}

// Factory / constructor method to assist with constructing responses

impl<A, B> RemoteEntryLinkRequest<A, B>
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    pub fn new(local_cell_entry: &A, add_remote_entries: &[B], remove_remote_entries: &[B]) -> Self {
        RemoteEntryLinkRequest {
            remote_entry: (*local_cell_entry).clone(),
            target_entries: add_remote_entries.to_vec(),
            removed_entries: remove_remote_entries.to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct RemoteEntryLinkResponse {
    pub indexes_created: Vec<OtherCellResult<HeaderHash>>,
    pub indexes_removed: Vec<OtherCellResult<HeaderHash>>,
}
