use std::ffi::{c_char, CString};

pub struct CStringArray {
    cstrings: Vec<CString>,
    cstrings_ptrs: Vec<*const c_char>,
}

impl CStringArray {
    pub fn from_vec<T: Into<Vec<u8>> + Clone>(strings: &Vec<T>) -> Self {
        let cstrings: Vec<CString> = strings.iter().cloned()
            .map(|s| CString::new(s).unwrap_or_default())
            .collect();

        let cstrings_ptrs: Vec<*const c_char> = (&cstrings).iter()
            .map(|cs| cs.as_ptr())
            .collect();

        Self {
            cstrings, cstrings_ptrs,
        }
    }

    pub fn as_ptr_vec(&self) -> &Vec<*const c_char> {
        &self.cstrings_ptrs
    }

    pub fn as_ptr_slice(&self) -> &[*const c_char] {
        self.cstrings_ptrs.as_slice()
    }

    pub fn as_cstring_vec(&self) -> &Vec<CString> {
        &self.cstrings
    }

    pub fn as_cstring_slice(&self) -> &[CString] {
        self.cstrings.as_slice()
    }
}