use std::{fs::File, io::Read};

use qrcode_generator::QrCodeEcc;

fn main() {
    code3()
}

fn code1() {
    qrcode_generator::to_svg_to_file("正直適正バチバチですよ。入部してみませんか？ https://oucrc.net/join/", QrCodeEcc::Low, 39, None::<&str>, "qrcode.svg").unwrap();
}

fn code3() {
    let mut file = File::open("qrcode.repath.svgpath.bz2").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    qrcode_generator::to_svg_to_file(ascii85::encode(&buf).as_bytes(), QrCodeEcc::Low, 101, None::<&str>, "qrcode.repath.svgpath.bz2.ascii85.svg").unwrap();
}
