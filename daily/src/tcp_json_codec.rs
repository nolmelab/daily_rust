//! Length delimited json codec implementation 
//! - practice Codec implementation
//! - make it usable for chat server
//! - resources: 
//!   - https://docs.rs/tokio-jsoncodec/latest/src/tokio_jsoncodec/lib.rs.html#1-243 
//!   - A very naive implementation that uses serde_json
//!   - We need to delimit with length to be more efficient
//! 
//! - Use LengthDelimitedCodec 
//!   - The apply serde-json  
