use std::path::Path;

use ANPR_bind::{anpr_plate, AnprImage, AnprOptions, anpr_video}; // Adjust the module path

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments. For help print {} /?", args[0]);
        return Ok(());
    } else if args[1] == "help" || args[1] == "-help" || args[1] == "--help" || args[1] == "/?" {
        return Ok(());
    } else if args.len() < 3 {
        println!("Too few arguments. For help print {} /?", args[0]);
        return Ok(());
    }
    let path = args[2].clone();
    let type_number: i32 = args
        .get(1)
        .unwrap_or(&"104".to_string())
        .parse()
        .unwrap_or(104);

    let options = AnprOptions::default()
        .with_type_number(104)
        .with_vers("1.6.0");

       
    match Path::new(&path).extension().and_then(|s| s.to_str()) {
        Some(ext)
            if ext.eq_ignore_ascii_case("jpg")
                || ext.eq_ignore_ascii_case("jpeg")
                || ext.eq_ignore_ascii_case("png") =>
        {
            let img = AnprImage::load_image(&path)?;
            let plate_numbers = anpr_plate(&img, &options)?;

            for plate in plate_numbers {
                println!("{}", plate);
            }
        }
        Some(ext) if ext.eq_ignore_ascii_case("avi") || ext.eq_ignore_ascii_case("mp4") => {
            anpr_video(Some(path), type_number)?;
        }
        _ => {
            if path.starts_with("http") || path.starts_with("rtsp") {
                anpr_video(Some(path), type_number)?;
            } else if path.starts_with("/dev/video") {
                // Assuming Linux device file for camera
                anpr_video(Some(path), type_number)?;
            } else {
                println!("Unsupported file type or URL. Please provide a valid image, video file, or URL.");
                return Ok(());
            }
        }
    }
    Ok(())
}
