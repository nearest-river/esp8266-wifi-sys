use bindgen::{
  RustEdition,
  BindgenError
};

use std::{
  fs,
  env,
  error::Error,
  io::{
    self,
    Write
  }
};


static LIB_PATH: &str="./libs/";
static BINDINGS_PATH: &str="./src/bindings.rs";
static WRAPPER_PATH: &str="./include/wrapper.h";
static XTENSA_CARGO_PATH: &str=concat!(env!("RUSTUP_HOME"),"toolchains/esp/bin/cargo");

static ENV_VARS_REMOVE: [&str;2]=["TARGET","TOOLCHAIN"];
static ENV_VARS_REASSIGN: [(&str,&str);1]=[
  ("CARGO",XTENSA_CARGO_PATH),
];

static LIBS: [&str;11]=[
  "clk",
  "core",
  "espnow",
  "gcc",
  "hal",
  "net80211",
  "phy",
  "pp",
  "rtc",
  "smartconfig",
  "ssc",
];

static C_TYPES_PREFIX: &str="crate::c_types";
static CLANG_ARGS: [&str;5]=["-I","./include","-I","./headers/","-w"];

static RAW_SOURCE: &str=r#"#![allow(
  dead_code,
  non_snake_case,
  improper_ctypes,
  non_camel_case_types,
  non_upper_case_globals
)]"#;


fn main()-> Result<(),Box<dyn Error>> {
  init();
  link()?;

  fs::write(BINDINGS_PATH,generate_bindings()?)?;
  Ok(())
}

fn init() {
  println!("cargo:rerun-if-changed=./build.rs");
  println!("cargo:rerun-if-changed={WRAPPER_PATH}");

  ENV_VARS_REMOVE
  .iter()
  .for_each(|key| env::remove_var(key));

  ENV_VARS_REASSIGN
  .iter()
  .for_each(|(key,val)| env::set_var(key,val));
}

fn link()-> io::Result<()> {
  let mut stdout=io::stdout().lock();

  writeln!(stdout,"cargo:rustc-link-search=native={LIB_PATH}")?;

  for lib in &LIBS {
    writeln!(stdout,"cargo:rustc-link-lib=static={lib}")?;
  }

  Ok(())
}


fn generate_bindings()-> Result<String,BindgenError> {
  let bindings=bindgen::builder()
  .clang_args(CLANG_ARGS)
  .ctypes_prefix(C_TYPES_PREFIX)
  .derive_debug(false)
  .header(WRAPPER_PATH)
  .layout_tests(false)
  .raw_line(RAW_SOURCE)
  .use_core()
  .wrap_unsafe_ops(true)
  .rust_edition(RustEdition::Edition2021)
  .generate()?
  .to_string()
  .replacen(r#"unsafe extern "C" {"#,r#"extern "C" {"#,usize::MAX)
  .replacen("u128","f128",usize::MAX);

  Ok(bindings)
}




