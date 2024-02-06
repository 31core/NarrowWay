use crate::{Cipher128, Cipher192, Cipher256};
use std::alloc::Layout;

#[no_mangle]
pub extern "C" fn new_cipher_128(key_raw: *const u8) -> *const u8 {
    let mut key = [0; 16];
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = unsafe { *key_raw.add(i) };
    }
    let cipher = Cipher128::new(key);

    let layout = Layout::new::<Cipher128>();

    unsafe {
        let addr = std::alloc::alloc(layout);
        std::ptr::write(addr as *mut Cipher128, cipher);
        addr
    }
}

#[no_mangle]
pub extern "C" fn encrypt_128(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher128) };
    let mut plaintext = [0; 16];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.encrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn decrypt_128(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher128) };
    let mut plaintext = [0; 16];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.decrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn new_cipher_192(key_raw: *const u8) -> *const u8 {
    let mut key = [0; 24];
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = unsafe { *key_raw.add(i) };
    }
    let cipher = Cipher192::new(key);

    let layout = Layout::new::<Cipher192>();

    unsafe {
        let addr = std::alloc::alloc(layout);
        std::ptr::write(addr as *mut Cipher192, cipher);
        addr
    }
}

#[no_mangle]
pub extern "C" fn encrypt_192(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher192) };
    let mut plaintext = [0; 24];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.encrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

#[no_mangle]
pub extern "C" fn decrypt_192(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
    let cipher = unsafe { &*(cipher as *const Cipher192) };
    let mut plaintext = [0; 24];
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte = unsafe { *plaintext_raw.add(i) };
    }
    let cipher_text = cipher.decrypt(plaintext);
    for (i, byte) in cipher_text.iter().enumerate() {
        unsafe { *buf.add(i) = *byte };
    }
}

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
pub extern "C" fn encrypt_256(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
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
pub extern "C" fn decrypt_256(cipher: *const u8, plaintext_raw: *const u8, buf: *mut u8) {
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
