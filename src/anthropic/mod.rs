pub mod error;
pub mod schema;
pub mod sse;

/// Anthropic Messages and Token Counting APIs reject bodies over 32 MB with 413 `request_too_large`.
/// https://platform.claude.com/docs/en/api/errors#request-size-limits
pub const MAX_ANTHROPIC_REQUEST_BYTES: usize = 32 * 1024 * 1024;

pub use self::error::{ErrorDetail, ErrorEnvelope, json_error};
pub use self::schema::{CountTokensResponse, Message, MessagesRequest};
pub use self::sse::{
    SseEvent, SseParseStats, encode_sse_event, parse_sse_events, parse_sse_events_with_stats,
};
