mod utils;

use std::panic;

use rmonkey_evaluator::Evaluator;
use rmonkey_fmt::Formatter;
use rmonkey_lexer::Lexer;
use rmonkey_parser::Parser;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn code_to_ast(code: &str) -> String {
    set_panic_hook();
    let l = Lexer::new(code);
    let mut p = Parser::new(l);
    let program = p.parse_program().map_err(|e| format!("{e}"));
    match program {
        Ok(ast) => match serde_json::to_string(&ast) {
            Ok(ast_json) => ast_json,
            Err(err) => err.to_string(),
        },
        Err(err) => err,
    }
}

#[wasm_bindgen]
pub fn fmt(code: &str) -> Result<String, JsValue> {
    set_panic_hook();
    let mut formatter = Formatter::default();
    let l = Lexer::new(code);
    let mut p = Parser::new(l);
    let program = p.parse_program().map_err(|e| format!("{e}"));
    match program {
        Ok(program) => Ok(formatter.fmt(program)),
        Err(_) => Err(JsValue::from(code.to_string())),
    }
}

#[wasm_bindgen]
pub fn eval_rmonkey(code: &str) -> String {
    let mut e = Evaluator::new();
    let l = Lexer::new(code);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    match program {
        Ok(program) => match e.eval(program) {
            Ok(result) => format!("{result}"),
            Err(err) => format!("{err}"),
        },
        Err(err) => err.to_string(),
    }
}
