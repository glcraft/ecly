use crate::json_rpc;
use serde::{Serialize, Deserialize};

type DocumentUri = String;
type URI = String;

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>
}

#[derive(Serialize, Deserialize)]
struct WorkspaceFolder {
    uri: DocumentUri,
    name: String
}
#[derive(Serialize, Deserialize)]
pub struct InitializeParams {
    processId: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clientInfo: Option<ClientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    workspaceFolder: Option<Vec<>>
}

pub struct Initialize(json_rpc::)