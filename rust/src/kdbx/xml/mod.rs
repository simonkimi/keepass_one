pub mod database;
pub mod entities;
pub mod errors;
pub mod protected_value;

#[cfg(test)]
mod tests {
    use crate::kdbx::xml::entities::KeePassDocument;
    use serde::Serialize;
    use std::fs;

    #[test]
    fn test_xml_serialization_deserialization() {
        let xml_data = fs::read_to_string("demo.xml").unwrap();
        let deserialized: KeePassDocument = quick_xml::de::from_str(&xml_data).unwrap();

        let mut serialized = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut serialized);
        ser.expand_empty_elements(true);
        deserialized.serialize(ser).unwrap();

        fs::write("src/kdbx/xml/test_serialized.xml", serialized).unwrap();
    }
}
