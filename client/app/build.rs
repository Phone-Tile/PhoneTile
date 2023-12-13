use std::env;

fn main() {
    let target_info = env::var("TARGET").unwrap_or("x86_64-unknown-linux-gnu".to_string());
    let target_info: Vec<&str> = target_info.split('-').collect();
    let arch = target_info[0];
    let (_make_arch, lib_ndk, _lib_folder) = match arch {
        "armv7" => ("arm", "arm", "armeabi-v7a"),
        "aarch64" => ("arm64", "aarch64", "arm64"),
        "i686" => ("x86", "i386", "x86"),
        a => (a, a, a),
    };

    let target_sys = target_info[2];

    if target_sys == "android" || target_sys == "androideabi" {
        build_android(lib_ndk);
    } else if target_sys == "linux" {
    } else {
        panic!("target {} not supported", target_sys);
    }
}

fn build_android(lib_ndk: &str) {
    #[cfg(target_os = "linux")]
    let sys = "linux";
    #[cfg(target_os = "macos")]
    let sys = "darwin";

    let ndk_home = env::var("NDK_HOME").unwrap_or("../android/ndk".to_string());

    let flags = [
        "-ffunction-sections",
        "-funwind-tables",
        "-fstack-protector-strong",
        "-fPIC",
        "-no-canonical-prefixes",
    ];
    for flag in flags {
        println!("cargo:rustc-link-arg={}", flag);
    }

    //for link some lib
    println!("cargo:rustc-link-arg=-I{ndk_home}/toolchains/llvm/prebuilt/{sys}-x86_64/sysroot/usr/include");
    println!(
        "cargo:rustc-link-arg=--sysroot={ndk_home}/toolchains/llvm/prebuilt/{sys}-x86_64/sysroot"
    );
    // useful for link unwind
    println!("cargo:rustc-link-arg=-L{ndk_home}/toolchains/llvm/prebuilt/{sys}-x86_64/lib/clang/17/lib/linux/{lib_ndk}");

    // some parameters
    println!("cargo:rustc-link-arg=-shared");
    println!("cargo:rustc-link-arg=-Wl,-soname,libphone_tile.so");
    println!("cargo:rustc-link-arg=-Wl,--exclude-libs,libatomic.a");
    println!("cargo:rustc-link-arg=-Wl,--build-id");
    println!("cargo:rustc-link-arg=-Wl,--no-undefined");
    println!("cargo:rustc-link-arg=-Wl,-z,noexecstack");
    println!("cargo:rustc-link-arg=-Wl,-z,relro");
    println!("cargo:rustc-link-arg=-Wl,-z,now");
    println!("cargo:rustc-link-arg=-Wl,--warn-shared-textrel");
    //println!("cargo:rustc-link-arg=-Wl,--fatal-warnings");
    println!("cargo:rustc-link-arg=-uANativeActivity_onCreate");
    //println!("cargo:rustc-link-arg=-Llib/{lib_folder}");
    println!("cargo:rustc-link-arg=-Wl,-G");

    println!("cargo:rustc-link-arg=-L../android");
    println!("cargo:rustc-link-arg=-Landroid/build/obj");

    println!("cargo:rustc-link-search=native=../android");
    println!("cargo:rustc-link-search=native=android/build/obj");
    //println!("cargo:rustc-link-search=lib/{lib_folder}");

    //println!("cargo:rustc-link-lib=static:+whole-archive=raylib");
}
