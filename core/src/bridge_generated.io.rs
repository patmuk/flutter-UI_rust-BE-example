use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_process_event(port_: i64, event: *mut wire_Event) {
    wire_process_event_impl(port_, event)
}

#[no_mangle]
pub extern "C" fn wire_view(port_: i64) {
    wire_view_impl(port_)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_event_0() -> *mut wire_Event {
    support::new_leak_box_ptr(wire_Event::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Event> for *mut wire_Event {
    fn wire2api(self) -> Event {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Event>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Event> for wire_Event {
    fn wire2api(self) -> Event {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.AddTodo);
                Event::AddTodo(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.RemoveTodo);
                Event::RemoveTodo(ans.field0.wire2api())
            },
            2 => Event::CleanList,
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Event {
    tag: i32,
    kind: *mut EventKind,
}

#[repr(C)]
pub union EventKind {
    AddTodo: *mut wire_Event_AddTodo,
    RemoveTodo: *mut wire_Event_RemoveTodo,
    CleanList: *mut wire_Event_CleanList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Event_AddTodo {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Event_RemoveTodo {
    field0: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Event_CleanList {}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Event {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Event_AddTodo() -> *mut EventKind {
    support::new_leak_box_ptr(EventKind {
        AddTodo: support::new_leak_box_ptr(wire_Event_AddTodo {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Event_RemoveTodo() -> *mut EventKind {
    support::new_leak_box_ptr(EventKind {
        RemoveTodo: support::new_leak_box_ptr(wire_Event_RemoveTodo {
            field0: Default::default(),
        }),
    })
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
