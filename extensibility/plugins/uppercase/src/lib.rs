mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn transform(input: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String {
        input.to_uppercase()
    }
}
