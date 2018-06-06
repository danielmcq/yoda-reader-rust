use local_types::to_u16le;
use local_types::to_u32le;
use local_types::to_u32;
use std::io::Read;
use std::env;
use std::fs::File;
use std::io::BufReader;

mod local_types;
mod tile;

fn get_input_filename_from_environment () -> String {
    static YODA_INPUT_FILE_ENV_KEY: &'static str = "YODA_INPUT_FILE";//'
    let input_filename = String::new();

    match env::var(YODA_INPUT_FILE_ENV_KEY) {
        Ok(input_filename) => input_filename,
        Err(_) => {
            println!("Environment variable {:?} could not be read.", YODA_INPUT_FILE_ENV_KEY);
            input_filename
        },
    }
}

fn get_input_filename_from_args () -> String {
    let mut input_filename = String::new();
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        input_filename = args[1].clone();
    }

    input_filename
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

fn sound_filename_size_reader <R> (reader: R) -> u32
where
    R: Read,
{
    let sound_filename_size = read_n(reader, 2);
    let sound_filename_size = to_u16le(&sound_filename_size);
    // println!("    Sound filename size: {}", sound_filename_size.clone());
    sound_filename_size.into()
}

fn generic_section_reader <R> (mut reader: R)
where
    R: Read,
{
    let section_size = section_size_reader(&mut reader);
    let _section_data = read_n(&mut reader, section_size);
}

fn sound_section_reader <R> (mut reader: R)
where
    R: Read,
{
    let mut total_sounds = 0;
    let mut section_size = section_size_reader(&mut reader);
    let mut sounds: Vec<String> = vec![];

    read_n(&mut reader, 2);
    section_size = section_size - 2;

    while section_size > 0 {

        let sound_filename_length = sound_filename_size_reader(&mut reader);
        section_size = section_size - 2;
        // println!("    Sound filename length: {}", sound_filename_length);

        let mut sound_filename_data = read_n(&mut reader, sound_filename_length);
        sound_filename_data.pop();
        let sound_filename = match String::from_utf8(sound_filename_data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("    Sound {:#} filename: {}", total_sounds, sound_filename);
        sounds.push(sound_filename);
        total_sounds = total_sounds + 1;
        section_size = section_size - sound_filename_length;
    }
    // println!("Sounds: {:?}", sounds);
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
            "SNDS" => sound_section_reader(&mut reader),
            "PUZ2" => generic_section_reader(&mut reader),
            "CHAR" => generic_section_reader(&mut reader),
            "CHWP" => generic_section_reader(&mut reader),
            "CAUX" => generic_section_reader(&mut reader),
            "TNAM" => generic_section_reader(&mut reader),
            "TILE" => tile::parse_section(&mut reader),
            "ZONE" => zone_reader(&mut reader),
            "ENDF" => break,
            _ => panic!("Unknown section: {:?}", section),
        }
    }
}

fn get_input_filename () -> String {
    let mut input_filename = get_input_filename_from_environment();

    if input_filename.is_empty() {
        input_filename = get_input_filename_from_args();
    }

    input_filename
}

fn main() {
    let input_filename = get_input_filename();

    println!("Opening file: {:?}", input_filename);
    match File::open(&input_filename) {
        Ok(file) => parse_input_file(&file),
        Err(e) => panic!("Failed to open file named: {:?}: {}", &input_filename, e),
    };
}
