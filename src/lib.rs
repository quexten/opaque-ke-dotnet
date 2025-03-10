uniffi::include_scaffolding!("opaque_ke");

pub extern fn hello_ffi() -> String {
    println!("Hello from Rust!");
    "Hello from Rust!".to_string()
}