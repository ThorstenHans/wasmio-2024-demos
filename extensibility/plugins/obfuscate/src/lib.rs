use bindings::Guest;

mod bindings;
struct Component;

impl Guest for Component {
    fn transform(input: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String {
        input
            .chars()
            .map(|x| match x {
                ' ' | 'A' | 'E' | 'U' | 'O' | 'I' | 'a' | 'e' | 'u' | 'o' | 'i' => '*',
                _ => x,
            })
            .collect()
    }
}
