struct FreePage {
    next: *mut FreePage
}

const NULL: *mut FreePage = 0 as *mut FreePage;

static mut FREE_PAGES: *mut FreePage = NULL;

pub fn free_page(pointer: *mut u8) {
    for i in 0..1024 * 4 {
        unsafe {
            *pointer.offset(i) = b'a';
        }
    }
    let new_page = pointer as *mut FreePage;
    unsafe {
        (*new_page).next = FREE_PAGES;
        FREE_PAGES = new_page;
    }
}

pub fn pages_available() -> usize {
    let mut count = 0;
    unsafe {
        let mut next: *mut FreePage = FREE_PAGES;
        while (*next).next != NULL {
            count += 1;
        }
    }
    count
}