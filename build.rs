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
    let platform = match target_os.as_str() {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "macos",
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
        "sdk/binaries.tar.xz",
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
        "macos" => [
            format!("lib{MD_API_BASENAME}.dylib"),
            format!("lib{TRADER_API_BASENAME}.dylib"),
        ],
        _ => [
            format!("lib{MD_API_BASENAME}.so"),
            format!("lib{TRADER_API_BASENAME}.so"),
        ],
    };

    // 三平台二进制以 xz 压缩包随 crate 分发（低于 crates.io 10MB 限制），
    // 构建时把当前平台的库解压到 OUT_DIR，不修改源码目录。
    let sdk_archive = root.join("sdk/binaries.tar.xz");
    let extract_dir = out_dir.join("ctp-sdk-libs").join(platform);
    if !extract_dir.join(&runtime_libraries[0]).exists() {
        fs::create_dir_all(&extract_dir)
            .unwrap_or_else(|error| panic!("创建 {} 失败：{error}", extract_dir.display()));
        let file = fs::File::open(&sdk_archive)
            .unwrap_or_else(|error| panic!("打开 {} 失败：{error}", sdk_archive.display()));
        let decoder = xz2::read::XzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        for entry in archive
            .entries()
            .unwrap_or_else(|error| panic!("读取 SDK 压缩包失败：{error}"))
        {
            let mut entry = entry.unwrap_or_else(|error| panic!("读取压缩包条目失败：{error}"));
            let path = entry
                .path()
                .unwrap_or_else(|error| panic!("压缩包条目路径无效：{error}"))
                .to_path_buf();
            if path
                .components()
                .next()
                .is_some_and(|part| part.as_os_str() == platform)
            {
                entry
                    .unpack_in(out_dir.join("ctp-sdk-libs"))
                    .unwrap_or_else(|error| panic!("解压 {} 失败：{error}", path.display()));
            }
        }
    }
    let library_dir = extract_dir;
    println!("cargo:rustc-link-search=native={}", library_dir.display());

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

    if target_os == "windows" {
        println!("cargo:rustc-link-lib={MD_API_BASENAME}");
        println!("cargo:rustc-link-lib={TRADER_API_BASENAME}");
    } else if target_os == "macos" {
        // 官方 dylib 的 install name 与代码签名面向 framework Bundle，
        // 解压后先改为 @rpath 形式并用 ad-hoc 签名替换，
        // 否则 Apple Silicon 的 dyld 会拒绝加载。
        for name in [MD_API_BASENAME, TRADER_API_BASENAME] {
            let dylib = library_dir.join(format!("lib{name}.dylib"));
            for mut command in [
                std::process::Command::new("install_name_tool"),
                std::process::Command::new("codesign"),
            ] {
                if command.get_program() == "install_name_tool" {
                    command.args(["-id", &format!("@rpath/lib{name}.dylib")]);
                } else {
                    command.args(["-f", "-s", "-"]);
                }
                let output = command.arg(&dylib).output().unwrap_or_else(|error| {
                    panic!("运行 {:?} 失败：{error}", command.get_program())
                });
                if !output.status.success() {
                    panic!(
                        "{:?} 处理 {} 失败：{}",
                        command.get_program(),
                        dylib.display(),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        println!("cargo:rustc-link-lib=dylib={MD_API_BASENAME}");
        println!("cargo:rustc-link-lib=dylib={TRADER_API_BASENAME}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", library_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    } else {
        println!("cargo:rustc-link-lib=dylib={MD_API_BASENAME}");
        println!("cargo:rustc-link-lib=dylib={TRADER_API_BASENAME}");
        // Unix 动态加载器默认不搜索可执行文件所在目录，且 CI 中
        // 测试二进制的输出位置不在 deps/，因此对 SDK 库目录和
        // 可执行文件所在目录同时建立 rpath。
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", library_dir.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }

    if target_os == "macos" {
        // macOS 库经 rpath 直接从解压目录加载，无需复制到目标目录。
        return;
    }
    let profile_dir = out_dir.ancestors().nth(3).unwrap_or_else(|| {
        panic!(
            "无法从 OUT_DIR 定位 Cargo profile 目录：{}",
            out_dir.display()
        )
    });
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
