use std::collections::HashMap;

use crate::data::DataElements;
use crate::error::ZagreusError;

pub trait ConfigValidate {
    fn validate(&self, validation_data: &ValidationData) -> Result<(), ZagreusError>;
}

pub struct ValidationData<'a> {
    pub data_elements: &'a DataElements,
}

fn count_elements_grouped<'a, T, F>(elements: &'a Vec<T>, mapping_function: F)
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

pub fn get_duplicate_elements<'a, T, F>(elements: &'a Vec<T>, mapping_function: F)
                                        -> Vec<&'a str> where F: Fn(&'a T) -> &'a str {
    let mut element_counts = count_elements_grouped(elements, mapping_function);
    element_counts.drain()
        .filter(|(key, value)| {
            *value > 1
        })
        .map(|(key, value)| key)
        .collect()
}