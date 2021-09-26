use serde::Serialize;
use strum::{self, AsRefStr};
// use strum_macros;
#[derive(Debug, Serialize, Default)]
pub struct FileOperation {
    /**
    Whether the client supports dynamic registration for file
    requests/notifications.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,

    /// The client has support for sending didCreateFiles notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didCreate: Option<bool>,

    /// The client has support for sending willCreateFiles requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willCreate: Option<bool>,

    /// The client has support for sending didRenameFiles notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didRename: Option<bool>,

    /// The client has support for sending willRenameFiles requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willRename: Option<bool>,

    /// The client has support for sending didDeleteFiles notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didDelete: Option<bool>,

    /// The client has support for sending willDeleteFiles requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willDelete: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct Workspace {
    /**
    The client supports applying batch edits
    to the workspace by supporting the request
    'workspace/applyEdit'
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applyEdit: Option<bool>,

    /// Capabilities specific to `WorkspaceEdit`s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaceEdit: Option<WorkspaceEditClient>,

    /**
    Capabilities specific to the `workspace/didChangeConfiguration`
    notification.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didChangeConfiguration: Option<DidChangeConfigurationClient>,

    /**
    Capabilities specific to the `workspace/didChangeWatchedFiles`
    notification.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didChangeWatchedFiles: Option<DidChangeWatchedFilesClient>,

    /// Capabilities specific to the `workspace/symbol` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<WorkspaceSymbolClient>,

    /// Capabilities specific to the `workspace/executeCommand` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executeCommand: Option<ExecuteCommandClient>,

    /**
    The client has support for workspace folders.

    *since lsp v3.6.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaceFolders: Option<bool>,

    /**
    The client supports `workspace/configuration` requests.

    *since lsp v3.6.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<bool>,

    /**
    Capabilities specific to the semantic token requests scoped to the
    workspace.

    *since lsp v3.16.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semanticTokens: Option<SemanticTokensWorkspaceClient>,

    /**
    Capabilities specific to the code lens requests scoped to the
    workspace.

    *since lsp v3.16.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codeLens: Option<CodeLensWorkspaceClient>,
    /**
    The client has support for file requests/notifications.

    *since lsp v3.16.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fileOperations: Option<FileOperation>,
}
#[derive(Debug, Serialize, AsRefStr)]
#[strum(serialize_all = "camel_case")]
pub enum ResourceOperationKind {
    Create,
    Rename,
    Delete,
}
#[derive(Debug, Serialize, AsRefStr)]
#[strum(serialize_all = "camel_case")]
pub enum FailureHandlingKind {
    Abort,
    Transactional,
    TextOnlyTransactional,
    Undo,
}
#[derive(Debug, Serialize, Default)]
pub struct ChangeAnnotationSupport {
    /**
    Whether the client groups edits with equal labels into tree nodes,
    for instance all edits labelled with "Changes in Strings" would
    be a tree node.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupsOnLabel: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct WorkspaceEditClient {
    /// The client supports versioned document changes in `WorkspaceEdit`s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentChanges: Option<bool>,

    /**
    The resource operations the client supports. Clients should at least
    support 'create', 'rename' and 'delete' files and folders.

    *since lsp v3.13.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resourceOperations: Option<Vec<ResourceOperationKind>>,

    /**
    The failure handling strategy of a client if applying the workspace edit
    fails.

    *since lsp v3.13.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failureHandling: Option<FailureHandlingKind>,

    /**
    Whether the client normalizes line endings to the client specific
    setting.
    If set to `true` the client will normalize line ending characters
    in a workspace edit to the client specific new line character(s).

    *since lsp v3.16.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalizesLineEndings: Option<bool>,

    /**
    Whether the client in general supports change annotations on text edits,
    create file, rename file and delete file changes.

    *since lsp v3.16.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeAnnotationSupport: Option<ChangeAnnotationSupport>,
}
#[derive(Debug, Serialize, Default)]
pub struct DidChangeConfigurationClient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct DidChangeWatchedFilesClient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,
}
#[derive(Debug, Serialize)]
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
    Boolean = 17,
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
#[derive(Debug, Serialize, Default)]
pub struct SymbolKindSupport {
    /**
    The symbol kind values the client supports. When this
    property exists the client also guarantees that it will
    handle values outside its set gracefully and falls back
    to a default value when unknown.

    If this property is not present the client only supports
    the symbol kinds from `File` to `Array` as defined in
    the initial version of the protocol.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    valueSet: Option<Vec<SymbolKind>>,
}
#[derive(Debug, Serialize, Default)]
pub struct WorkspaceSymbolClient {
    /// Symbol request supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,

    /**
    Specific capabilities for the `SymbolKind` in the `workspace/symbol`
    request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolKind: Option<SymbolKindSupport>,
    /* *
    The client supports tags on `SymbolInformation`.
    Clients supporting tags have to handle unknown tags gracefully.

    *since lsp v3.16.0*
     */
    //  TODO: tagSupport?: TagSupport
}
#[derive(Debug, Serialize, Default)]
pub struct ExecuteCommandClient {
    /// Execute command supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct SemanticTokensWorkspaceClient {
    /**
    Whether the client implementation supports a refresh request sent from
    the server to the client.

    Note that this event is global and will force the client to refresh all
    semantic tokens currently shown. It should be used with absolute care
    and is useful for situation where a server for example detect a project
    wide change that requires such a calculation.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshSupport: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct CodeLensWorkspaceClient {
    /**
    Whether the client implementation supports a refresh request sent from the
    server to the client.

    Note that this event is global and will force the client to refresh all
    code lenses currently shown. It should be used with absolute care and is
    useful for situation where a server for example detect a project wide
    change that requires such a calculation.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshSupport: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct MessageActionItem {
    /**
    Whether the client supports additional attributes which
    are preserved and sent back to the server in the
    request's response.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additionalPropertiesSupport: Option<bool>
}
#[derive(Debug, Serialize, Default)]
pub struct ShowMessageRequestClient
{
    /// Capabilities specific to the `MessageActionItem` type.
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messageActionItem: Option<MessageActionItem>
}
#[derive(Debug, Serialize, Default)]
pub struct ShowDocumentClient
{
    /**
	The client has support for the show document
    request.
	*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<bool>
}
#[derive(Debug, Serialize, Default)]
pub struct Window {
    /**
    Whether client supports handling progress notifications. If set
    servers are allowed to report in `workDoneProgress` property in the
    request specific server capabilities.

    *since lsp v3.15.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workDoneProgress: Option<bool>,

    /**
    Capabilities specific to the showMessage request

    *since lsp v3.16.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showMessage: Option<ShowMessageRequestClient>,

    /**
    Client capabilities for the show document request.

    *since lsp v3.16.0*
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showDocument: Option<ShowDocumentClient>,
}
#[derive(Debug, Serialize, Default)]
pub struct StaleRequestSupport {
    /// The client will actively cancel the request.
    pub cancel: bool,

    /**
    The list of requests for which the client
    will retry the request if it receives a
    response with error code `ContentModified`
    */
    pub retryOnContentModified: Vec<String>,
}
#[derive(Debug, Serialize, Default)]
pub struct RegularExpressionsClient {
    /// The engine's name.
    pub engine: String,

