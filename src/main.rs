use qrcode_generator::QrCodeEcc;

fn main() {
    qrcode_generator::to_svg_to_file("正直適正バチバチですよ。入部してみませんか？ https://oucrc.net/join/", QrCodeEcc::Low, 39, None::<&str>, "qrcode.svg").unwrap();
}
