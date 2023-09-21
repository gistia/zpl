use zpl::{BarcodeType, LabelBuilder};

fn main() {
    let label = LabelBuilder::new()
        .add_text(10, 10, "Hello World")
        .add_barcode(10, 30, BarcodeType::Code128, "12345")
        .build();
    println!("^XA{}^XZ", label.to_zpl());
}
