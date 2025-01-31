use crate::data_json_utils::JsonGetter;
use anyhow::Result;
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
fn is_wasm() -> bool {
    true
}

#[cfg(not(target_arch = "wasm32"))]
fn is_wasm() -> bool {
    false
}

/// Get the appropriate one of `os-current-dir` and `wasm-current-dir` in data_json,
/// depending on whether the compilation target is in the WASM environment.
pub fn get_current_dir(data_json: &Value) -> Result<String> {
    let wasm_current_dir = String::get_value(data_json, ["wasm-current-dir"]);
    let os_current_dir = String::get_value(data_json, ["os-current-dir"]);

    if is_wasm() {
        wasm_current_dir
    } else {
        os_current_dir
    }
}

/// Get the appropriate one of `os-target` and `wasm-target` in data_json,
/// depending on whether the compilation target is in the WASM environment.
pub fn get_target(data_json: &Value) -> Result<String> {
    let wasm_target = String::get_value(data_json, ["wasm-target"]);
    let os_target = String::get_value(data_json, ["os-target"]);

    if is_wasm() {
        wasm_target
    } else {
        os_target
    }
}
