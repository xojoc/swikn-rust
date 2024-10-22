use swikn;
use swikn::tools;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlDivElement, HtmlElement, HtmlInputElement, HtmlTextAreaElement};

#[wasm_bindgen]
extern "C" {
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
    val.set_inner_html(tools::test_library());

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn call_tool(e: HtmlElement) {
    let Some(data_tool) = e.get_attribute("data-tool") else {
        return;
    };

    for mut tool in tools::get_all_tools() {
        if tool.slug() == data_tool {
            let parameters_nodes = e.query_selector_all("input[data-parameter]").unwrap();
            for pn in parameters_nodes.values() {
                let pn = pn.unwrap();
                let pn = pn.dyn_into::<HtmlInputElement>().unwrap();
                let pname = pn.get_attribute("data-parameter").unwrap();
                tool.get_mut_parameters()
                    .set_parse_string(&pname, &pn.value());
            }

            let error_elt = e
                .query_selector(&format!("#error-{}", tool.slug()))
                .unwrap()
                .unwrap();
            let error_elt = error_elt.dyn_into::<HtmlDivElement>().unwrap();

            let output_elt = e
                .query_selector(&format!("#output-{}", tool.slug()))
                .unwrap()
                .unwrap();
            let output_elt = output_elt.dyn_into::<HtmlTextAreaElement>().unwrap();

            let Some(input_elt) = e
                .query_selector(&format!("#input-{}", tool.slug()))
                .unwrap()
            else {
                return;
            };
            let input_elt: HtmlTextAreaElement = input_elt.dyn_into().unwrap();
            let v = input_elt.value();

            let t = tool.transform(&v);
            match t {
                Ok(t) => {
                    error_elt.set_inner_html("");
                    output_elt.set_value(&t)
                }
                Err(err) => error_elt.set_inner_html(&format!("<p>{}</p>", err)),
            }

            // e.set_value(&t);
            // e.set_value(&t);
            return;
        }
    }
}

#[wasm_bindgen]
pub fn get_tools() -> String {
    swikn::html::html_all_tools()

    // swikn::html::
}