	/// The engine's version.
    #[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,
}
#[derive(Debug, Serialize, Default)]
pub struct MarkdownClient {
    /// The name of the parser.
	parser: String,

	/// The version of the parser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Serialize, Default)]
pub struct CompletionItemResolveSupport {
    properties: Vec<String>,
}
#[derive(Debug, Serialize)]
pub enum CompletionItemTag {
    /// Render a completion as obsolete, usually using a strike-out.
	Deprecated = 1,
}
#[derive(Debug, Serialize, Default)]
pub struct CompletionItemTagSupport {
    pub valueSet: Vec<CompletionItemTag>,
}

#[derive(Debug, Serialize)]
pub enum InsertTextMode {
    /**
    The insertion or replace strings is taken as it is. If the
    value is multi line the lines below the cursor will be
    inserted using the indentation defined in the string value.
    The client will not apply any kind of adjustments to the
    string.
    */
	AsIs = 1,

	/**
    The editor adjusts leading whitespace of new lines so that
    they match the indentation up to the cursor of the line for
    which the item is accepted.
    
    Consider a line like this: <2tabs><cursor><3tabs>foo. Accepting a
    multi line completion item is indented using 2 tabs and all
    following lines inserted will be indented using 2 tabs as well.
    */
	AdjustIndentation = 2,
}
#[derive(Debug, Serialize, Default)]
pub struct CompletionItemInsertTextModeSupport {
    pub valueSet: Vec<InsertTextMode>,
}

#[derive(Debug, Serialize, AsRefStr)]
#[strum(serialize_all = "camel_case")]
pub enum MarkupKind {
    PlainText,
    Markdown
}
#[derive(Debug, Serialize, Default)]
pub struct CompletionItem {
    /**
    Client supports snippets as insert text.
    *
    A snippet can define tab stops and placeholders with `$1`, `$2`
    and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    the end of the snippet. Placeholders with equal identifiers are
    linked, that is typing in one will update others too.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippetSupport: Option<bool>,

    /// Client supports commit characters on a completion item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitCharactersSupport: Option<bool>,

    /**
    Client supports the follow content formats for the documentation
    property. The order describes the preferred format of the client.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentationFormat: Option<Vec<MarkupKind>>,

    /// Client supports the deprecated property on a completion item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecatedSupport: Option<bool>,

    /// Client supports the preselect property on a completion item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preselectSupport: Option<bool>,

    /**
    Client supports the tag property on a completion item. Clients
    supporting tags have to handle unknown tags gracefully. Clients
    especially need to preserve unknown tags when sending a completion
    item back to the server in a resolve call.
    *
    @since 3.15.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagSupport: Option<CompletionItemTagSupport>,

    /**
    Client supports insert replace edit to control different behavior if
    a completion item is inserted in the text or should replace text.
    *
    @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertReplaceSupport: Option<bool>,

    /**
    Indicates which properties a client can resolve lazily on a
    completion item. Before version 3.16.0 only the predefined properties
    `documentation` and `detail` could be resolved lazily.
    *
    @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolveSupport: Option<CompletionItemResolveSupport>,

    /**
    The client supports the `insertTextMode` property on
    a completion item to override the whitespace handling mode
    as defined by the client (see `insertTextMode`).
    *
    @since 3.16.0
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertTextModeSupport: Option<CompletionItemInsertTextModeSupport>,
/**
    The client has support for completion item label
    details (see also `CompletionItemLabelDetails`).
    *
    @since 3.17.0 - proposed state
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labelDetailsSupport: Option<bool>,
}

#[derive(Debug, Serialize)]
pub enum CompletionItemKind {
    Text = 1,
	Method = 2,
	Function = 3,
	Constructor = 4,
	Field = 5,
	Variable = 6,
	Class = 7,
	Interface = 8,
	Module = 9,
	Property = 10,
	Unit = 11,
	Value = 12,
	Enum = 13,
	Keyword = 14,
	Snippet = 15,
	Color = 16,
	File = 17,
	Reference = 18,
	Folder = 19,
	EnumMember = 20,
	Constant = 21,
	Struct = 22,
	Event = 23,
	Operator = 24,
	TypeParameter = 25,
}
#[derive(Debug, Serialize, Default)]
pub struct CompletionItemKindSet {
    /**
    The completion item kind values the client supports. When this
    property exists the client also guarantees that it will
    handle values outside its set gracefully and falls back
    to a default value when unknown.
    *
    If this property is not present the client only supports
    the completion items kinds from `Text` to `Reference` as defined in
    the initial version of the protocol.
    */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueSet: Option<Vec<CompletionItemKind>>,
}
#[derive(Debug, Serialize, Default)]
pub struct CompletionClientCapabilities {
    /// Whether completion supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,

