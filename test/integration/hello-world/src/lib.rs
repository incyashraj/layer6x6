#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn run() {
        bindings::layer36::phase1::host::print("Hello, Layer36!");
        bindings::layer36::phase1::host::exit(0);
    }
}

bindings::export!(Component with_types_in bindings);
