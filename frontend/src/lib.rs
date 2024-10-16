use swikn;
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body: web_sys::HtmlElement = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html(swikn::test_library());

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn pretty_print_json(e: HtmlTextAreaElement) {
    use serde_json;
    log("called");
    log(&e.value());
    let v: serde_json::Value = serde_json::from_str(&e.value()).unwrap();
    let pp = serde_json::to_string_pretty(&v).unwrap();
    log(&pp);
    e.set_value(&pp)
}
