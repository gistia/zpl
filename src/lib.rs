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

        zpl
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    typ: ElementType,
    x: u32,
    y: u32,
}

impl Element {
    pub fn to_zpl(&self) -> String {
        self.typ.to_zpl(self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub enum ElementType {
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

impl ElementType {
    pub fn to_zpl(&self, x: u32, y: u32) -> String {
        match self {
            ElementType::Image(path) => {
                format!("^FO{},{}{}^FS\n", x, y, image_to_zpl(path))
            }
            ElementType::Text(text) => format!("^FO{},{}^FD{}^FS\n", x, y, text),
            ElementType::Barcode { typ, data } => format!(
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
            ElementType::Icon(_typ) => {
                format!("^FO{},{}^GRI,,Y,N^FS\n^FO{},{}^GRI,,Y,N^FS\n", x, y, x, y,)
            }
            ElementType::Shape {
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

pub struct LabelBuilder {
    elements: Vec<Element>,
}

impl Default for LabelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LabelBuilder {
    pub fn new() -> Self {
        LabelBuilder {
            elements: Vec::new(),
        }
    }

    pub fn add_image(mut self, x: u32, y: u32, path: &str) -> Self {
        self.elements.push(Element {
            typ: ElementType::Image(path.to_string()),
            x,
            y,
        });
        self
    }

    pub fn add_text(mut self, x: u32, y: u32, text: &str) -> Self {
        self.elements.push(Element {
            typ: ElementType::Text(text.to_string()),
            x,
            y,
        });
        self
    }

    pub fn add_barcode(mut self, x: u32, y: u32, barcode_type: BarcodeType, data: &str) -> Self {
        self.elements.push(Element {
            typ: ElementType::Barcode {
                typ: barcode_type,
                data: data.to_string(),
            },
            x,
            y,
        });
        self
    }

    pub fn add_icon(mut self, x: u32, y: u32, icon_type: IconType) -> Self {
        self.elements.push(Element {
            typ: ElementType::Icon(icon_type),
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
        self.elements.push(Element {
            typ: ElementType::Shape {
                typ: shape_type,
                width,
                height,
            },
            x,
            y,
        });
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
