use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .rustfmt_bindings(true)
        .whitelist_function("dispatch_queue_create")
        .whitelist_function("xpc.*")
        .whitelist_type("_os_alloc_once_s")
        .whitelist_type("uuid_t")
        .whitelist_type("xpc_global_data")
        .whitelist_type("xpc_pipe_t")
        .whitelist_var("XPC.*")
        .whitelist_var("_os_alloc_once_table")
        .whitelist_var("_xpc.*")
        .whitelist_var("xpc.*");

    if let Ok(cflags) = std::env::var("CFLAGS") {
        let cflags = cflags
            .split_ascii_whitespace()
            .map(|slice| slice.to_string())
            .collect::<Vec<String>>();
        builder = builder.clang_args(cflags);
    }

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
