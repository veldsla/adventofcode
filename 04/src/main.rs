extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let input = "ckczppom".to_string();
 
    let take_n = 6;
    let mut sh = Md5::new();

    let mut i: u64 = 0;
    loop {
        sh.input_str((input.clone() + i.to_string().as_ref()).as_ref());
        let res = sh.result_str();
        if res.as_bytes().iter().take(take_n).all(|&c| c as char == '0') {
            println!("key={}, md5={}", i, res);
            break;
        }
        sh.reset();
        i+=1;
    }
}
