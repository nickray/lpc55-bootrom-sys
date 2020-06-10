use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();

    let builder = builder
        .flag("-std=c11")
        .file("lpc55-sdk-2.7.1/devices/LPC55S69/drivers/fsl_iap.c")
        .file("lpc55-sdk-2.7.1/devices/LPC55S69/utilities/fsl_assert.c")
        // need to pick one
        .define("CPU_LPC55S69JBD100_cm33_core0", None)
        // need to pick one
        .define("SERIAL_PORT_TYPE_SWO", None)
        // to avoid pulling in all sorts of dependencies
        .define("NDEBUG", None)
        .include("lpc55-sdk-2.7.1/devices/LPC55S69")
        .include("lpc55-sdk-2.7.1/devices/LPC55S69/drivers")
        .include("lpc55-sdk-2.7.1/devices/LPC55S69/utilities/debug_console")
        .include("lpc55-sdk-2.7.1/components/serial_manager")
        .include("lpc55-sdk-2.7.1/CMSIS/Include")
    ;

    // panic!("{:?}", builder.get_compiler());
    builder.compile("lpc55-bootrom-sys");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("lpc55-sdk-2.7.1/devices/LPC55S69/drivers/fsl_iap.h")
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .clang_arg("-DCPU_LPC55S69JBD100_cm33_core0")
        .clang_arg("-Ilpc55-sdk-2.7.1/devices/LPC55S69")
        .clang_arg("-Ilpc55-sdk-2.7.1/devices/LPC55S69/drivers")
        .clang_arg("-Ilpc55-sdk-2.7.1/CMSIS/Include")

        .generate()
        .expect("Unable to generate bindings");

    let out_file = out_dir.join("bindings.rs");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");

    Ok(())
}
