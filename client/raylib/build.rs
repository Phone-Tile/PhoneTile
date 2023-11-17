use std::{path::PathBuf,env};


pub fn main() {
    //track file
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let binding = env::var("TARGET").unwrap();
    let binding : Vec<&str>= binding.split('-').collect();
    let arch = binding[0];
    let make_arch = match arch {
        "armv7" => "arm",
        "aarch64" => "arm64",
        "i686" => "x86",
        "i386" => "x86",
        a => a
    };


    let ndk_home = env::var("NDK_HOME").unwrap_or("../android/ndk".to_string());
    let ndk_home = ndk_home.as_str();

    //generate bindings
    generate_bindings();

    compile_lib(arch, make_arch, ndk_home);

    link_lib(arch,ndk_home);

}

fn link_lib(arch : &str, ndk_home : &str){
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:warning={}", out_path.clone().display());
    println!("cargo:rustc-link-search=native={}",out_path.display());
    println!("cargo:rustc-link-search=native=lib/{}",arch);
    println!("cargo:rustc-link-search=native={ndk_home}/toolchains/llvm/prebuilt/linux-x86_64/lib/clang/17/lib/linux/{arch}");

    println!("cargo:rustc-link-lib=static=raylib");

    println!("cargo:rustc-link-search=native=../android");
    println!("cargo:rustc-link-search=native=android/build/obj");

    let libs = [
        "native_app_glue",
        "log",
        "android",
        "EGL",
        "GLESv2",
        "OpenSLES",
        "atomic",
        "m",
        "dl",
        "c",
    ];

    for lib in libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

}
pub fn get_blocked_enum_names() -> Vec<String> {
    vec![
        "BlendMode",
        "CameraMode",
        "CameraProjection",
        "ConfigFlags",
        "CubemapLayout",
        "FontType",
        "GamepadAxis",
        "GamepadButton",
        "Gesture",
        "KeyboardKey",
        "MaterialMapIndex",
        "MouseButton",
        "MouseCursor",
        "NPatchLayout",
        "PixelFormat",
        "ShaderAttributeDataType",
        "ShaderLocationIndex",
        "ShaderUniformDataType",
        "TextureFilter",
        "TextureWrap",
        "TraceLogLevel"
    ].into_iter().map(|s| s.to_string()).collect()
}

fn generate_bindings(){
    let header_path = "src/wrapper.h";

    let builder = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
       // .blocklist_item("DE2GRAD")
       // .blocklist_item("PI")
       // .blocklist_item("RAD2DEG")
       // .blocklist_item("__GNU_VA_LIST")
       // .blocklist_item("__bool_true_false_are_defined")
       // .blocklist_item("false_")
       // .blocklist_item("true_")
        ;


    //for enum_name in get_blocked_enum_names(){
    //    builder = builder.blocklist_type(format!("{}.*",enum_name))
    //}

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings !");
}

fn compile_lib(arch : &str, make_arch : &str, ndk_home : &str){
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    if !std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("clean")
        .output()
        .expect("could not spawn `make`")
        .status
        .success()
    {
        panic!("error in make");
    }
    println!("cargo:warning={}", "start");
    println!("cargo:warning={}", arch);
    let make = std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("PLATFORM=PLATFORM_ANDROID")
        .arg(format!("ANDROID_NDK={ndk_home}"))
        .arg(format!("ANDROID_ARCH={}", make_arch))
        .arg("ANDROID_API_VERSION=29")
        .arg(format!("RAYLIB_RELEASE_PATH={}",out_path.display()))
        .status()
        .expect("could not spawn `make`");
    if !make.success()
    {
        panic!("error in make {}", make);
    }
    println!("cargo:warning={}", "clean");
    if !std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("clean")
        .output()
        .expect("could not spawn `make`")
        .status
        .success()
    {
        panic!("error in make");
    }
}
