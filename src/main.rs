use std::{fs::File, io::Read};

use qrcode_generator::QrCodeEcc;

fn main() {
    code3(code2())
}

fn code1() {
    qrcode_generator::to_svg_to_file("正直適正バチバチですよ。入部してみませんか？ https://oucrc.net/join/", QrCodeEcc::Low, 39, None::<&str>, "qrcode.svg").unwrap();
}

fn code2() -> String {
    let mut file = File::open("qrcode.repath.min.svg.bz2").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    ascii85::encode(&buf)
}

fn code3(s: String) {
    qrcode_generator::to_svg_to_file(s.as_bytes(), QrCodeEcc::Low, 113, None::<&str>, "qrcode.repath.min.svg.vz2.ascii85.svg").unwrap();
}
