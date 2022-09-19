mod aes;
use aes::{decrypt, encrypt};
fn main() {
    let data = "data to enc";
    let password = "password";
    let num_rounds = 4;
    println!("plain text: {}", data);
    let res = encrypt(data.as_bytes(), password.as_bytes(), num_rounds);
    println!("encrypted: {:02x?}", res);
    let d_res = decrypt(res.as_slice(), password.as_bytes(), num_rounds);
    let recovered = String::from_utf8(d_res).unwrap();
    println!("decrypted: {}", recovered);
}
