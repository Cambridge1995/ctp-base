use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include_dir = root.join("sdk/include");
    let wrapper = root.join("cpp/wrapper.h");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") | ("linux", "x86_64") => {}
        ("macos", "x86_64") | ("macos", "aarch64") => {}
        _ => panic!("当前 CTP SDK 不支持 {target_os} {target_arch}"),
    }
    let platform_dir = match target_os.as_str() {
        "windows" => "sdk/windows",
        "linux" => "sdk/linux",
        "macos" => "sdk/macos",
        other => panic!("当前原生桥接仅支持 Windows、Linux 和 macOS，实际目标系统：{other}"),
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
    let runtime_libraries: [String; 2] = match target_os.as_str() {
        "windows" => [
            format!("{MD_API_BASENAME}.dll"),
            format!("{TRADER_API_BASENAME}.dll"),
        ],
        "macos" => [MD_API_BASENAME.to_string(), TRADER_API_BASENAME.to_string()],
        _ => [
            format!("lib{MD_API_BASENAME}.so"),
            format!("lib{TRADER_API_BASENAME}.so"),
        ],
    };
    for library in &runtime_libraries {
        if target_os == "macos" {
            println!(
                "cargo:rerun-if-changed={platform_dir}/{library}.framework/Versions/A/{library}"
            );
        } else {
            println!("cargo:rerun-if-changed={platform_dir}/{library}");
        }
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
    } else if target_os == "macos" {
        // macOS 官方库必须作为完整 framework Bundle 加载（dyld 按 Bundle
        // 的 CodeResources 校验代码签名），通过 -F/-framework 链接，
        // 运行时经 rpath 定位 sdk/macos 下的 framework。
        println!("cargo:rustc-link-arg=-F{}", library_dir.display());
        println!("cargo:rustc-link-arg=-framework {MD_API_BASENAME}");
        println!("cargo:rustc-link-arg=-framework {TRADER_API_BASENAME}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", library_dir.display());
    } else {
        println!("cargo:rustc-link-lib=dylib={MD_API_BASENAME}");
        println!("cargo:rustc-link-lib=dylib={TRADER_API_BASENAME}");
        // Unix 动态加载器默认不搜索可执行文件所在目录，且 CI 中
        // 测试二进制的输出位置不在 deps/，因此对 SDK 库目录和
        // 可执行文件所在目录同时建立 rpath。
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}/sdk/linux",
            root.display()
        );
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }

    let profile_dir = out_dir.ancestors().nth(3).unwrap_or_else(|| {
        panic!(
            "无法从 OUT_DIR 定位 Cargo profile 目录：{}",
            out_dir.display()
        )
    });
    if target_os == "macos" {
        // macOS 库经 rpath 直接从 sdk/macos 加载，无需复制到目标目录。
        return;
    }
    let runtime_libraries: [String; 2] = match target_os.as_str() {
        "windows" => [
            format!("{MD_API_BASENAME}.dll"),
            format!("{TRADER_API_BASENAME}.dll"),
        ],
        "macos" => [MD_API_BASENAME.to_string(), TRADER_API_BASENAME.to_string()],
        _ => [
            format!("lib{MD_API_BASENAME}.so"),
            format!("lib{TRADER_API_BASENAME}.so"),
        ],
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
