

use ANPR_bind::{AnprImage, AnprOptions, anpr_plate}; // Adjust the module path

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
    
    let img_path = args[2].clone();
    let save_path = "gray.jpg";

    let img = AnprImage::load_image(&img_path)?;
    let options = AnprOptions::new("1.6.0");

    let plate_numbers = anpr_plate(&img, &options)?;

    for plate in plate_numbers {
        println!("{}", plate);
    }

    Ok(())
}