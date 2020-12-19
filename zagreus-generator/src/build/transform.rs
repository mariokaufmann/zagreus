use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use xml::{EmitterConfig, EventReader, EventWriter};

pub fn create_xml_reader(input_path: &Path) -> EventReader<BufReader<File>> {
    let input_file = File::open(input_path).unwrap();
    let input_file = BufReader::new(input_file);
    EventReader::new(input_file)
}

pub fn create_xml_writer(output_path: &Path) -> EventWriter<File> {
    let file = File::create(output_path).unwrap();
    EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false)
        .normalize_empty_elements(false)
        .create_writer(file)
}