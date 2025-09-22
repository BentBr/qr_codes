use clap::Parser;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use std::{fs, process};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    uri: String,

    #[arg(short, long)]
    letters: Option<String>,

    #[arg(short, long, default_value_t = 20)]
    square: u32,

    #[arg(short, long, default_value_t = 40)]
    font_size: u32,
}

fn sanitize_filename(uri: &str) -> String {
    uri.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.")
        .replace(['/', ':', '?', '&', '='], "_")
}

fn main() {
    let args = Args::parse();

    // Validation
    if args.square > 100 {
        eprintln!("Error: --square must not be greater than 100");
        process::exit(1);
    }

    // QR with error correction
    let code = QrCode::with_error_correction_level(args.uri.as_bytes(), EcLevel::H).unwrap();

    let final_svg = if let Some(letters) = args.letters {
        let svg_xml = code
            .render::<svg::Color>()
            .min_dimensions(512, 512)
            .quiet_zone(true)
            .build();

        let white_rect = format!(
            r#"<rect x="{x}%" y="{y}%" width="{w}%" height="{h}%" fill="white"/>"#,
            x = 50 - args.square / 2,
            y = 50 - args.square / 2,
            w = args.square,
            h = args.square
        );

        let font_size = args.font_size;

        let text_svg = format!(
            r#"<text x="50%" y="50%"
                     dominant-baseline="middle"
                     text-anchor="middle"
                     font-size="{font_size}"
                     font-family="Arial"
                     fill="black">{letters}</text>"#,
        );

        svg_xml.replace("</svg>", &format!("{white_rect}\n{text_svg}\n</svg>"))
    } else {
        code.render::<svg::Color>().min_dimensions(512, 512).build()
    };

    let filename = sanitize_filename(&args.uri);
    fs::write(format!("{filename}.svg"), final_svg).unwrap();
}
