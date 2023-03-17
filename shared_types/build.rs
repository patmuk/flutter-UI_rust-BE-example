use anyhow::Result;
use core::api::{Effect, Event, ViewModel};
use crux_core::{typegen::TypeGen, Request};
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=../shared");

    let mut gen = TypeGen::new();

    register_types(&mut gen).expect("type registration failed");

    let output_root = PathBuf::from("./generated");

    gen.dart("shared_types", output_root.join("dart"))
        .expect("dart type gen failed");

    gen.swift("shared_types", output_root.join("swift"))
        .expect("swift type gen failed");

    gen.java("com.example.counter.shared_types", output_root.join("java"))
        .expect("java type gen failed");

    gen.typescript("shared_types", output_root.join("typescript"))
        .expect("typescript type gen failed");
}

fn register_types(gen: &mut TypeGen) -> Result<()> {
    gen.register_type::<Request<Effect>>()?;
    gen.register_type::<Effect>()?;
    gen.register_type::<Event>()?;
    gen.register_type::<ViewModel>()?;

    Ok(())
}
