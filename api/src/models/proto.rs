//! Sprint 158: Protobuf message structs for V2 sync protocol.
//! Hand-written prost::Message derives — no build.rs/protoc needed.
//! Wire-compatible with proto/sync.proto.

use std::collections::HashMap;
use serde_json::Value;

// ============================================================
// Push Request
// ============================================================

/// A single change record in a push request.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoChangeRecord {
    #[prost(string, tag = "1")]
    pub uuid: String,
    #[prost(string, tag = "2")]
    pub table_name: String,
    #[prost(string, tag = "3")]
    pub action: String,
    /// JSON-encoded row data as bytes
    #[prost(bytes = "vec", tag = "4")]
    pub data: Vec<u8>,
    #[prost(int64, tag = "5")]
    pub timestamp: i64,
}

/// Push request — sent by client.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoPushRequest {
    #[prost(string, tag = "1")]
    pub device_id: String,
    #[prost(string, tag = "2")]
    pub batch_id: String,
    #[prost(message, repeated, tag = "3")]
    pub changes: Vec<ProtoChangeRecord>,
    #[prost(int64, optional, tag = "4")]
    pub max_journal_id: Option<i64>,
}

/// Conflict info in push response.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoConflictInfo {
    #[prost(string, tag = "1")]
    pub table_name: String,
    #[prost(string, tag = "2")]
    pub record_uuid: String,
    #[prost(string, tag = "3")]
    pub resolution: String,
}

/// Push response — sent by server.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoPushResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub message: String,
    #[prost(int64, tag = "3")]
    pub new_cursor: i64,
    #[prost(int32, tag = "4")]
    pub processed: i32,
    #[prost(message, repeated, tag = "5")]
    pub conflicts: Vec<ProtoConflictInfo>,
    /// JSON-encoded computed_updates as bytes
    #[prost(bytes = "vec", tag = "6")]
    pub computed_updates: Vec<u8>,
    #[prost(int64, optional, tag = "7")]
    pub last_processed_client_tx_id: Option<i64>,
}

// ============================================================
// Pull Response
// ============================================================

/// A single change record in a pull response.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoPullChangeRecord {
    #[prost(string, tag = "1")]
    pub uuid: String,
    #[prost(string, tag = "2")]
    pub operation: String,
    /// JSON-encoded row data as bytes
    #[prost(bytes = "vec", tag = "3")]
    pub data: Vec<u8>,
}

/// Changes for a single table in pull response.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoPullTableChanges {
    #[prost(string, tag = "1")]
    pub table_name: String,
    #[prost(message, repeated, tag = "2")]
    pub records: Vec<ProtoPullChangeRecord>,
}

/// Pull response — sent by server.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoPullResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(int64, tag = "2")]
    pub cursor: i64,
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    #[prost(message, repeated, tag = "4")]
    pub changes: Vec<ProtoPullTableChanges>,
    /// JSON-encoded computed_updates as bytes
    #[prost(bytes = "vec", tag = "5")]
    pub computed_updates: Vec<u8>,
}

// ============================================================
// Snapshot Response
// ============================================================

/// Records for a single table in snapshot.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoSnapshotTable {
    #[prost(string, tag = "1")]
    pub table_name: String,
    /// Each entry is a JSON-encoded row as bytes
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub records: Vec<Vec<u8>>,
}

/// Snapshot response — sent by server.
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProtoSnapshotResponse {
    #[prost(message, repeated, tag = "1")]
    pub tables: Vec<ProtoSnapshotTable>,
    #[prost(int64, tag = "2")]
    pub watermark_cursor: i64,
}

// ============================================================
// Conversion helpers
// ============================================================

use crate::models::sync_v2::{ChangeRecord, ConflictInfo};

impl ProtoPushRequest {
    /// Convert protobuf push request into the HashMap format merge_engine expects.
    pub fn into_changes(self) -> (String, String, HashMap<String, Vec<ChangeRecord>>, Option<i64>) {
        let mut changes: HashMap<String, Vec<ChangeRecord>> = HashMap::new();

        for record in self.changes {
            let data: Value = serde_json::from_slice(&record.data).unwrap_or_default();
            let entry = changes.entry(record.table_name).or_default();
            entry.push(ChangeRecord {
                uuid: record.uuid,
                operation: record.action,
                data,
            });
        }

        (self.device_id, self.batch_id, changes, self.max_journal_id)
    }
}

impl From<&ConflictInfo> for ProtoConflictInfo {
    fn from(c: &ConflictInfo) -> Self {
        ProtoConflictInfo {
            table_name: c.table_name.clone(),
            record_uuid: c.record_uuid.clone(),
            resolution: c.resolution.clone(),
        }
    }
}
