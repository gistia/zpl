use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Label {
    elements: Vec<Element>,
}

impl Label {
    pub fn to_zpl(&self) -> String {
        let mut zpl = String::new();

        // Add the elements
        for element in &self.elements {
            zpl.push_str(&element.to_zpl());
        }

        format!("^XA{}^XZ", zpl)
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Component { typ: ComponentType, x: u32, y: u32 },
    Setting(SettingType),
}

#[derive(Debug, Clone)]
pub enum SettingType {
    LabelDimensions(u32, u32, Option<u32>),
    FontSize(i32),
    FontName(FontName),
}

impl SettingType {
    pub fn to_zpl(&self) -> String {
        match self {
            SettingType::LabelDimensions(_, height, dpmm) => {
                let dpmm = match dpmm {
                    Some(dpmm) => dpmm.to_string(),
                    None => "8".to_string(),
                };
                format!("^XA^LL{height}\n^JM{dpmm}^LH0,0^BY,,40\n",)
            }
            SettingType::FontSize(size) => format!("^CF,{size},\n"),
            SettingType::FontName(font_name) => match font_name {
                FontName::Zebra0 => format!("^CF{font_name},,\n"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Image(String),
    Text(String),
    Barcode {
        typ: BarcodeType,
        data: String,
    },
    Icon(IconType),
    Shape {
        typ: ShapeType,
        width: u32,
        height: u32,
    },
}

impl Element {
    pub fn to_zpl(&self) -> String {
        match self {
            Element::Setting(typ) => typ.to_zpl(),
            Element::Component { typ, x, y } => match typ {
                ComponentType::Image(path) => {
                    format!("^FO{},{}{}^FS\n", x, y, image_to_zpl(path))
                }
                ComponentType::Text(text) => format!("^FO{},{}^FD{}^FS\n", x, y, text),
                ComponentType::Barcode { typ, data } => format!(
                    "^FO{},{}^BY2^B{}^FD{}^FS\n",
                    x,
                    y,
                    match typ {
                        BarcodeType::Code39 => "3",
                        BarcodeType::Code128 => "C",
                        BarcodeType::Aztec => "Z",
                        BarcodeType::DataMatrix => "D",
                        BarcodeType::EAN13 => "E",
                        BarcodeType::EAN8 => "E",
                        BarcodeType::GS1DataBar => "K",
                        BarcodeType::QRCode => "Q",
                    },
                    data
                ),
                ComponentType::Icon(_typ) => {
                    format!("^FO{},{}^GRI,,Y,N^FS\n^FO{},{}^GRI,,Y,N^FS\n", x, y, x, y,)
                }
                ComponentType::Shape {
                    typ,
                    width,
                    height: _height,
                } => format!(
                    "^FO{},{}^GB{},{}^FS\n",
                    x,
                    y,
                    match typ {
                        ShapeType::Ellipse => "E",
                        ShapeType::Rectangle => "R",
                        ShapeType::Triangle => "T",
                    },
                    width
                ),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum BarcodeType {
    Code39,
    Code128,
    Aztec,
    DataMatrix,
    EAN13,
    EAN8,
    GS1DataBar,
    QRCode,
}

#[derive(Debug, Clone)]
pub enum IconType {
    Arrow,
    Checkmark,
    Cross,
    Ellipse,
    Rectangle,
    Triangle,
}

#[derive(Debug, Clone)]
pub enum ShapeType {
    Ellipse,
    Rectangle,
    Triangle,
}

#[derive(Debug, Clone)]
pub enum FontName {
    Zebra0,
}

impl Display for FontName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FontName::Zebra0 => write!(f, "0"),
        }
    }
}

pub struct LabelBuilder {
    elements: Vec<Element>,
}

impl LabelBuilder {
    pub fn new(width_in_mm: u32, height_in_mm: u32, dpmm: Option<u32>) -> Self {
        LabelBuilder {
            elements: vec![Element::Setting(SettingType::LabelDimensions(
                width_in_mm,
                height_in_mm,
                dpmm,
            ))],
        }
    }

    pub fn add_image(mut self, x: u32, y: u32, path: &str) -> Self {
        self.elements.push(Element::Component {
            typ: ComponentType::Image(path.to_string()),
            x,
            y,
        });
        self
    }

    pub fn add_text(mut self, x: u32, y: u32, text: &str) -> Self {
        self.elements.push(Element::Component {
            typ: ComponentType::Text(text.to_string()),
            x,
            y,
        });
        self
    }

    pub fn add_barcode(mut self, x: u32, y: u32, barcode_type: BarcodeType, data: &str) -> Self {
        self.elements.push(Element::Component {
            typ: ComponentType::Barcode {
                typ: barcode_type,
                data: data.to_string(),
            },
            x,
            y,
        });
        self
    }

    pub fn add_icon(mut self, x: u32, y: u32, icon_type: IconType) -> Self {
        self.elements.push(Element::Component {
            typ: ComponentType::Icon(icon_type),
            x,
            y,
        });
        self
    }

    pub fn add_shape(
        mut self,
        x: u32,
        y: u32,
        shape_type: ShapeType,
        width: u32,
        height: u32,
    ) -> Self {
        self.elements.push(Element::Component {
            typ: ComponentType::Shape {
                typ: shape_type,
                width,
                height,
            },
            x,
            y,
        });
        self
    }

    pub fn font_size(mut self, size: i32) -> Self {
        self.elements
            .push(Element::Setting(SettingType::FontSize(size)));
        self
    }

    pub fn font_name(mut self, font_name: FontName) -> Self {
        self.elements
            .push(Element::Setting(SettingType::FontName(font_name)));
        self
    }

    pub fn build(self) -> Label {
        Label {
            elements: self.elements,
        }
    }
}

pub fn image_to_zpl(img_path: &str) -> String {
    // Load the image
    let img = image::open(img_path).unwrap();

    // Convert the image to grayscale
    let img_gray = img.to_luma8();

    // Calculate padding to make width a multiple of 8
    let padding = (8 - (img_gray.width() % 8)) % 8;

    // Convert the image to binary and then to hex
    let mut hex_string = String::new();
    for y in 0..img_gray.height() {
        let mut byte = 0u8;
        let mut bit_position = 0;

        for x in 0..img_gray.width() + padding {
            // Account for padding
            let pixel = if x < img_gray.width() {
                img_gray.get_pixel(x, y)[0]
            } else {
                255 // Pad with white
            };

            if pixel < 128 {
                byte |= 1 << (7 - bit_position);
            }

            bit_position += 1;
            if bit_position == 8 || x == img_gray.width() + padding - 1 {
                hex_string.push_str(&format!("{:02X}", byte));
                byte = 0;
                bit_position = 0;
            }
        }
    }

    // Build the ZPL II command
    let total_bytes = hex_string.len() / 2;
    let bytes_per_row = (img_gray.width() + padding) as usize / 8; // Account for padding
    format!(
        "^GFA,{}, {}, {},{}\n",
        total_bytes, total_bytes, bytes_per_row, hex_string
    )
}
