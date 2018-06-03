use std::io::Read;
use std::env;
use std::fs::File;
use std::io::BufReader;


fn get_input_filename_from_environment () -> std::string::String {
    static YODA_INPUT_FILE_ENV_KEY: &'static str = "YODA_INPUT_FILE";//'
    let yoda_input_file_name = String::new();

    match env::var(YODA_INPUT_FILE_ENV_KEY) {
        Ok(yoda_input_file_name) => yoda_input_file_name,
        Err(_) => yoda_input_file_name,
    }
}

/**
 * read_n
 * Credit to https://stackoverflow.com/questions/30412521/how-to-read-a-specific-number-of-bytes-from-a-stream#30413877
 */
fn read_n<R>(reader: R, bytes_to_read: u32) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read.into());
    // Do appropriate error handling for your situation
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    buf
}

/**
 * to_u32
 * Credit to https://www.reddit.com/r/rust/comments/36ixl0/converting_a_vector_of_bits_to_an_integer/crehkpw/
 */
fn to_u32(slice: &Vec<u8>) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

fn to_u32le(slice: &Vec<u8>) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc.rotate_left(8) + b as u32)
}

fn to_u16(slice: &Vec<u8>) -> u16 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u16)
}

fn to_u16le(slice: &Vec<u8>) -> u16 {
    slice.iter().rev().fold(0, |acc, &b| acc.rotate_left(8) + b as u16)
}

fn version_reader <R> (reader: R)
where
    R: Read,
{
    let version = read_n(reader, 4);
    let version = to_u32(&version);
    println!("    Version: {}", version);
}

fn section_size_reader <R> (reader: R) -> u32
where
    R: Read,
{
    let section_size = read_n(reader, 4);
    let section_size = to_u32le(&section_size);
    println!("    Section size: {}", section_size.clone());
    section_size
}

fn zone_size_reader <R> (reader: R) -> u32
where
    R: Read,
{
    let zone_size = read_n(reader, 4);
    let zone_size = to_u32le(&zone_size);
    // println!("    Zone size: {}", zone_size.clone());
    zone_size
}

fn generic_section_reader <R> (mut reader: R)
where
    R: Read,
{
    let section_size = section_size_reader(&mut reader);
    let _section_data = read_n(&mut reader, section_size);
}

fn tile_reader <R> (mut reader: R)
where
    R: Read,
{

    let tile_section_size = section_size_reader(&mut reader);
    let _tile_section_data = read_n(&mut reader, tile_section_size);
}

fn zone_reader <R> (mut reader: R)
where
    R: Read,
{
    let zone_count = read_n(&mut reader, 2);
    let zone_count = to_u16le(&zone_count);
    println!("    Zone count: {}", zone_count);
    for _ in 0..zone_count {
        let _zone_flags = read_n(&mut reader, 2);
        // let zone_flags = to_u16(&zone_flags);
        // println!("        Zone flags: {:#b}", zone_flags);
        let zone_size = zone_size_reader(&mut reader);
        let _zone_data = read_n(&mut reader, zone_size);
    }
}

fn parse_input_file (file: &File) {
    let mut reader = BufReader::new(file);
    const SECTION_NAME_SIZE: u32 = 4;

    loop {
        let section = read_n(&mut reader, SECTION_NAME_SIZE);
        let section = String::from_utf8(section).unwrap();
        println!("Read section: {}", section);

        match section.as_ref() {
            "VERS" => version_reader(&mut reader),
            "STUP" => generic_section_reader(&mut reader),
            "SNDS" => generic_section_reader(&mut reader),
            "PUZ2" => generic_section_reader(&mut reader),
            "CHAR" => generic_section_reader(&mut reader),
            "CHWP" => generic_section_reader(&mut reader),
            "CAUX" => generic_section_reader(&mut reader),
            "TNAM" => generic_section_reader(&mut reader),
            "TILE" => tile_reader(&mut reader),
            "ZONE" => zone_reader(&mut reader),
            "ENDF" => break,
            _ => panic!("Unknown section: {:?}", section),
        }
    }
}

fn main() {
    let yoda_input_file_name = get_input_filename_from_environment();

    println!("Opening file: {:?}", yoda_input_file_name);
    match File::open(&yoda_input_file_name) {
        Ok(file) => parse_input_file(&file),
        Err(e)  => panic!("Failed to open file named: {:?}: {}", &yoda_input_file_name, e),
    };
}
