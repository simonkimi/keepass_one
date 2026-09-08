pub mod database;
pub mod entities;
pub mod errors;
pub mod protected_value;

#[cfg(test)]
mod tests {
    use crate::kdbx::xml::entities::KeePassFile;
    use serde::Serialize;
    use std::fs;

    #[test]
    #[ignore = "requires local demo.xml fixture"]
    fn test_xml_serialization_deserialization() {
        let xml_data = fs::read_to_string("demo.xml").unwrap();
        let deserialized: KeePassFile = quick_xml::de::from_str(&xml_data).unwrap();

        let mut serialized = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut serialized);
        ser.expand_empty_elements(true);
        deserialized.serialize(ser).unwrap();

        fs::write("src/kdbx/xml/test_serialized.xml", serialized).unwrap();
    }

    #[test]
    fn test_nullable_bool_and_autotype_defaults() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<KeePassFile>
  <Meta>
    <Generator>KeePassOne</Generator>
    <SettingsChanged>2020-01-01T00:00:00Z</SettingsChanged>
    <DatabaseName>test</DatabaseName>
    <DatabaseNameChanged>2020-01-01T00:00:00Z</DatabaseNameChanged>
    <DatabaseDescription></DatabaseDescription>
    <DatabaseDescriptionChanged>2020-01-01T00:00:00Z</DatabaseDescriptionChanged>
    <DefaultUserName></DefaultUserName>
    <DefaultUserNameChanged>2020-01-01T00:00:00Z</DefaultUserNameChanged>
    <MaintenanceHistoryDays>365</MaintenanceHistoryDays>
    <Color></Color>
    <MasterKeyChanged>2020-01-01T00:00:00Z</MasterKeyChanged>
    <MasterKeyChangeRec>-1</MasterKeyChangeRec>
    <MasterKeyChangeForce>-1</MasterKeyChangeForce>
    <MemoryProtection>
      <ProtectTitle>False</ProtectTitle>
      <ProtectUserName>False</ProtectUserName>
      <ProtectPassword>True</ProtectPassword>
      <ProtectURL>False</ProtectURL>
      <ProtectNotes>False</ProtectNotes>
    </MemoryProtection>
    <CustomIcons></CustomIcons>
    <RecycleBinEnabled>False</RecycleBinEnabled>
    <RecycleBinChanged>2020-01-01T00:00:00Z</RecycleBinChanged>
    <EntryTemplatesGroupChanged>2020-01-01T00:00:00Z</EntryTemplatesGroupChanged>
    <HistoryMaxItems>10</HistoryMaxItems>
    <HistoryMaxSize>6291456</HistoryMaxSize>
  </Meta>
  <Root>
    <Group>
      <UUID>AQEBAQEBAQEBAQEBAQEBAQ==</UUID>
      <Name>Root</Name>
      <IconID>0</IconID>
      <Times>
        <LastAccessTime>2020-01-01T00:00:00Z</LastAccessTime>
        <ExpiryTime>2020-01-01T00:00:00Z</ExpiryTime>
        <Expires>False</Expires>
        <UsageCount>0</UsageCount>
        <LocationChanged>2020-01-01T00:00:00Z</LocationChanged>
      </Times>
      <EnableAutoType></EnableAutoType>
      <EnableSearching>Null</EnableSearching>
      <Entry>
        <UUID>AgICAgICAgICAgICAgICAg==</UUID>
        <IconID>0</IconID>
        <Times>
          <LastAccessTime>2020-01-01T00:00:00Z</LastAccessTime>
          <ExpiryTime>2020-01-01T00:00:00Z</ExpiryTime>
          <Expires>False</Expires>
          <UsageCount>0</UsageCount>
          <LocationChanged>2020-01-01T00:00:00Z</LocationChanged>
        </Times>
      </Entry>
    </Group>
  </Root>
</KeePassFile>"#;
        let file: KeePassFile = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(file.root.group.entry.len(), 1);
    }
}
