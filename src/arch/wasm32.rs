use wasm_bindgen::prelude::*;


// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }


#[wasm_bindgen]
#[cfg(debug_assertions)]
pub fn enable_debugging() {
    /// This function helps with debugging by sending
    /// rust panic messages to the js console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
