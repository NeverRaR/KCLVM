use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct Command {
    clang_path: String,
    rust_stdlib: String,
    executable_root: String,
}

impl Command {
    pub fn new() -> Self {
        let executable_root = Self::get_executable_root();
        let rust_stdlib = Self::get_rust_stdlib(executable_root.as_str());
        let clang_path = Self::get_clang_path();

        Self {
            clang_path,
            rust_stdlib,
            executable_root,
        }
    }

    pub fn link_libs(&mut self, libs: &[String], lib_path: &str) -> String {
        let lib_suffix = Self::get_lib_suffix();
        let lib_path = if lib_path.is_empty() {
            format!("{}{}", "_a.out", lib_suffix)
        } else if !lib_path.ends_with(&lib_suffix) {
            format!("{}{}", lib_path, lib_suffix)
        } else {
            lib_path.to_string()
        };

        let mut args: Vec<String> = vec![
            "-Wno-override-module".to_string(),
            "-Wno-error=unused-command-line-argument".to_string(),
            "-Wno-unused-command-line-argument".to_string(),
            "-shared".to_string(),
            "-undefined".to_string(),
            "dynamic_lookup".to_string(),
            format!("-Wl,-rpath,{}/lib", self.executable_root),
            format!("-L{}/lib", self.executable_root),
            "-lkclvm_native_shared".to_string(),
            format!("-I{}/include", self.executable_root),
        ];
        let mut bc_files = libs.to_owned();
        args.append(&mut bc_files);
        let mut more_args = vec![
            self.rust_stdlib.clone(),
            "-fPIC".to_string(),
            "-o".to_string(),
            lib_path.to_string(),
        ];
        args.append(&mut more_args);

        std::process::Command::new(self.clang_path.clone())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .args(&args)
            .output()
            .expect("clang failed");

        lib_path
    }

