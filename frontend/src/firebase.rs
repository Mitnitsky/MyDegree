/// Firebase JS bridge — wasm-bindgen extern declarations for firebase_bridge.js

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Start auth widget in the given DOM element ID.
    #[wasm_bindgen(js_name = startAuthUI)]
    pub fn start_auth_ui(element_id: &str);

    /// Register a callback for auth state changes.
    /// Callback receives a JSON string `{uid, displayName, email}` or null.
    #[wasm_bindgen(js_name = onAuthChange)]
    pub fn on_auth_change(cb: &Closure<dyn FnMut(Option<String>)>);

    /// Sign out the current user.
    #[wasm_bindgen(js_name = signOutUser)]
    pub fn sign_out_user();

    /// Read user document from Firestore. Returns Promise<string|null>.
    #[wasm_bindgen(js_name = firestoreGet)]
    pub fn firestore_get(uid: &str) -> js_sys::Promise;

    /// Write user document to Firestore. `json_data` is a JSON string.
    #[wasm_bindgen(js_name = firestoreSet)]
    pub fn firestore_set(uid: &str, json_data: &str) -> js_sys::Promise;
}

/// Auth user info returned from JS bridge.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthUser {
    pub uid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}
