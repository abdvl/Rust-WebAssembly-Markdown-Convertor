use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(md: String) -> String {
    let res: String = md;
    println!("markdown : {}", res);
    return res;
}