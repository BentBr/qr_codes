use image::{DynamicImage, Luma, Rgba};
use qrcode::QrCode;
use qrcode::render::svg;
use std::fs;

fn main() {
    let code = QrCode::new(b"https://www.sparepartsnow.de/?ref=ytvkyjj").unwrap();

    // Render QR code to 512x512
    let qr = code.render::<Luma<u8>>().min_dimensions(512, 512).build();
    let mut img = DynamicImage::ImageLuma8(qr).to_rgba8();

    // Draw white square in the middle
    let (w, h) = img.dimensions();
    let rect_w = w / 4;
    let rect_h = h / 4;
    let x0 = (w - rect_w) / 2;
    let y0 = (h - rect_h) / 2;

    for y in y0..y0 + rect_h {
        for x in x0..x0 + rect_w {
            img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    img.save("qrcode_with_square.png").unwrap();



    let svg_xml = code.render::<svg::Color>().build();
    // Save SVG to file.
    fs::write("qrcode.svg", svg_xml).unwrap();

    let code = QrCode::new(b"https://www.sparepartsnow.de/?ref=ytvkyjj").unwrap();

    // Render QR to SVG string
    let svg_xml = code.render::<svg::Color>().min_dimensions(512, 512).build();

    // White rectangle in the middle
    let white_rect = r#"<rect x="40%" y="40%" width="20%" height="20%" fill="white"/>"#;

    // Text in the middle
    let text_svg = r#"<text x="50%" y="50%"
                         dominant-baseline="middle"
                         text-anchor="middle"
                         font-size="48"
                         font-family="Arial"
                         fill="black">KIV</text>"#;

    // Inject shapes before closing </svg>
    let svg_with_logo_space =
        svg_xml.replace("</svg>", &format!("{}\n{}\n</svg>", white_rect, text_svg));

    fs::write("qrcode_with_space.svg", svg_with_logo_space).unwrap();
}
