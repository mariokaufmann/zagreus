use std::borrow::Cow;
use std::path::Path;

use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriterEvent;

use crate::data::DataElements;
use crate::error::ZagreusError;

pub fn process_svg(input_file_path: &Path, output_file_path: &Path) -> Result<DataElements, ZagreusError> {
    let template_reader = crate::build::transform::create_xml_reader(input_file_path);
    let mut processed_template_writer = crate::build::transform::create_xml_writer(output_file_path);

    let mut found_elements = Vec::new();

    for e in template_reader {
        match e {
            Ok(evt) => {
                if let ReaderEvent::StartElement { name: _, attributes, namespace: _ } = &evt {
                    for attribute in attributes {
                        if attribute.name.local_name.eq("id") {
                            found_elements.push(attribute.value.clone());
                        }
                    }
                }

                match transform_event(&evt) {
                    Some(transformed_event) => {
                        if let Err(err) = processed_template_writer.write(transformed_event) {
                            error!("Could not write event to output SVG file: {}.", err);
                        }
                    }
                    None => warn!("Could not transform event, it is skipped {:?}", evt),
                }
            }
            Err(err) => {
                error!("Could not read XML event: {}.", err);
                break;
            }
        }
    }

    Ok(DataElements::new(found_elements))
}

fn transform_event(event: &ReaderEvent) -> Option<WriterEvent> {
    match event {
        ReaderEvent::StartDocument { .. } => None,
        ReaderEvent::StartElement { name, attributes, namespace } => {
            let attributes = attributes.iter().map(|attribute| attribute.borrow()).collect();
            Option::Some(WriterEvent::StartElement { name: name.borrow(), attributes, namespace: Cow::Borrowed(namespace) })
        }
        other => other.as_writer_event()
    }
}

#[cfg(test)]
mod tests {
    use crate::fs::temp::TempFolder;

    use super::*;

    #[test]
    fn process_valid_from_affinity_designer() {
        let temp_folder = TempFolder::new().unwrap();
        let input_file_path = Path::new("fixtures/svg/valid.svg");
        let expected_output_path = Path::new("fixtures/svg/valid_processed.svg");
        let actual_output_path = temp_folder.join("output.svg");

        process_svg(input_file_path, &actual_output_path).unwrap();

        let actual_contents = std::fs::read_to_string(actual_output_path).unwrap();
        let expected_contents = std::fs::read_to_string(expected_output_path).unwrap();

        assert_eq!(actual_contents, expected_contents);
    }
}

