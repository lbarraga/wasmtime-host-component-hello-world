use crate::my::capitalize::host_functions::Host;

#[derive(Default)]
pub struct MyHostFunctions;

impl Host for MyHostFunctions {
    fn capitalize(&mut self, input: String) -> String {
        println!("Host: Received call to 'capitalize' with: '{}'", input);
        input.to_uppercase()
    }
}