    pub fn run_clang(&mut self, bc_path: &str, lib_path: &str) -> String {
        let mut bc_path = bc_path.to_string();
        let mut lib_path = lib_path.to_string();

        let mut bc_files = vec![];

        for entry in glob::glob(&format!("{}*.ll", bc_path)).unwrap() {
            match entry {
                Ok(path) => {
                    if path.exists() {
                        bc_files.push(path);
                    }
                }
                Err(e) => println!("{:?}", e),
            };
        }
        let mut bc_files = bc_files
            .iter()
            .map(|f| f.to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        if !Self::path_exist(bc_path.as_str()) {
            let s = format!("{}.ll", bc_path);
            if Self::path_exist(s.as_str()) {
                bc_path = s;
            } else {
                let s = format!("{}.ll", bc_path);
                if Self::path_exist(s.as_str()) {
                    bc_path = s;
                }
            }
        }

        if lib_path.is_empty() {
            lib_path = format!("{}{}", bc_path, Self::get_lib_suffix());
        }

        let mut args: Vec<String> = vec![
            "-Wno-override-module".to_string(),
            "-Wno-error=unused-command-line-argument".to_string(),
            "-Wno-unused-command-line-argument".to_string(),
            "-shared".to_string(),
            "-undefined".to_string(),
            "dynamic_lookup".to_string(),
            format!("-Wl,-rpath,{}/lib", self.executable_root),
            format!("-L{}/lib", self.executable_root),
            "-lkclvm_native_shared".to_string(),
            format!("-I{}/include", self.executable_root),
        ];
        args.append(&mut bc_files);
        let mut more_args = vec![
            self.rust_stdlib.clone(),
            "-fPIC".to_string(),
            "-o".to_string(),
            lib_path.to_string(),
        ];
        args.append(&mut more_args);

        std::process::Command::new(self.clang_path.clone())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .args(&args)
            .output()
            .expect("clang failed");

        lib_path
    }

    pub fn run_clang_single(&mut self, bc_path: &str, lib_path: &str) -> String {
        let mut bc_path = bc_path.to_string();
        let mut lib_path = lib_path.to_string();

        if !Self::path_exist(bc_path.as_str()) {
            let s = format!("{}.ll", bc_path);
            if Self::path_exist(s.as_str()) {
                bc_path = s;
            } else {
                let s = format!("{}.ll", bc_path);
                if Self::path_exist(s.as_str()) {
                    bc_path = s;
                }
            }
        }

        if lib_path.is_empty() {
            lib_path = format!("{}{}", bc_path, Self::get_lib_suffix());
        }

        let mut args: Vec<String> = vec![
            "-Wno-override-module".to_string(),
            "-Wno-error=unused-command-line-argument".to_string(),
            "-Wno-unused-command-line-argument".to_string(),
            "-shared".to_string(),
            "-undefined".to_string(),
            "dynamic_lookup".to_string(),
            format!("-Wl,-rpath,{}/lib", self.executable_root),
            format!("-L{}/lib", self.executable_root),
            "-lkclvm_native_shared".to_string(),
            format!("-I{}/include", self.executable_root),
        ];
        let mut bc_files = vec![bc_path];
        args.append(&mut bc_files);
        let mut more_args = vec![
            self.rust_stdlib.clone(),
            "-fPIC".to_string(),
            "-o".to_string(),
            lib_path.to_string(),
        ];
        args.append(&mut more_args);

        let output = std::process::Command::new(self.clang_path.clone())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .args(&args)
            .output()
            .expect("clang failed");

        // Use absolute path.
        let path = PathBuf::from(&lib_path).canonicalize().expect(&format!(
            "{} not found; assembly info: {:?}",
            lib_path, output
        ));
        path.to_str().unwrap().to_string()
    }

    /// Get the kclvm executable root.
    fn get_executable_root() -> String {
        if Self::is_windows() {
            todo!();
        }

        let kclvm_exe = if Self::is_windows() {
            "kclvm.exe"
        } else {
            "kclvm"
        };
        let p = if let Some(x) = Self::find_it(kclvm_exe) {
            x
        } else {
            std::env::current_exe().unwrap()
        };

        let p = p.parent().unwrap().parent().unwrap();
        p.to_str().unwrap().to_string()
    }

    fn get_rust_stdlib(executable_root: &str) -> String {
        let txt_path = std::path::Path::new(&executable_root)
            .join(if Self::is_windows() { "libs" } else { "lib" })
            .join("rust-libstd-name.txt");
        let rust_libstd_name = std::fs::read_to_string(txt_path).expect("rust libstd not found");
        let rust_libstd_name = rust_libstd_name.trim();
        format!("{}/lib/{}", executable_root, rust_libstd_name)
    }

    fn get_clang_path() -> String {
        // ${KCLVM_CLANG}
        let env_kclvm_clang = env::var("KCLVM_CLANG");
        if let Ok(clang_path) = env_kclvm_clang {
            if !clang_path.is_empty() {
                if Self::is_windows() {
                    return format!("{}.exe", clang_path);
                } else {
                    return clang_path;
                }
            }
        }

        // {root}/tools/clang/bin/clang
        let executable_root = Self::get_executable_root();
        let clang_path = std::path::Path::new(&executable_root)
            .join("tools")
            .join("clang")
            .join("bin")
            .join(if Self::is_windows() {
                "clang.exe"
            } else {
                "clang"
            });
        if clang_path.exists() {
            return clang_path.to_str().unwrap().to_string();
        }

        let clang_exe = if Self::is_windows() {
            "clang.exe"
        } else {
            "clang"
        };

        if let Some(s) = Self::find_it(clang_exe) {
            return s.to_str().unwrap().to_string();
        }

        panic!("get_clang_path failed")
    }

    pub fn get_lib_suffix() -> String {
        if Self::is_windows() {
            return ".dll.lib".to_string();
        }
        if Self::is_macos() {
            return ".dylib".to_string();
        }
        if Self::is_linux() {
            return ".so".to_string();
        }
        panic!("unsupport os")
    }

    fn is_windows() -> bool {
        cfg!(target_os = "windows")
    }
    fn is_macos() -> bool {
        cfg!(target_os = "macos")
    }
    fn is_linux() -> bool {
        cfg!(target_os = "linux")
    }

    fn path_exist(path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn find_it<P>(exe_name: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        std::env::var_os("PATH").and_then(|paths| {
            std::env::split_paths(&paths)
                .filter_map(|dir| {
                    let full_path = dir.join(&exe_name);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                })
                .next()
        })
    }
}
