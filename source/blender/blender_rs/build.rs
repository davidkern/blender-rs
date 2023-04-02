use std::path::PathBuf;
use std::env;

fn main() {
    // let variables: Vec<(String, String)> = std::env::vars().collect();
    // panic!("{:?}", variables);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-I/home/david/src/blender-project/blender/intern/clod")
        .clang_arg("-I/home/david/src/blender-project/blender/intern/guardedalloc")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/blenkernel")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/blenlib")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/blenloader")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/depsgraph")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/editors/include")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/gpu")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/imbuf")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/io/usd")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/makesdna")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/makesrna")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/render")
        .clang_arg("-I/home/david/src/blender-project/blender/source/blender/windowmanager")
      
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}