//! JSON-related client functionality
//!
//! This module handles JSON data sending and processing for the client.

use super::core::send_command;
use crate::ipc::protocol::Command;
use anyhow::{Context, Result};

/// Send JSON data to the server
///
/// This function sends JSON data via the binary protocol.
/// The protocol uses length-prefixed JSON for robust transmission.
///
/// # Arguments
/// * `json_data` - JSON string data to send
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::json;
/// let json = r#"{"events": []}"#;
/// json::send_json(json).unwrap();
/// ```
pub fn send_json(json_data: &str) -> Result<()> {
    // Parse the JSON to validate it
    let json_value: serde_json::Value =
        serde_json::from_str(json_data).context("Failed to parse JSON data")?;

    let command = Command::PlayJson { data: json_value };
    send_command(command)
}
