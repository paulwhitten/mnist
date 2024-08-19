use clap::Parser;
use std::{fs::File, io::{Read, Seek, SeekFrom}, process::exit};

/**
 * convert slice to array
 */
fn poof(my_slice: &[u8]) -> [u8; 4] {
    my_slice.try_into().expect("Invalid slice size")
}

fn main() {
    let args = Args::parse();

    let mut f = File::open(args.file_name).unwrap();
    let mut file_header: [u8; 16] = [0; 16];
    f.read(& mut file_header).expect("problem reading header");
    let magic_num = u32::from_be_bytes(poof(&file_header[0..4]));
    let num_images = u32::from_be_bytes(poof(&file_header[4..8]));
    let rows = u32::from_be_bytes(poof(&file_header[8..12]));
    let columns = u32::from_be_bytes(poof(&file_header[12..16]));

    println!("magic: {}, num_images: {}, rows: {}, columns: {}", magic_num, num_images, rows, columns);

    // confirm we can read arg.number
    if args.number >= magic_num {
        println!("Trying to read image {} which is greater than {}", args.number, num_images - 1);
        exit(1);
    }

    f.seek(SeekFrom::Current((args.number * rows * columns) as i64)).expect("problem seeking");
    let mut buf = vec![0; (rows * columns) as usize];
    f.read(&mut buf).expect("problem reading");
    println!("{:?}", &buf);

    image::save_buffer("myfile.png", &buf, columns, rows, image::ColorType::L8).expect("problem saving image");
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Name of the image file to open
    #[arg(short, long)]
    file_name: String,

    /// Zero based image number to extract
    #[arg(short, long)]
    number: u32,
}
