mod aes;
use aes::{decrypt, encrypt};

use base64::{encode, decode};


fn help() {
    println!("Expected either enc or dec followed by a password and value");
    std::process::exit(0);
}

fn main() {
    let mut args = std::env::args();

    if args.len() != 4{
        help();
    }

    args.next().unwrap();

    let method = args.next().unwrap();
    let password = args.next().unwrap();
    let value = args.next().unwrap();

    let num_rounds = 10;


    match method.as_str() {
        "enc" => {
            let res = encrypt(value.as_bytes(), password.as_bytes(), num_rounds);
            println!("{}", encode(res));
        },
        "dec" => {
            let bytes = decode(&value).unwrap();
            let res = decrypt(bytes.as_slice(), password.as_bytes(), num_rounds);
            let recovered = String::from_utf8(res).unwrap();
            println!("{}", recovered);
        },

        _ => help()

    }
}
