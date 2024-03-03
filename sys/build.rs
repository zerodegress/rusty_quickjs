use std::collections::HashMap;

struct Configs {
    lto: bool,
    clang: bool,
    bignum: bool,
    werror: bool,
    win32: bool,
    profile: bool,
    asan: bool,
}

impl Default for Configs {
    fn default() -> Self {
        let config_darwin = std::env::consts::OS == "macos";
        let config_clang = config_darwin;
        Self {
            lto: false,
            clang: config_clang,
            bignum: true,
            werror: false,
            win32: std::env::consts::OS == "windows",
            profile: false,
            asan: false,
        }
    }
}

impl Configs {
    fn cflags(&self) -> Vec<&'static str> {
        let mut cflags = Vec::new();
        if self.clang {
            cflags.extend_from_slice(&[
                "-g",
                "-w",
                // "-Wall",
                // "-Wextra",
                // "-Wno-sign-compare",
                // "-Wno-missing-field-initializers",
                // "-Wundef",
                // "-Wuninitialized",
                // "-Wunused",
                // "-Wno-unused-parameter",
                // "-Wwrite-strings",
                // "-Wchar-subscripts",
            ]);
        } else {
            cflags.extend_from_slice(&[
                "-g",
                "-w",
                // "-Wall",
                // "-Wno-array-bounds",
                // "-Wno-format-truncation",
            ]);
        }
        cflags.extend_from_slice(&["-fwrapv"]);
        if self.werror {
            cflags.extend_from_slice(&["-Werror"]);
        }
        cflags.extend_from_slice(&["-O2"]);
        if self.lto {
            cflags.extend_from_slice(&["-flto"]);
        }
        if self.profile {
            cflags.extend_from_slice(&["-p"])
        }
        if self.asan {
            cflags.extend_from_slice(&["-fsanitize=address", "-fno-omit-frame-pointer"])
        }
        cflags
    }

    fn defines(&self) -> HashMap<&'static str, Option<String>> {
        let mut defines = HashMap::new();
        defines.insert("_GNU_SOURCE", None);
        defines.insert(
            "CONFIG_VERSION",
            Some(
                "\"".to_string()
                    + String::from_utf8(
                        std::fs::read("deps/quickjs/VERSION").expect("Cannot read VERSION."),
                    )
                    .expect("VERSION encoding error.")
                    .trim()
                    + "\"",
            ),
        );
        if self.bignum {
            defines.insert("CONFIG_BIGNUM", None);
        }
        if self.win32 {
            defines.insert("__USE_MINGW_ANSI_STDIO", None);
        }
        defines
    }

    fn ldflags(&self) -> Vec<&'static str> {
        let mut ldflags = Vec::new();
        ldflags.extend_from_slice(&["-g"]);
        if self.lto {
            ldflags.extend_from_slice(&["-flto"]);
        }
        if self.profile {
            ldflags.extend_from_slice(&["-p"]);
        }
        if self.asan {
            ldflags.extend_from_slice(&["-fsanitize=address", "-fno-omit-frame-pointer"])
        }
        ldflags
    }

    fn libs(&self) -> Vec<&'static str> {
        let mut libs = Vec::new();
        libs.extend_from_slice(&["-lm", "-lpthread"]);
        if !self.win32 {
            libs.extend_from_slice(&["-ldl"]);
        }
        libs
    }
}

fn compile(configs: &Configs) {
    let cflags = configs.cflags();
    let ldflags = configs.ldflags();
    let libs = configs.libs();
    let defines = configs.defines();
    let mut cc_builder = cc::Build::new();
    cc_builder.files([
        "deps/quickjs/quickjs.c",
        "deps/quickjs/libregexp.c",
        "deps/quickjs/libunicode.c",
        "deps/quickjs/cutils.c",
        "deps/quickjs/quickjs-libc.c",
        "deps/quickjs/libbf.c",
    ]);
    for cflag in cflags {
        cc_builder.flag_if_supported(cflag);
    }
    for ldflag in ldflags {
        cc_builder.flag_if_supported(ldflag);
    }
    for lib in libs {
        cc_builder.flag(lib);
    }
    for (name, value) in defines {
        if let Some(value) = value {
            cc_builder.define(name, value.as_str());
        } else {
            cc_builder.define(name, None);
        }
    }
    cc_builder.compile("libquickjs.a");
}

fn bindgen(configs: &Configs) {
    let bindings = bindgen::Builder::default()
        .header("deps/quickjs/quickjs.h")
        .header("deps/quickjs/quickjs-libc.h")
        .header("deps/quickjs/cutils.h")
        .generate()
        .expect("Unable to generate bindings.");
    let out_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined."));
    bindings
        .write_to_file(out_path.join("quickjs_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=deps");

    let configs = Configs::default();
    compile(&configs);
    bindgen(&configs);
}
