mod json_rpc;
mod lsp;
#[cfg(test)]
mod tests {
    use crate::json_rpc;

    #[test]
    fn json_rpc() {
        let request = json_rpc::Request::<()>::new("method_name", None);
        assert_eq!(serde_json::to_string(&request).unwrap(), r#"{"jsonrpc":"2.0","id":1,"method":"method_name"}"#);
        let notif = json_rpc::Notification::<()>::new("initialize_new", None);
        assert_eq!(serde_json::to_string(&notif).unwrap(), r#"{"jsonrpc":"2.0","method":"initialize_new"}"#);
    }
}

