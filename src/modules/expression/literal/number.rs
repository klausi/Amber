use heraclitus_compiler::prelude::*;
use crate::{utils::metadata::{ParserMetadata, TranslateMetadata}, modules::types::{Type, Typed}};
use crate::translate::module::TranslateModule;

#[derive(Debug, Clone)]
pub struct Number {
    value: String
}

impl Typed for Number {
    fn get_type(&self) -> Type {
        Type::Num
    }
}

impl SyntaxModule<ParserMetadata> for Number {
    syntax_name!("Number");

    fn new() -> Self {
        Number {
            value: String::new()
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        if let Ok(sym) = token(meta, "-") {
            self.value.push_str(&sym);
        }
        if let Ok(value) = integer(meta, vec![]) {
            self.value.push_str(&value);
        }
        if let Ok(sym) = token(meta, ".") {
            self.value.push_str(&sym);
            self.value.push_str(&integer(meta, vec![])?);
        }
        if self.value.is_empty() {
            return Err(Failure::Quiet(PositionInfo::from_metadata(meta)))
        }
        Ok(())
    }
}

impl TranslateModule for Number {
    fn translate(&self, _meta: &mut TranslateMetadata) -> String {
        self.value.to_string()
    }
}
