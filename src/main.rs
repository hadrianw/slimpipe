extern crate chardetng;
extern crate lol_html;

use std::fs::File;
use std::io::{Read, Write};
use std::cmp;
use chardetng::EncodingDetector;
use lol_html::{AsciiCompatibleEncoding, HtmlRewriter, Settings};

fn main() {
    let mut buf = [0u8; 8192];
    let mut file = File::open("simple.html").unwrap();
    let mut size = file.read(&mut buf).unwrap();

    let mut det = EncodingDetector::new();
    det.feed(&buf[..cmp::min(1024, size)], false);
    let enc = det.guess(Some(b"com"), false);
    let ascii_comp_enc = AsciiCompatibleEncoding::new(enc).unwrap();

    let mut rewriter = HtmlRewriter::new(
        Settings {
            encoding: ascii_comp_enc,
            ..Settings::default()
        },
        |buf: &[u8]| { std::io::stdout().write_all(buf).unwrap(); },
    );

    loop {
        if let Err(err) = rewriter.write(&buf[..size]) {
            println!("error: {:?}", err);
        }
        if size < buf.len() {
            break;
        }
        size = file.read(&mut buf).unwrap();
    }

    if let Err(err) = rewriter.end() {
        println!("error: {:?}", err);
    }
}
