
use askama::Template;

use crate::doc::symbol::{doc_symbol::DocSymbol, documentation::Documentation};

#[derive(Template)]
#[template(path = "single_html.html")]
pub struct DocsSingleHtmlTemplate{
    pub symbols: Vec<DocSymbol>
}

impl Documentation {
    pub fn export_to_single_html_str(&self) -> String {
        let template = DocsSingleHtmlTemplate{
            symbols: self.modules[0].root_symbols.clone()
        };

        template.render().unwrap()
    }
}
