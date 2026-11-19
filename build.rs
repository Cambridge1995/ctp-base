use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include_dir = root.join("sdk/include");
    let wrapper = root.join("cpp/wrapper.h");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch != "x86_64" {
        panic!("当前 CTP SDK 仅支持 x86_64，实际目标架构：{target_arch}");
    }
    let platform_dir = match target_os.as_str() {
        "windows" => "sdk/windows",
        "linux" => "sdk/linux",
        other => panic!("当前原生桥接仅支持 Windows x64 和 Linux x64，实际目标系统：{other}"),
    };
    const MD_API_BASENAME: &str = "thostmduserapi_se";
    const TRADER_API_BASENAME: &str = "thosttraderapi_se";

    for file in [
        "build.rs",
        "cpp/wrapper.h",
        "cpp/bridge.h",
        "cpp/md_bridge.cpp",
        "cpp/trader_bridge.cpp",
        "src/sys/ffi/md.rs",
        "src/sys/ffi/trader.rs",
        "sdk/include/ThostFtdcMdApi.h",
        "sdk/include/ThostFtdcTraderApi.h",
        "sdk/include/ThostFtdcUserApiDataType.h",
        "sdk/include/ThostFtdcUserApiStruct.h",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    let runtime_libraries: [String; 2] = if target_os == "windows" {
        [
            format!("{MD_API_BASENAME}.dll"),
            format!("{TRADER_API_BASENAME}.dll"),
        ]
    } else {
        [
            format!("lib{MD_API_BASENAME}.so"),
            format!("lib{TRADER_API_BASENAME}.so"),
        ]
    };
    for library in &runtime_libraries {
        println!("cargo:rerun-if-changed={platform_dir}/{library}");
    }

    let bindings = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .allowlist_type("CThostFtdc.*Field")
        .derive_copy(true)
        .derive_default(true)
        .derive_debug(true)
        .generate_comments(false)
        .layout_tests(false)
        .generate()
        .unwrap_or_else(|error| panic!("bindgen 生成失败：{error}"));
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap_or_else(|error| panic!("写入 bindgen 结果失败：{error}"));

    cxx_build::bridges(["src/sys/ffi/md.rs", "src/sys/ffi/trader.rs"])
        .include(&root)
        .include(&include_dir)
        .files(["cpp/md_bridge.cpp", "cpp/trader_bridge.cpp"])
        .warnings(false)
        .extra_warnings(false)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/std:c++17")
        .flag_if_supported("/utf-8")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-fPIC")
        .compile("ctp_native_bridge");

    let library_dir = root.join(platform_dir);
    println!("cargo:rustc-link-search=native={}", library_dir.display());
    if target_os == "windows" {
        println!("cargo:rustc-link-lib={MD_API_BASENAME}");
        println!("cargo:rustc-link-lib={TRADER_API_BASENAME}");
    } else {
        println!("cargo:rustc-link-lib=dylib={MD_API_BASENAME}");
        println!("cargo:rustc-link-lib=dylib={TRADER_API_BASENAME}");
    }

    let profile_dir = out_dir.ancestors().nth(3).unwrap_or_else(|| {
        panic!(
            "无法从 OUT_DIR 定位 Cargo profile 目录：{}",
            out_dir.display()
        )
    });
    let runtime_libraries: [String; 2] = if target_os == "windows" {
        [
            format!("{MD_API_BASENAME}.dll"),
            format!("{TRADER_API_BASENAME}.dll"),
        ]
    } else {
        [
            format!("lib{MD_API_BASENAME}.so"),
            format!("lib{TRADER_API_BASENAME}.so"),
        ]
    };
    for library in &runtime_libraries {
        let source = library_dir.join(library);
        for destination_dir in [profile_dir.to_path_buf(), profile_dir.join("deps")] {
            fs::create_dir_all(&destination_dir).unwrap_or_else(|error| {
                panic!("创建目录 {} 失败：{error}", destination_dir.display())
            });
            let destination = destination_dir.join(library);
            fs::copy(&source, &destination).unwrap_or_else(|error| {
                panic!(
                    "复制 {} 到 {} 失败：{error}",
                    source.display(),
                    destination.display()
                )
            });
        }
    }
}
