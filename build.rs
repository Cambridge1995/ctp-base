use std::{env, fs, path::PathBuf, process::Command};

/// 当前平台的库文件名。
fn library_file(base: &str, os: &str) -> String {
    match os {
        "windows" => format!("{base}.dll"),
        "macos" => format!("lib{base}.dylib"),
        _ => format!("lib{base}.so"),
    }
}

/// 目录中是否存在当前平台的 CTP 库（不区分商密/非商密）。
fn dir_has_libs(dir: &std::path::Path, os: &str) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    entries.filter_map(|entry| entry.ok()).any(|entry| {
        let name = entry.file_name().to_string_lossy().to_string();
        let extension_matches = match os {
            "windows" => name.ends_with(".dll"),
            "macos" => name.ends_with(".dylib"),
            _ => name.ends_with(".so"),
        };
        extension_matches && name.contains("userapi")
    })
}

/// 解析实际使用的库基名：优先非商密（se），不存在时降级匹配 sm 等其他变体，
/// 使使用者只需放入对应版本的库文件，无需修改 build.rs。
fn resolve_basename(
    dir: &std::path::Path,
    preferred: &str,
    family: &str,
    os: &str,
) -> Option<String> {
    if dir.join(library_file(preferred, os)).exists() {
        return Some(preferred.to_string());
    }
    let mut found: Vec<String> = fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            match os {
                "windows" => {
                    if !name.ends_with(".dll") || !name.starts_with(&format!("{family}_")) {
                        return None;
                    }
                    Some(name.trim_end_matches(".dll").to_string())
                }
                "macos" => {
                    if !name.ends_with(".dylib") || !name.starts_with(&format!("lib{family}_")) {
                        return None;
                    }
                    Some(
                        name.trim_end_matches(".dylib")
                            .trim_start_matches("lib")
                            .to_string(),
                    )
                }
                _ => {
                    if !name.ends_with(".so") || !name.starts_with(&format!("lib{family}_")) {
                        return None;
                    }
                    Some(
                        name.trim_end_matches(".so")
                            .trim_start_matches("lib")
                            .to_string(),
                    )
                }
            }
        })
        .collect();
    found.sort();
    found.into_iter().next()
}

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

    // 库来源优先级：
    // 1. 源码目录 sdk/<平台>/（使用者自行放入的库，se/sm 均可）；
    // 2. target 下的持久缓存（从 sdk/binaries.tar.xz 解压，存在即复用）。
    // 缓存不放在源码目录（会破坏 cargo package 校验），也不放 OUT_DIR
    //（其哈希目录会随依赖指纹变化而重建）。
    let manifest_libs = root.join("sdk").join(platform);
    let target_root = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        })
        .unwrap_or_else(|| root.join("target"));
    let cache_libs = target_root.join("ctp-sdk-libs").join(platform);
    let sdk_archive = root.join("sdk/binaries.tar.xz");

    let library_dir = if dir_has_libs(&manifest_libs, &target_os) {
        println!("cargo:rerun-if-changed={}", manifest_libs.display());
        manifest_libs
    } else {
        // 缓存指纹：SDK 压缩包变化（升级）时自动重新解压。
        let fingerprint = {
            use std::hash::{Hash, Hasher};
            let data = fs::read(&sdk_archive)
                .unwrap_or_else(|error| panic!("读取 {} 失败：{error}", sdk_archive.display()));
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            data.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        };
        let stamp_file = target_root.join("ctp-sdk-libs").join("fingerprint");
        let cached = dir_has_libs(&cache_libs, &target_os)
            && fs::read_to_string(&stamp_file).is_ok_and(|stamp| stamp.trim() == fingerprint);
        if !cached {
            let _ = fs::remove_dir_all(target_root.join("ctp-sdk-libs"));
            fs::create_dir_all(&cache_libs)
                .unwrap_or_else(|error| panic!("创建缓存目录失败：{error}"));
            let status = Command::new("tar")
                .arg("-xJf")
                .arg(&sdk_archive)
                .arg("-C")
                .arg(target_root.join("ctp-sdk-libs"))
                .arg(platform)
                .status()
                .unwrap_or_else(|error| {
                    panic!(
                        "无法运行 tar 解压 {}：{error}。请确认系统自带 tar（Windows 10 及以上、macOS、Linux 均内置）",
                        sdk_archive.display()
                    )
                });
            if !status.success() {
                panic!("tar 解压 {} 失败（{status}）", sdk_archive.display());
            }
            fs::write(
                target_root.join("ctp-sdk-libs").join("fingerprint"),
                &fingerprint,
            )
            .unwrap_or_else(|error| panic!("写入缓存指纹失败：{error}"));
        }
        cache_libs
    };

    let md_base = resolve_basename(&library_dir, MD_API_BASENAME, "thostmduserapi", &target_os)
        .unwrap_or_else(|| {
            panic!(
                "在 {} 中未找到行情库（thostmduserapi_se 或 sm 版本）",
                library_dir.display()
            )
        });
    let trader_base = resolve_basename(
        &library_dir,
        TRADER_API_BASENAME,
        "thosttraderapi",
        &target_os,
    )
    .unwrap_or_else(|| {
        panic!(
            "在 {} 中未找到交易库（thosttraderapi_se 或 sm 版本）",
            library_dir.display()
        )
    });
    println!("cargo:rustc-link-search=native={}", library_dir.display());

    if target_os == "windows" {
        let mut header = String::from("#pragma once\n\n");
        for (name, base) in [
            ("CTP_BASE_MD_DLL_PATH", &md_base),
            ("CTP_BASE_TRADER_DLL_PATH", &trader_base),
        ] {
            let path = library_dir.join(library_file(base, &target_os));
            let path = path
                .to_string_lossy()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            header.push_str(&format!("#define {name} L\"{path}\"\n"));
        }
        fs::write(out_dir.join("ctp_sdk_libraries.h"), header)
            .unwrap_or_else(|error| panic!("写入 Windows CTP 库路径失败：{error}"));
    }

    if target_os == "macos" {
        // 官方 dylib 的 install name 与代码签名面向 framework Bundle，
        // 处理后改为 @rpath 形式并用 ad-hoc 签名替换，否则 Apple Silicon
        // 的 dyld 会拒绝加载。重复执行是幂等的。
        for base in [&md_base, &trader_base] {
            let dylib = library_dir.join(library_file(base, "macos"));
            for (program, args) in [
                (
                    "install_name_tool",
                    vec!["-id".to_string(), format!("@rpath/lib{base}.dylib")],
                ),
                (
                    "codesign",
                    vec!["-f".to_string(), "-s".to_string(), "-".to_string()],
                ),
            ] {
                let output = Command::new(program)
                    .args(&args)
                    .arg(&dylib)
                    .output()
                    .unwrap_or_else(|error| panic!("运行 {program} 失败：{error}"));
                if !output.status.success() {
                    panic!(
                        "{program} 处理 {} 失败：{}",
                        dylib.display(),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
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
        .include(&out_dir)
        .files(["cpp/md_bridge.cpp", "cpp/trader_bridge.cpp"])
        .warnings(false)
        .extra_warnings(false)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/std:c++17")
        .flag_if_supported("/utf-8")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-fPIC")
        .compile("ctp_native_bridge");

    if target_os != "windows" {
        println!("cargo:rustc-link-lib=dylib={md_base}");
        println!("cargo:rustc-link-lib=dylib={trader_base}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", library_dir.display());
        if target_os == "macos" {
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        } else {
            // Unix 动态加载器默认不搜索可执行文件所在目录，且 CI 中
            // 测试二进制的输出位置不在 deps/，因此同时建立 $ORIGIN rpath。
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
    }

    if target_os == "macos" {
        // macOS 库经 rpath 直接从库目录加载，无需复制到目标目录。
        return;
    }
    let profile_dir = out_dir.ancestors().nth(3).unwrap_or_else(|| {
        panic!(
            "无法从 OUT_DIR 定位 Cargo profile 目录：{}",
            out_dir.display()
        )
    });
    for base in [&md_base, &trader_base] {
        let library = library_file(base, &target_os);
        let source = library_dir.join(&library);
        for destination_dir in [profile_dir.to_path_buf(), profile_dir.join("deps")] {
            fs::create_dir_all(&destination_dir).unwrap_or_else(|error| {
                panic!("创建目录 {} 失败：{error}", destination_dir.display())
            });
            let destination = destination_dir.join(&library);
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
