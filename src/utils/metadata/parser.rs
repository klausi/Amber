use heraclitus_compiler::prelude::*;
use crate::utils::memory::Memory;
use crate::utils::import_history::ImportHistory;

#[derive(Clone, Debug)]
pub struct ParserMetadata {
    pub expr: Vec<Token>,
    index: usize,
    pub path: Option<String>,
    pub code: Option<String>,
    pub binop_border: Option<usize>,
    pub mem: Memory,
    debug: Option<usize>,
    pub trace: Vec<PositionInfo>,
    pub import_history: ImportHistory,
    pub loop_ctx: bool,
    pub function_ctx: bool,
    pub messages: Vec<Message>
}

impl ParserMetadata {
    pub fn push_trace(&mut self, position: PositionInfo) {
        self.trace.push(position);
    }

    pub fn pop_trace(&mut self) -> Option<PositionInfo> {
        self.trace.pop()
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

impl Metadata for ParserMetadata {
    fn new(tokens: Vec<Token>, path: Option<String>, code: Option<String>) -> Self {
        ParserMetadata {
            expr: tokens,
            index: 0,
            path: path.clone(),
            code,
            binop_border: None,
            mem: Memory::new(),
            debug: None,
            trace: Vec::new(),
            import_history: ImportHistory::new(path),
            loop_ctx: false,
            function_ctx: false,
            messages: Vec::new(),
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index
    }

    fn get_token_at(&self, index: usize) -> Option<Token> {
        self.expr.get(index).cloned()
    }

    fn get_debug(&mut self) -> Option<usize> {
        self.debug
    }

    fn set_debug(&mut self, indent: usize) {
        self.debug = Some(indent)
    }

    fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }
}