#[macro_use]
extern crate c2rust_bitfields;

mod mln_rsa;

use std::ptr;
use std::io::{self, Write};

fn main() {
    let s: &[u8] = b"Hello";
    let mut tmp: mln_rsa::mln_string_t = mln_rsa::mln_string_t { data: ptr::null_mut(), len: 0 };
    let mut cipher: *mut mln_rsa::mln_string_t = ptr::null_mut();
    let mut text: *mut mln_rsa::mln_string_t = ptr::null_mut();
    let mut pub_key: *mut mln_rsa::mln_rsa_key_t = ptr::null_mut();
    let mut pri_key: *mut mln_rsa::mln_rsa_key_t = ptr::null_mut();

    unsafe {
        pub_key = mln_rsa::mln_rsa_key_new();
        pri_key = mln_rsa::mln_rsa_key_new();

        if pub_key.is_null() || pri_key.is_null() {
            eprintln!("new pub/pri key failed");
            return;
        }

        if mln_rsa::mln_rsa_key_generate(pub_key, pri_key, 128) < 0 {
            eprintln!("key generate failed");
            return;
        }

        mln_rsa::mln_string_set(&mut tmp, s);

        cipher = mln_rsa::mln_RSAESPKCS1V15_public_encrypt(pub_key, &tmp);
        if cipher.is_null() {
            eprintln!("pub key encrypt failed");
            return;
        }

        // Print the encrypted data
        io::stdout().write_all(&(*cipher).data).unwrap();
        io::stdout().write_all(b"\n").unwrap();

        text = mln_rsa::mln_RSAESPKCS1V15_private_decrypt(pri_key, cipher);
        if text.is_null() {
            eprintln!("pri key decrypt failed");
            return;
        }

        // Print the decrypted text
        io::stdout().write_all(&(*text).data).unwrap();
        io::stdout().write_all(b"\n").unwrap();
    }

    // Free memory
    unsafe {
        if !pub_key.is_null() {
            mln_rsa::mln_rsa_key_free(pub_key);
        }
        if !pri_key.is_null() {
            mln_rsa::mln_rsa_key_free(pri_key);
        }
        if !cipher.is_null() {
            mln_rsa::mln_RSAESPKCS1V15_free(cipher);
        }
        if !text.is_null() {
            mln_rsa::mln_RSAESPKCS1V15_free(text);
        }
    }
}
