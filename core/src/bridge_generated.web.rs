use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_process_event(port_: MessagePort, event: JsValue) {
    wire_process_event_impl(port_, event)
}

#[wasm_bindgen]
pub fn wire_view(port_: MessagePort) {
    wire_view_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}

impl Wire2Api<Event> for JsValue {
    fn wire2api(self) -> Event {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Event::AddTodo(self_.get(1).wire2api()),
            1 => Event::RemoveTodo(self_.get(1).wire2api()),
            2 => Event::CleanList,
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
