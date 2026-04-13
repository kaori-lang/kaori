use super::value::Value;
use ahash::AHashMap;
use std::alloc::{Layout, alloc, dealloc};
use std::ptr;

#[repr(C)]
pub struct GcObject<T> {
    marked: bool,
    drop_and_dealloc: unsafe fn(*mut GcObject<()>),
    object: T,
}

#[repr(C)]
struct GcHeader {
    marked: bool,
    drop_and_dealloc: unsafe fn(*mut GcObject<()>),
}

impl<T> GcObject<T> {
    pub fn get(&self) -> &T {
        &self.object
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.object
    }

    unsafe fn drop_and_dealloc(ptr: *mut GcObject<()>) {
        unsafe {
            let typed = ptr as *mut GcObject<T>;
            ptr::drop_in_place(&mut (*typed).object);
            dealloc(ptr as *mut u8, Layout::new::<GcObject<T>>());
        }
    }
}

#[derive(Default)]
pub struct Gc {
    heap: Vec<*mut GcObject<()>>,
    strings_interned: AHashMap<String, *mut GcObject<String>>,
}

impl Gc {
    pub fn allocate_dict(&mut self) -> Value {
        Value::dict(self.alloc(AHashMap::default()))
    }

    pub fn allocate_vec(&mut self) -> Value {
        Value::vec(self.alloc(Vec::default()))
    }

    pub fn allocate_string(&mut self, s: &str) -> Value {
        if let Some(&ptr) = self.strings_interned.get(s) {
            return Value::string(ptr);
        }
        let ptr = self.alloc(s.to_owned());
        self.strings_interned.insert(s.to_owned(), ptr);
        Value::string(ptr)
    }

    fn alloc<T>(&mut self, object: T) -> *mut GcObject<T> {
        unsafe {
            let layout = Layout::new::<GcObject<T>>();
            let raw = alloc(layout) as *mut GcObject<T>;
            assert!(!raw.is_null(), "allocation failed");
            raw.write(GcObject {
                marked: false,
                drop_and_dealloc: GcObject::<T>::drop_and_dealloc,
                object,
            });
            self.heap.push(raw as *mut GcObject<()>);
            raw
        }
    }
}

impl Drop for Gc {
    fn drop(&mut self) {
        unsafe {
            for &ptr in &self.heap {
                let header = &*(ptr as *mut GcHeader);
                (header.drop_and_dealloc)(ptr);
            }
        }
    }
}
