mod bindings;

use bindings::Guest;

pub struct Component;

impl Guest for Component {
    fn transform(input: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String {
        input.to_uppercase()
    }
}

bindings::export!(Component with_types_in bindings);
