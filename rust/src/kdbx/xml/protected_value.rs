use crate::kdbx::xml::entities::{Entry, Group, KeePassDocument, Value};

pub fn collect_protected_values_document(document: &mut KeePassDocument) {
    collect_protected_values_group(&mut document.root.group, 0);
}

fn collect_protected_values_group(group: &mut Group, stream_offset: usize) -> usize {
    let mut stream_offset = stream_offset;
    for entry in &mut group.entry {
        stream_offset = collect_protected_values_entry(entry, stream_offset);
    }
    for group in &mut group.group {
        stream_offset = collect_protected_values_group(group, stream_offset);
    }
    stream_offset
}

fn collect_protected_values_entry(entry: &mut Entry, stream_offset: usize) -> usize {
    let mut stream_offset = stream_offset;

    stream_offset = process_protected_values(stream_offset, entry);
    if let Some(ref mut history) = entry.history {
        for history_entry in &mut history.entry {
            stream_offset = process_protected_values(stream_offset, history_entry);
        }
    }
    stream_offset
}

fn process_protected_values(stream_offset: usize, entry: &mut Entry) -> usize {
    let mut stream_offset = stream_offset;
    for value in &mut entry.string {
        if let Value::Protected {
            ref value,
            ref mut offset,
        } = value.value
        {
            *offset = Some(stream_offset);
            stream_offset += value.len();
        }
    }
    stream_offset
}
