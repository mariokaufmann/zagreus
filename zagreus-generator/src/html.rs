use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use xml::reader::XmlEvent as ReaderEvent;

const HTML_PART_1: &str = "<html><head><meta charset=\"UTF-8\" />";
const HTML_PART_2: &str = "</head><body><div id=\"zagreus-svg-container\" class=\"zagreus-hidden\">";
const HTML_PART_3: &str = "</div><script src=\"/static/zagreus-runtime.js\"></script></body></html>";

pub fn write_raw_html(processed_template_path: &PathBuf, raw_html_path: &PathBuf, template_name: &str, stylesheets: &Vec<String>) {
    let mut raw_html_file = File::create(raw_html_path).unwrap();
    let processed_template_data = std::fs::read(processed_template_path).unwrap();

    raw_html_file.write_all(HTML_PART_1.as_bytes()).unwrap();

    // write base tag
    raw_html_file.write_all(format!("<base href=\"/static/template/{}/\" />", template_name).as_bytes()).unwrap();
    // write stylesheets
    for stylesheet in stylesheets {
        raw_html_file.write_all(format!("<link rel=\"stylesheet\" type=\"text/css\" href=\"assets/{}\" />", stylesheet).as_bytes()).unwrap();
    }

    raw_html_file.write_all(HTML_PART_2.as_bytes()).unwrap();
    raw_html_file.write_all(processed_template_data.as_slice()).unwrap();
    raw_html_file.write_all(HTML_PART_3.as_bytes()).unwrap();
}

pub fn process_raw_html(raw_html_path: &PathBuf, processed_html_path: &PathBuf) {
    let html_reader = crate::transform::create_xml_reader(&raw_html_path);
    let mut html_writer = crate::transform::create_xml_writer(&processed_html_path);

    for evt in html_reader {
        let reader_event = evt.unwrap();
        let writer_event = match &reader_event {
            ReaderEvent::StartDocument { .. } => None,
            reader_event => reader_event.as_writer_event(),
        };
        if let Some(writer_event) = writer_event {
            html_writer.write(writer_event).unwrap();
        }
    }
}