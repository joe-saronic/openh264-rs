use walkdir::WalkDir;

fn ugly_cpp_import(x: &str) -> Vec<String> {
    WalkDir::new(x)
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.path().to_str().unwrap().ends_with("cpp"))
        .map(|x| x.path().to_str().unwrap().to_string())
        // Otherwise fails when compiling on Linux
        .filter(|x| !x.contains("DllEntry.cpp"))
        .collect()
}

fn main() {
    cc::Build::new()
        .include("upstream/codec/api/svc/")
        .include("upstream/codec/common/inc/")
        .include("upstream/codec/decoder/core/inc/")
        .include("upstream/codec/decoder/plus/inc/")
        .include("upstream/codec/processing/interface/")
        .files(ugly_cpp_import("upstream/codec/common"))
        .files(ugly_cpp_import("upstream/codec/decoder"))
        .cpp(true)
        .warnings(false)
        .opt_level(3)
        .pic(true)
        // Upstream sets these two and if we don't we get segmentation faults on Linux and MacOS ... Happy times.
        // .flag_if_supported("-fno-strict-aliasing")
        // .flag_if_supported("-fstack-protector-all")
        // .flag_if_supported("-fembed-bitcode")
        // .flag_if_supported("-fsanitize=address")
        // .flag_if_supported("-fno-common")
        // .flag_if_supported("-undefined dynamic_lookup")
        // .debug(true)
        .compile("libopenh264_decode.a");

    cc::Build::new()
        .include("upstream/codec/api/svc/")
        .include("upstream/codec/common/inc/")
        .include("upstream/codec/encoder/core/inc/")
        .include("upstream/codec/encoder/plus/inc/")
        .include("upstream/codec/processing/interface/")
        .include("upstream/codec/processing/src/common/")
        .files(ugly_cpp_import("upstream/codec/encoder"))
        .files(ugly_cpp_import("upstream/codec/processing"))
        .cpp(true)
        .warnings(false)
        .opt_level(3)
        .pic(true)
        // Upstream sets these two and if we don't we get segmentation faults on Linux and MacOS ... Happy times.
        // .flag_if_supported("-fno-strict-aliasing")
        // .flag_if_supported("-fstack-protector-all")
        // .flag_if_supported("-fembed-bitcode")
        // .flag_if_supported("-fsanitize=address")
        // .flag_if_supported("-fno-common")
        // .flag_if_supported("-undefined dynamic_lookup")
        // .debug(true)
        .compile("libopenh264_encode.a");

    println!("cargo:rustc-link-lib=static=openh264_encode");
    println!("cargo:rustc-link-lib=static=openh264_decode");
    // println!("cargo:rustc-link-arg=-fsanitize=address");
    // println!("cargo:rustc-flags=-fsanitize=address");
}
