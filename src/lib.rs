use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(md: String) -> String {
    let mut res = md;
    println!("markdown : {}",res);
    return res;

}