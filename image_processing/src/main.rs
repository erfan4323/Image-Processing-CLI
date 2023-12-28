use clap::ArgMatches;

mod args;
use args::i_args;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn main() {
    let binding = "".to_string();

    let matches: ArgMatches = i_args();

    let effect = matches.get_one::<String>("effect").unwrap().as_str();

    let infile = matches.get_one::<String>("input").unwrap_or(&binding);

    let outfile = matches.get_one::<String>("output").unwrap();

    if !(effect == "generate" || effect == "fractal") && infile.is_empty() {
        panic!("you must specify your input file.")
    }

    match effect {
        "blur" => blur(infile.clone(), outfile.clone()),
        "brighten" => brighten(infile.clone(), outfile.clone()),
        "crop" => crop(infile.clone(), outfile.clone()),
        "rotate" => rotate(infile.clone(), outfile.clone()),
        "invert" => invert(infile.clone(), outfile.clone()),
        "graysclae" => grayscale(infile.clone(), outfile.clone()),
        "generate" => generate(outfile.clone()),
        "fractal" => fractal(outfile.clone()),
        _ => println!("Please get some help"),
    }
}

fn blur(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    println!("Please enter the amount that you want to blur : ");
    read!(x as f32);

    let img2 = img.blur(x);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    println!("Please enter the amount of brightness you want : ");
    read!(x as i32);

    let img2 = img.brighten(x);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    println!("Please add four things : x, y of the image you want to crop and width, height of your new image");
    read!(x as u32);
    read!(y as u32);
    read!(width as u32);
    read!(height as u32);

    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    println!("Please enter your desired rotation (90, 180, 270) : ");
    read!(x as u16);

    let img2 = match x {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => panic!("Invalid rotation"),
    };
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    img.grayscale();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(width, height);

    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;
        let mut green = 0;
        while green < 255 {
            green += 1;
        }
        *pixel = image::Rgb([red, green, blue]);
    });
    imgbuf.save(outfile).unwrap();
}

fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    });

    imgbuf.save(outfile).unwrap();
}
