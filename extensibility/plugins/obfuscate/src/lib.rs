use bindings::Guest;

mod bindings;
struct Component;

impl Guest for Component {
    fn transform(input: String) -> String {
        input
            .chars()
            .map(|x| match x {
                'A' | 'E' | 'U' | 'O' | 'I' | 'a' | 'e' | 'u' | 'o' | 'i' => '*',
                _ => x,
            })
            .collect()
    }
}

bindings::export!(Component with_types_in bindings);
