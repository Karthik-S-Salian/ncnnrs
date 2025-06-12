# ncnnrs

> The existing [tpoisonooo/rust-ncnn](https://github.com/tpoisonooo/rust-ncnn) library downloads and compiles the entire NCNN library during build time, with fixed build options. This makes cross-compilation difficult. `ncnnrs` was created to solve this issue.

## Features

- Pure Rust bindings for NCNN
- Separate static linking support for cross-compilation

---

## 🧰 Usage Guide

### Step 1: Set NCNN include path

Use your compiled NCNN headers or download precompiled NCNN from [Tencent/ncnn releases](https://github.com/Tencent/ncnn/releases):

```bash
export NCNN_INCLUDE_DIR=/path/to/ncnn/include/ncnn
```

### Step 2: Configure build.rs to link the NCNN libraries
```rust
println!("cargo:rustc-link-lib=dylib=c++");
println!("cargo:rustc-link-search=native=/path/to/ncnn/lib");
println!("cargo:rustc-link-lib=static=ncnn");
// println!("cargo:rustc-link-lib=static=GenericCodeGen");
// println!("cargo:rustc-link-lib=static=glslang");
println!("cargo:rustc-link-lib=static=MachineIndependent"); // Required
println!("cargo:rustc-link-lib=static=OSDependent");        // Required
println!("cargo:rustc-link-lib=static=SPIRV");              // Required

// If Vulkan is enabled, add Vulkan-related libraries here
// println!("cargo:rustc-link-lib=static=ncnn");  // Static link: libncnn.a / ncnn.lib
// println!("cargo:rustc-link-lib=dylib=ncnn");   // Dynamic link: libncnn.dylib / ncnn.dll
```

### Step 3: Build your project
```bash
cargo add ncnnrs
cargo run
```

### 📸 Demo
```bash
cd demo/get_version
cargo run
# Example output:
# build size: 295.73kb mac arm64
# Out: ncnn version: 1.0.20240727
```

### 🔧 Common Example
```rust
use ncnnrs::{Mat, Net, Option};

fn main() {
    let mut opt = Option::new();
    opt.set_num_threads(4);
    opt.set_vulkan_compute(true);

    let mut net = Net::new();
    net.set_option(&opt); // Set runtime options

    net.load_param("xxx.param"); // Load model param file
    net.load_model("xxx.bin");   // Load model weights

    let mut in0 = Mat::new_3d(224, 224, 3, None);
    let mut out = Mat::new();

    let mut ex = net.create_extractor();
    ex.input("in0", &mut in0);
    ex.extract("out0", &mut out);

    println!("{:?}", out);
}
```
> For more examples, refer to tpoisonooo/rust-ncnn
### 🌍 Cross-Platform Support
ncnnrs only depends on NCNN header files during compilation and does not bundle NCNN libs. This makes it easy to link platform-specific binaries for cross-compilation. Just link the appropriate libraries in build.rs.


### 🚫 CPU-only Inference
By default, Vulkan is enabled. To run on CPU-only, enable the cpu feature:
```toml
# Globally enable CPU-only
ncnnrs = { version = "*", features = ["cpu"] }

# OR only for specific targets (e.g., Linux on ARM64)
[target.'cfg(all(target_os = "linux", target_arch = "aarch64"))'.dependencies]
ncnnrs = { version = "*", features = ["cpu"] }
```

### 📚 References
* [tpoisonooo/rust-ncnn](https://github.com/tpoisonooo/rust-ncnn)