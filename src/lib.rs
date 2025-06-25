#[allow(warnings)]
mod bindings;

use bindings::exports::theater::simple::actor::Guest;
use bindings::theater::simple::runtime::log;

struct Component;

impl Guest for Component {
    fn init(state: Option<Vec<u8>>, _params: (String,)) -> Result<(Option<Vec<u8>>,), String> {
        log(&format!("Hello Actor initialized with state: {:?}", state));
        Ok((state,))
    }
}

bindings::export!(Component with_types_in bindings);