	/**
    The client supports the following `CompletionItem` specific
    capabilities.
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub completionItem: Option<CompletionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completionItemKind: Option<CompletionItemKindSet>,

    /**
    The client supports to send additional context information for a
    `textDocument/completion` request.
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub contextSupport: Option<bool>,

	/**
    The client's default when the completion item doesn't provide a
    `insertTextMode` property.
    
    @since 3.17.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub insertTextMode: Option<InsertTextMode>,
}
#[derive(Debug, Serialize, Default)]
pub struct HoverClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct SignatureHelpClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DeclarationClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DefinitionClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct TypeDefinitionClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct ImplementationClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct ReferenceClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentHighlightClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentSymbolClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct CodeActionClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct CodeLensClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentLinkClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentColorClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentFormattingClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentRangeFormattingClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct DocumentOnTypeFormattingClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct RenameClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct PublishDiagnosticsClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct FoldingRangeClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct SelectionRangeClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct LinkedEditingRangeClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct CallHierarchyClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct SemanticTokensClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct MonikerClientCapabilities {

}
#[derive(Debug, Serialize, Default)]
pub struct TextDocumentSyncClientCapabilities {
    /// Whether text document synchronization supports dynamic registration.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub dynamicRegistration: Option<bool>,

    /// The client supports sending will save notifications.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub willSave: Option<bool>,

	/**
    The client supports sending a will save request and
    waits for a response providing text edits which will
    be applied to the document before it is saved.
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub willSaveWaitUntil: Option<bool>,

    /// The client supports did save notifications.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub didSave: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct TextDocumentClient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,

    /// Capabilities specific to the `textDocument/completion` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CompletionClientCapabilities>,

    /// Capabilities specific to the `textDocument/hover` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverClientCapabilities>,

    /// Capabilities specific to the `textDocument/signatureHelp` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub signatureHelp: Option<SignatureHelpClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/declaration` request.
    
	 @since 3.14.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub declaration: Option<DeclarationClientCapabilities>,

    /// Capabilities specific to the `textDocument/definition` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DefinitionClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/typeDefinition` request.
    
	 @since 3.6.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub typeDefinition: Option<TypeDefinitionClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/implementation` request.
    
	 @since 3.6.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub implementation: Option<ImplementationClientCapabilities>,

    /// Capabilities specific to the `textDocument/references` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferenceClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentHighlight` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub documentHighlight: Option<DocumentHighlightClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentSymbol` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub documentSymbol: Option<DocumentSymbolClientCapabilities>,

    /// Capabilities specific to the `textDocument/codeAction` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub codeAction: Option<CodeActionClientCapabilities>,

    /// Capabilities specific to the `textDocument/codeLens` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub codeLens: Option<CodeLensClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentLink` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub documentLink: Option<DocumentLinkClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/documentColor` and the
	 `textDocument/colorPresentation` request.
    
	 @since 3.6.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub colorProvider: Option<DocumentColorClientCapabilities>,

    /// Capabilities specific to the `textDocument/formatting` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<DocumentFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/rangeFormatting` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub rangeFormatting: Option<DocumentRangeFormattingClientCapabilities>,

	/** request.
	 Capabilities specific to the `textDocument/onTypeFormatting` request.
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub onTypeFormatting: Option<DocumentOnTypeFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/rename` request.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<RenameClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/publishDiagnostics`
	 notification.
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub publishDiagnostics: Option<PublishDiagnosticsClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/foldingRange` request.
    
	 @since 3.10.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub foldingRange: Option<FoldingRangeClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/selectionRange` request.
    
	 @since 3.15.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub selectionRange: Option<SelectionRangeClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/linkedEditingRange` request.
    
	 @since 3.16.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub linkedEditingRange: Option<LinkedEditingRangeClientCapabilities>,

	/**
	 Capabilities specific to the various call hierarchy requests.
    
	 @since 3.16.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub callHierarchy: Option<CallHierarchyClientCapabilities>,

	/**
	 Capabilities specific to the various semantic token requests.
    
	 @since 3.16.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub semanticTokens: Option<SemanticTokensClientCapabilities>,

	/**
	 Capabilities specific to the `textDocument/moniker` request.
    
	 @since 3.16.0
    */
	#[serde(skip_serializing_if = "Option::is_none")]
    pub moniker: Option<MonikerClientCapabilities>,
}
#[derive(Debug, Serialize, Default)]
pub struct General {
    /**
    Client capability that signals how the client
    handles stale requests (e.g. a request
    for which the client will not process the response
    anymore since the information is outdated).

    *since lsp v3.17.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staleRequestSupport: Option<StaleRequestSupport>,

    /**
    Client capabilities specific to regular expressions.

    *since lsp v3.16.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regularExpressions: Option<RegularExpressionsClient>,

    /**
    Client capabilities specific to the client's markdown parser.

    *since lsp v3.16.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownClient>,
}
#[derive(Debug, Serialize, Default)]
pub struct Client {
    /// Workspace specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,

    /// Text document specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textDocument: Option<TextDocumentClient>,

    /// Window specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<Window>,

    /**
    General client capabilities.

    *since lsp v3.16.0*
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<General>,
    /* *
    Experimental client capabilities.
     */
    // 	#[serde(skip_serializing_if = "Option::is_none")]
    // pub experimental: Option<any>,
}
