mod utils;

use rmonkey_evaluator::Evaluator;
use rmonkey_lexer::Lexer;
use rmonkey_parser::Parser;
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
pub fn output_log(s: &str) {
    log(&format!("Hello {s}"));
}

#[wasm_bindgen]
pub fn eval_rmonkey(code: &str) -> String {
    let mut e = Evaluator::new();
    let l = Lexer::new(code);
    let mut p = Parser::new(l);
    let program = p.parse_program().map_err(|e| format!("{e}")).unwrap();
    match e.eval(program) {
        Ok(result) => format!("{result}"),
        Err(err) => format!("{err}"),
    }
}
