use crate::{Cipher256, Cipher384, Cipher512};
use std::alloc::Layout;

#[no_mangle]
pub extern "C" fn new_cipher_256(key_raw: *const u8) -> *const u8 {
    let mut key = [0; 32];
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = unsafe { *key_raw.add(i) };
    }
    let cipher = Cipher256::new(key);

    let layout = Layout::new::<Cipher256>();

    unsafe {
        let addr = std::alloc::alloc(layout);
        std::ptr::write(addr as *mut Cipher256, cipher);
        addr
    }
}

#[no_mangle]
pub extern "C" fn nw_encrypt_256(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher256) };
    let mut plaintext = [0; 32];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.encrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn nw_decrypt_256(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher256) };
    let mut plaintext = [0; 32];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.decrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn new_cipher_384(key_raw: *const u8) -> *const u8 {
    let mut key = [0; 48];
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = unsafe { *key_raw.add(i) };
    }
    let cipher = Cipher384::new(key);

    let layout = Layout::new::<Cipher384>();

    unsafe {
        let addr = std::alloc::alloc(layout);
        std::ptr::write(addr as *mut Cipher384, cipher);
        addr
    }
}

#[no_mangle]
pub extern "C" fn nw_encrypt_384(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher384) };
    let mut plaintext = [0; 48];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.encrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn nw_decrypt_384(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher384) };
    let mut plaintext = [0; 48];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.decrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn new_cipher_512(key_raw: *const u8) -> *const u8 {
    let mut key = [0; 64];
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = unsafe { *key_raw.add(i) };
    }
    let cipher = Cipher512::new(key);

    let layout = Layout::new::<Cipher512>();

    unsafe {
        let addr = std::alloc::alloc(layout);
        std::ptr::write(addr as *mut Cipher512, cipher);
        addr
    }
}

#[no_mangle]
pub extern "C" fn nw_encrypt_512(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher512) };
    let mut plaintext = [0; 64];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.encrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn nw_decrypt_512(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher512) };
    let mut plaintext = [0; 64];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.decrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}
