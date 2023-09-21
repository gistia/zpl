use zpl::{BarcodeType, FontName, LabelBuilder};

fn main() {
    let label = LabelBuilder::new(50, 25, None)
        .font_name(FontName::Zebra0)
        .font_size(20)
        .add_text(10, 10, "Last, First")
        .add_text(320, 10, "T1 - 20'")
        .add_text(10, 30, "12/12/1962")
        .add_barcode(120, 80, BarcodeType::Code128, "12345")
        .add_image(0, 140, "examples/assets/logo.jpg")
        .font_size(10)
        .add_text(300, 180, "2023-01-01 10:05pm")
        .build();
    println!("{}", label.to_zpl());
}
