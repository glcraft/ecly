#![allow(dead_code)]

use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Result<V, E> {
    #[serde(rename = "result")]
    Ok(V),
    #[serde(rename = "error")]
    Err(E)
}

#[derive(Serialize)]
pub struct Request<P: Serialize> {
    #[serde(rename = "jsonrpc")]
    version: String,
    id: i64,
    method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>
}
lazy_static!{
	static ref current_id: Mutex<i64> = Mutex::new(1);
}
fn gen_id() -> i64 {
	let mut lock = match current_id.lock() {
		Ok(v) => v,
		Err(e) => e.into_inner(),
	};
	let prev = *lock;
	*lock+=1;
	prev
}
impl <P: Serialize> Request<P> {
	pub fn new<S: ToString>(method: S, params: Option<P>) -> Request<P> {
		
		Request {
			version: "2.0".to_string(),
			id: gen_id(),
			method: method.to_string(),
			params,
		}
	}
	pub fn get_id(&self) -> i64 {
		self.id
	}
	
	pub fn get_method(&self) -> &str{
		self.method.as_str()
	}
	
	pub fn get_version(&self) -> &str {
		self.version.as_str()
	}
}

#[derive(Serialize)]
pub struct Notification<P: Serialize> 
{
    #[serde(rename = "jsonrpc")]
    version: String,
    method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>
}
impl <P: Serialize> Notification<P> {
	pub fn new<S: ToString>(method: S, params: Option<P>) -> Notification<P> {
		
		Notification {
			version: "2.0".to_string(),
			method: method.to_string(),
			params,
		}
	}
	pub fn get_version(&self) -> &str {
		self.version.as_str()
	}
	pub fn get_method(&self) -> &str{
		self.method.as_str()
	}
}

#[derive(Deserialize)]
pub struct Response<V, E> {
    #[serde(rename = "jsonrpc")]
    version: String,
    id: Option<i64>,
    #[serde(flatten)]
    result: Result<V,E>
}
#[derive(Deserialize)]
pub struct ResponseError<E> {
    code: i64,
    message: String,
    data: Option<E>
}

/// Defined by JSON RPC
pub mod error_codes {
    /// Invalid JSON was received by the server.
	/// An error occurred on the server while parsing the JSON text.
	pub const PARSE_ERROR: i64 = -32700;
	/// The JSON sent is not a valid Request object.
	pub const INVALID_REQUEST: i64 = -32600;
	/// The method does not exist / is not available
	pub const METHOD_NOT_FOUND: i64 = -32601;
	/// Invalid method parameter(s).
	pub const INVALID_PARAMS: i64 = -32602;
	/// Internal JSON-RPC error.
	pub const INTERNAL_ERROR: i64 = -32603;

	/**
	 This is the start range of JSON RPC reserved error codes.
	 It doesn't denote a real error code. No LSP error codes should
	 be defined between the start and end range. For backwards
	 compatibility the `ServerNotInitialized` and the `UnknownErrorCode`
	 are left in the range.
	 
	 *since 3.16.0*
	 */
	pub const JSONRPC_RESERVED_ERROR_RANGE_START: i64 = -32099;
	/// **deprecated**, use jsonrpcReservedErrorRangeStart
	#[deprecated = "use jsonrpcReservedErrorRangeStart"]
	pub const SERVER_ERROR_START: i64 = JSONRPC_RESERVED_ERROR_RANGE_START;

	/**
	 Error code indicating that a server received a notification or
	 request before the server has received the `initialize` request.
	 */
	pub const SERVER_NOT_INITIALIZED: i64 = -32002;
	pub const UNKNOWN_ERROR_CODE: i64 = -32001;

	/**
	 This is the end range of JSON RPC reserved error codes.
	 It doesn't denote a real error code.
	 
	 *since 3.16.0*
	 */
	pub const JSONRPC_RESERVED_ERROR_RANGE_END: i64 = -32000;
	
	/// **deprecated**, use jsonrpcReservedErrorRangeEnd
	#[deprecated = "use jsonrpcReservedErrorRangeEnd"]
	pub const SERVER_ERROR_END: i64 = JSONRPC_RESERVED_ERROR_RANGE_END;

	/**
	 This is the start range of LSP reserved error codes.
	 It doesn't denote a real error code.
	 
	 *since 3.16.0*
	 */
	pub const LSP_RESERVED_ERROR_RANGE_START: i64 = -32899;

	pub const CONTENT_MODIFIED: i64 = -32801;
	pub const REQUEST_CANCELLED: i64 = -32800;

	/**
	 This is the end range of LSP reserved error codes.
	 It doesn't denote a real error code.
	 
	 *since 3.16.0*
	 */
	pub const LSP_RESERVED_ERROR_RANGE_END: i64 = -32800;
}