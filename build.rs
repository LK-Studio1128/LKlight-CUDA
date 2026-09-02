// build.rs — compiles src/cuda/far_field.cu with nvcc when the `cuda` feature
// is enabled. Produces a static archive consumed by cargo:
//   target/cuda/libfarfield.a   (linked as `static=farfield`)
// and links the CUDA runtime (cudart) found next to nvcc.
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/cuda/far_field.cu");
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_FEATURE_CUDA").is_err() {
        // Plain CPU build — nothing to do.
        return;
    }

    // Locate nvcc (respect NVCC env override).
    let nvcc = std::env::var("NVCC").unwrap_or_else(|_| "nvcc".to_string());

    // out dir for the object + archive
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    std::fs::create_dir_all(&out_dir).unwrap();
    let obj = out_dir.join("far_field.o");
    let lib = out_dir.join("libfarfield.a");

    // Compile the kernel.
    let status = Command::new(&nvcc)
        .args(["-O3", "-arch=native", "-c"])
        .arg("src/cuda/far_field.cu")
        .arg("-o")
        .arg(&obj)
        .status()
        .expect("failed to spawn nvcc — is the CUDA toolkit installed?");
    if !status.success() {
        panic!("nvcc failed to compile src/cuda/far_field.cu (status {status})");
    }

    // Archive it (ar, as used by the toolchain).
    let ar = std::env::var("AR").unwrap_or_else(|_| "ar".to_string());
    let status = Command::new(&ar)
        .args(["crs"])
        .arg(&lib)
        .arg(&obj)
        .status()
        .expect("failed to spawn ar");
    if !status.success() {
        panic!("ar failed to create {lib:?}");
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=farfield");
    // nvcc emits C++-flavoured objects (device stubs use __cxa_guard /
    // __gxx_personality) → link the C++ runtime too.
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // cudart lives next to nvcc (nvcc/../lib64 or lib).
    if let Ok(nvcc_path) = Command::new(&nvcc).arg("--version").status() {
        let _ = nvcc_path;
    }
    // Try `which nvcc` dir.
    let which = Command::new("which").arg(&nvcc).output();
    if let Ok(out) = which {
        if out.status.success() {
            let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let bin = PathBuf::from(&p);
            if let Some(parent) = bin.parent() {
                if let Some(toolchain) = parent.parent() {
                    for cand in ["lib64", "lib"] {
                        let libdir = toolchain.join(cand);
                        if libdir.exists() {
                            println!("cargo:rustc-link-search=native={}", libdir.display());
                        }
                    }
                }
            }
        }
    }
    println!("cargo:rustc-link-lib=dylib=cudart");
}
