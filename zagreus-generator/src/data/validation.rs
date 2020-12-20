use std::collections::HashMap;

use crate::data::DataElements;
use crate::error::ZagreusError;

pub trait ConfigValidate {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError>;
}

pub struct ValidationData<'a> {
    pub data_elements: &'a DataElements,
}

fn count_elements_grouped<'a, T, F>(elements: &'a [T], mapping_function: F)
                                    -> HashMap<&str, u16> where F: Fn(&'a T) -> &'a str {
    let mut result = HashMap::new();
    for element in elements {
        let id = mapping_function(element);
        let new_count = match result.get(id) {
            Some(previous_mapping) => previous_mapping + 1,
            None => 1,
        };
        result.insert(id, new_count);
    }
    result
}

pub fn get_duplicate_elements<'a, T, F>(elements: &'a [T], mapping_function: F)
                                        -> Vec<&'a str> where F: Fn(&'a T) -> &'a str {
    let mut element_counts = count_elements_grouped(elements, mapping_function);
    element_counts.drain()
        .filter(|(_, value)| {
            *value > 1
        })
        .map(|(key, _)| key)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestElement {
        id: String,
    }

    #[test]
    fn count_elements_grouped_multiple() {
        let test_elements = create_elements(true);

        let grouped_elements = count_elements_grouped(&test_elements, |animation| &animation.id);

        assert_eq!(grouped_elements.len(), 2);

        let id1_count = grouped_elements.get("id1");
        assert!(id1_count.is_some());
        let id1_count = id1_count.unwrap();
        assert_eq!(*id1_count, 2);

        let id2_count = grouped_elements.get("id2");
        assert!(id2_count.is_some());
        let id2_count = id2_count.unwrap();
        assert_eq!(*id2_count, 1);
    }

    #[test]
    fn count_elements_grouped_single() {
        let test_elements = create_elements(false);

        let grouped_elements = count_elements_grouped(&test_elements, |animation| &animation.id);

        assert_eq!(grouped_elements.len(), 2);

        let id1_count = grouped_elements.get("id1");
        assert!(id1_count.is_some());
        let id1_count = id1_count.unwrap();
        assert_eq!(*id1_count, 1);

        let id2_count = grouped_elements.get("id2");
        assert!(id2_count.is_some());
        let id2_count = id2_count.unwrap();
        assert_eq!(*id2_count, 1);
    }

    #[test]
    fn get_duplicates_present() {
        let test_elements = create_elements(true);

        let duplicates = get_duplicate_elements(&test_elements, |animation| &animation.id);

        assert_eq!(duplicates.len(), 1);

        let duplicate = duplicates.get(0);
        assert!(duplicate.is_some());
        let duplicate = duplicate.unwrap();
        assert_eq!(*duplicate, "id1");
    }

    #[test]
    fn get_duplicates_none() {
        let test_elements = create_elements(false);

        let duplicates = get_duplicate_elements(&test_elements, |animation| &animation.id);

        assert!(duplicates.is_empty());
    }

    fn create_elements(create_duplicates: bool) -> Vec<TestElement> {
        let mut test_elements = vec![
            TestElement { id: String::from("id1") },
            TestElement { id: String::from("id2") },
        ];

        if create_duplicates {
            test_elements.push(TestElement { id: String::from("id1") });
        }

        test_elements
    }
}
