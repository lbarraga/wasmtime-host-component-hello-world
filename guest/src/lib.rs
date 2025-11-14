wit_bindgen::generate!({
    world: "app",
    path: "../wit",
});

use crate::my::capitalize::host_functions;

struct MyComponent;

impl Guest for MyComponent {

    fn run(input: String) -> String {
        let capitalized_string = host_functions::capitalize(&input);
        format!("Guest: I sent '{}'. Host returned '{}'.", input, capitalized_string)
    }

}

export!(MyComponent);
