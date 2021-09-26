use serde::Serialize;
use super::capabilities;
type DocumentUri = String;
use strum::{self, AsRefStr};
type URI = String;

#[derive(Serialize)]
pub struct ClientInfo {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>
}

#[derive(Serialize)]
struct WorkspaceFolder {
    uri: DocumentUri,
    name: String
}
#[derive(Debug, Serialize, AsRefStr)]
#[strum(serialize_all = "camel_case")]
enum TraceValue {
    Off,
    Messages,
    Verbose,
}
#[derive(Serialize)]
pub struct InitializeParams {
    processId: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clientInfo: Option<ClientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    capabilities: capabilities::Client,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<TraceValue>,
    workspaceFolder: Option<Vec<WorkspaceFolder>>
}