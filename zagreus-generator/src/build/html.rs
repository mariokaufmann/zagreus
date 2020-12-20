use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use xml::reader::XmlEvent as ReaderEvent;

const HTML_PART_1: &str = "<html><head><meta charset=\"UTF-8\" />";
const HTML_PART_2: &str = "</head><body><div id=\"zagreus-svg-container\" class=\"zagreus-hidden\">";
const HTML_PART_3: &str = "</div><script src=\"/static/zagreus-runtime.js\"></script></body></html>";

pub fn write_raw_html(processed_template_path: &Path, raw_html_path: &Path, template_name: &str, mut stylesheets: HashSet<String>) {
    let mut raw_html_file = File::create(raw_html_path).unwrap();
    let processed_template_data = std::fs::read(processed_template_path).unwrap();

    raw_html_file.write_all(HTML_PART_1.as_bytes()).unwrap();

    // write base tag
    raw_html_file.write_all(format!("<base href=\"/static/template/{}/\" />", template_name).as_bytes()).unwrap();
    // sort stylesheet names for a stable conversion
    let mut stylesheets: Vec<String> = stylesheets.drain().collect();
    stylesheets.sort();
    for stylesheet in stylesheets {
        raw_html_file.write_all(format!("<link rel=\"stylesheet\" type=\"text/css\" href=\"assets/{}\" />", stylesheet).as_bytes()).unwrap();
    }

    raw_html_file.write_all(HTML_PART_2.as_bytes()).unwrap();
    raw_html_file.write_all(processed_template_data.as_slice()).unwrap();
    raw_html_file.write_all(HTML_PART_3.as_bytes()).unwrap();
}

// This step is mainly there to pretty print the RAW HTML
pub fn process_raw_html(raw_html_path: &Path, processed_html_path: &Path) {
    let html_reader = crate::build::transform::create_xml_reader(&raw_html_path);
    let mut html_writer = crate::build::transform::create_xml_writer(&processed_html_path);

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::fs::temp::TempFolder;

    use super::*;

    #[test]
    fn write_raw_html_valid() {
        let temp_folder = TempFolder::new().unwrap();
        let processed_template_path = Path::new("fixtures/svg/valid_processed.svg");
        let expected_output_path = Path::new("fixtures/html/valid_raw.html");
        let actual_output_path = temp_folder.join("index_raw.html");

        let stylesheets: HashSet<String> = [
            String::from("assets/main.css"),
            String::from("assets/animations.css"),
        ].iter().cloned().collect();
        write_raw_html(
            processed_template_path,
            &actual_output_path,
            "test-template",
            stylesheets,
        );

        let actual_contents = std::fs::read_to_string(actual_output_path).unwrap();
        let expected_contents = std::fs::read_to_string(expected_output_path).unwrap();

        assert_eq!(actual_contents, expected_contents);
    }

    #[test]
    fn process_raw_html_valid() {
        let temp_folder = TempFolder::new().unwrap();
        let raw_html_path = Path::new("fixtures/html/valid_raw.html");
        let expected_output_path = Path::new("fixtures/html/valid_processed.html");
        let actual_output_path = temp_folder.join("index.html");

        process_raw_html(
            raw_html_path,
            &actual_output_path,
        );

        let actual_contents = std::fs::read_to_string(actual_output_path).unwrap();
        let expected_contents = std::fs::read_to_string(expected_output_path).unwrap();

        assert_eq!(actual_contents, expected_contents);
    }
}
