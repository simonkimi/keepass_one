#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

/// Open a KDBX key file and return the key
#[flutter_rust_bridge::frb(sync)]
pub fn open_kdbx_key_file(key_file: &[u8]) -> Vec<u8> {
    crate::kdbx::keys::parse_keyfile(key_file)
}
