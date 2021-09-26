use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize)]
pub struct FileOperation {
    /**
    * Whether the client supports dynamic registration for file
    * requests/notifications.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,

    /**
    * The client has support for sending didCreateFiles notifications.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didCreate: Option<bool>,

    /**
    * The client has support for sending willCreateFiles requests.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willCreate: Option<bool>,

    /**
    * The client has support for sending didRenameFiles notifications.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didRename: Option<bool>,

    /**
    * The client has support for sending willRenameFiles requests.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willRename: Option<bool>,

    /**
    * The client has support for sending didDeleteFiles notifications.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didDelete: Option<bool>,

    /**
    * The client has support for sending willDeleteFiles requests.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willDelete: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct Workspace {
    /**
    * The client supports applying batch edits
    * to the workspace by supporting the request
    * 'workspace/applyEdit'
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub applyEdit: Option<bool>,

    /**
    * Capabilities specific to `WorkspaceEdit`s
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub workspaceEdit: Option<WorkspaceEditClient>,

    /**
    * Capabilities specific to the `workspace/didChangeConfiguration`
    * notification.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub didChangeConfiguration: Option<DidChangeConfigurationClient>,

    /**
    * Capabilities specific to the `workspace/didChangeWatchedFiles`
    * notification.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub didChangeWatchedFiles: Option<DidChangeWatchedFilesClient>,

    /**
    * Capabilities specific to the `workspace/symbol` request.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub symbol: Option<WorkspaceSymbolClient>,

    /**
    * Capabilities specific to the `workspace/executeCommand` request.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub executeCommand: Option<ExecuteCommandClient>,

    /**
    * The client has support for workspace folders.
    *
    * @since 3.6.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub workspaceFolders: Option<bool>,

    /**
    * The client supports `workspace/configuration` requests.
    *
    * @since 3.6.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub configuration: Option<bool>,

    /**
    * Capabilities specific to the semantic token requests scoped to the
    * workspace.
    *
    * @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub semanticTokens: Option<SemanticTokensWorkspaceClient>,

    /**
    * Capabilities specific to the code lens requests scoped to the
    * workspace.
    *
    * @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub codeLens: Option<CodeLensWorkspaceClient>,
    /**
    * The client has support for file requests/notifications.
    *
    * @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
pub fileOperations: Option<FileOperation>,
}
#[derive(Debug, Serialize)]
#[serde(flatten)]
pub enum ResourceOperationKind {
    Create = "create",
    Rename = "rename",
    Delete = "delete",
}
#[derive(Debug, Serialize)]
#[serde(flatten)]
pub enum FailureHandlingKind {
    Abort = "abort",
    Transactional = "transactional",
    TextOnlyTransactional = "textOnlyTransactional",
    Undo = "undo",
}
#[derive(Debug, Serialize)]
pub struct ChangeAnnotationSupport {
    /**
    * Whether the client groups edits with equal labels into tree nodes,
    * for instance all edits labelled with "Changes in Strings" would
    * be a tree node.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupsOnLabel: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct WorkspaceEditClient {
    /**
	 * The client supports versioned document changes in `WorkspaceEdit`s
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub documentChanges: Option<bool>,

	/**
	 * The resource operations the client supports. Clients should at least
	 * support 'create', 'rename' and 'delete' files and folders.
	 *
	 * @since 3.13.0
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub resourceOperations: Option<Vec<ResourceOperationKind>>,

	/**
	 * The failure handling strategy of a client if applying the workspace edit
	 * fails.
	 *
	 * @since 3.13.0
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub failureHandling: Option<FailureHandlingKind>,

	/**
	 * Whether the client normalizes line endings to the client specific
	 * setting.
	 * If set to `true` the client will normalize line ending characters
	 * in a workspace edit to the client specific new line character(s).
	 *
	 * @since 3.16.0
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub normalizesLineEndings: Option<bool>,

	/**
	 * Whether the client in general supports change annotations on text edits,
	 * create file, rename file and delete file changes.
	 *
	 * @since 3.16.0
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub changeAnnotationSupport: Option<ChangeAnnotationSupport>,
}
#[derive(Debug, Serialize)]
pub struct DidChangeConfigurationClient {
    #[serde(skip_serializing_if = "Option::is_none")]
pub dynamicRegistration: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct DidChangeWatchedFilesClient {
    #[serde(skip_serializing_if = "Option::is_none")]
pub dynamicRegistration: Option<bool>,
}
#[derive(Debug, Serialize)]
#[serde(flatten)]
pub enum SymbolKind {
    File = 1,
	Module = 2,
	Namespace = 3,
	Package = 4,
	Class = 5,
	Method = 6,
	Property = 7,
	Field = 8,
	Constructor = 9,
	Enum = 10,
	Interface = 11,
	Function = 12,
	Variable = 13,
	Constant = 14,
	String = 15,
	Number = 16,
	bool = 17,
	Array = 18,
	Object = 19,
	Key = 20,
	Null = 21,
	EnumMember = 22,
	Struct = 23,
	Event = 24,
	Operator = 25,
	TypeParameter = 26,
}
#[derive(Debug, Serialize)]
pub struct SymbolKindSupport {
    /**
    * The symbol kind values the client supports. When this
    * property exists the client also guarantees that it will
    * handle values outside its set gracefully and falls back
    * to a default value when unknown.
    *
    * If this property is not present the client only supports
    * the symbol kinds from `File` to `Array` as defined in
    * the initial version of the protocol.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    valueSet: Option<Vec<SymbolKind>>
} 
#[derive(Debug, Serialize)]
pub struct WorkspaceSymbolClient {
/**
	 * Symbol request supports dynamic registration.
	 */
     #[serde(skip_serializing_if = "Option::is_none")]
pub dynamicRegistration: Option<bool>,

