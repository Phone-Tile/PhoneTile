use std::{path::PathBuf,env};


pub fn main() {
    //track file
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let binding = env::var("TARGET").unwrap();
    let binding_vec : Vec<&str>= binding.split('-').collect();
    let arch = binding_vec[0];
    let (make_arch,_compiler) = match arch {
        "armv7" => ("arm","armv7a"),
        "aarch64" => ("arm64","aarch64"),
        "i686" => ("x86","i686"),
        a => (a,a)
    };

    let compiler = env::var("RUSTC_LINKER").unwrap();
    let compiler = compiler.as_str();

    let ndk_home = env::var("NDK_HOME").unwrap_or("../android/ndk".to_string());
    let ndk_home = ndk_home.as_str();

    //generate bindings
    generate_bindings(ndk_home);

    compile_lib(arch, make_arch,compiler,ndk_home);

    link_lib(arch,ndk_home);

}

fn link_lib(arch : &str, ndk_home :&str){
    println!("cargo:rustc-link-search=native=lib/{arch}");
    println!("cargo:rustc-link-search=native={ndk_home}/toolchains/llvm/prebuilt/linux-x86_64/lib/clang/17/lib/linux/{arch}");
}

//bindgen wrapper.h -o src/ffi_armv7.rs -- --sysroot=../../../raylibbind/android/ndk-save/sysroot --target=armv7-linux-androideabi

fn generate_bindings(ndk_home :&str){
    let header_path = "src/wrapper.h";

    let builder = bindgen::Builder::default()
        .header(header_path)
        .clang_arg(format!("--sysroot={ndk_home}/toolchains/llvm/prebuilt/linux-x86_64/sysroot"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));


    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings !");
}

fn compile_lib(_arch : &str, _make_arch : &str,compiler : &str,ndk_home :&str){
    cc::Build::new()
        .file(format!("{ndk_home}/sources/android/native_app_glue_modified/android_native_app_glue.c"))
        .flag(format!("--sysroot={ndk_home}/toolchains/llvm/prebuilt/linux-x86_64/sysroot").as_str())
        .compiler(compiler)
        .archiver(format!("{ndk_home}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"))
        .compile("libnative_app_glue.a");
}