     /**
      * Specific capabilities for the `SymbolKind` in the `workspace/symbol`
      * request.
      */
     #[serde(skip_serializing_if = "Option::is_none")]
pub symbolKind: Option<SymbolKindSupport>,
 
     /**
      * The client supports tags on `SymbolInformation`.
      * Clients supporting tags have to handle unknown tags gracefully.
      *
      * @since 3.16.0
      */
    //  TODO: tagSupport?: TagSupport
}
#[derive(Debug, Serialize)]
pub struct ExecuteCommandClient {
    /**
	 * Execute command supports dynamic registration.
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub dynamicRegistration: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct SemanticTokensWorkspaceClient {
    /**
	 * Whether the client implementation supports a refresh request sent from
	 * the server to the client.
	 *
	 * Note that this event is global and will force the client to refresh all
	 * semantic tokens currently shown. It should be used with absolute care
	 * and is useful for situation where a server for example detect a project
	 * wide change that requires such a calculation.
	 */
     #[serde(skip_serializing_if = "Option::is_none")]
pub refreshSupport: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct CodeLensWorkspaceClient {
	/**
	 * Whether the client implementation supports a refresh request sent from the
	 * server to the client.
	 *
	 * Note that this event is global and will force the client to refresh all
	 * code lenses currently shown. It should be used with absolute care and is
	 * useful for situation where a server for example detect a project wide
	 * change that requires such a calculation.
	 */
     #[serde(skip_serializing_if = "Option::is_none")]
pub refreshSupport: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Window {
    /**
    * Whether client supports handling progress notifications. If set
    * servers are allowed to report in `workDoneProgress` property in the
    * request specific server capabilities.
    *
    * @since 3.15.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workDoneProgress: Option<bool>,
    
    /**
    * Capabilities specific to the showMessage request
    *
    * @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showMessage: Option<ShowMessageRequestClientCapabilities>,
    
    /**
    * Client capabilities for the show document request.
    *
    * @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showDocument: Option<ShowDocumentClientCapabilities>,
}
pub struct StaleRequestSupport {
    /**
    * The client will actively cancel the request.
    */
    pub cancel: bool,

    /**
    * The list of requests for which the client
    * will retry the request if it receives a
    * response with error code `ContentModified``
    */
    pub retryOnContentModified: Vec<String>
}
struct General {
		/**
		 * Client capability that signals how the client
		 * handles stale requests (e.g. a request
		 * for which the client will not process the response
		 * anymore since the information is outdated).
		 *
		 * @since 3.17.0
		 */
		#[serde(skip_serializing_if = "Option::is_none")]
pub staleRequestSupport: Option<StaleRequestSupport>,

		/**
		 * Client capabilities specific to regular expressions.
		 *
		 * @since 3.16.0
		 */
		#[serde(skip_serializing_if = "Option::is_none")]
pub regularExpressions: Option<RegularExpressionsClientCapabilities>,

		/**
		 * Client capabilities specific to the client's markdown parser.
		 *
		 * @since 3.16.0
		 */
		#[serde(skip_serializing_if = "Option::is_none")]
pub markdown: Option<MarkdownClientCapabilities>,
}
#[derive(Debug, Serialize)]
pub struct Client {
	/**
	 * Workspace specific client capabilities.
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub workspace: Option<Workspace>,

	/**
	 * Text document specific client capabilities.
	 */
	#[serde(skip_serializing_if = "Option::is_none")]
pub textDocument: Option<TextDocumentClientCapabilities>,

	/**
	 * Window specific client capabilities.
	 */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<Window>,

	/**
	 * General client capabilities.
	 *
	 * @since 3.16.0
	 */
     #[serde(skip_serializing_if = "Option::is_none")]
	pub general: Option<General>, 

	/**
	 * Experimental client capabilities.
	 */
// 	#[serde(skip_serializing_if = "Option::is_none")]
// pub experimental: Option<any>,
}