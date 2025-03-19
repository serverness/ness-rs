use clap::Parser;
use newline_converter;
use progenitor::{GenerationSettings, Generator, TagStyle};
use rustfmt_wrapper;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser)]
#[command(name = "xtask")]
enum Xtask {
    #[command()]
    Generate {
        #[clap(long)]
        sdk: bool,

        #[clap(long)]
        httpmock: bool,

        #[clap(long)]
        cli: bool,

        #[clap(long)]
        nu: bool,
    },
}

fn main() -> Result<(), String> {
    let args = Xtask::parse();

    match args {
        Xtask::Generate {
            sdk,
            httpmock,
            cli,
            nu,
        } => generate(sdk, httpmock, cli, nu),
    }
}

fn generate(sdk: bool, httpmock: bool, cli: bool, nu: bool) -> Result<(), String> {
    let xtask_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_path = xtask_path.parent().unwrap().to_path_buf();
    let mut spec_path = root_path.clone();
    spec_path.push("serverness.json");

    let file = File::open(spec_path).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(progenitor::InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate)
            .with_derive("schemars::JsonSchema")
            .with_map_type("indexmap::IndexMap"),
    );

    if sdk {
        std::io::stdout().flush().unwrap();
        let code = generator.generate_tokens(&spec).unwrap();
        let contents = format_code(code.to_string());

        let mut out_path = root_path.clone();
        out_path.push("serverness");
        out_path.push("src");
        out_path.push("generated_sdk.rs");

        std::fs::write(out_path, &contents).unwrap();
    }

    if httpmock {
        std::io::stdout().flush().unwrap();
        let code = generator.httpmock(&spec, "serverness").unwrap().to_string();
        let contents = format_code(code);

        let mut out_path = root_path.clone();
        out_path.push("serverness-httpmock");
        out_path.push("src");
        out_path.push("generated_httpmock.rs");

        std::fs::write(out_path, &contents).unwrap();
    }

    if cli {
        std::io::stdout().flush().unwrap();
        let code = generator.cli(&spec, "serverness").unwrap().to_string();
        let contents = format_code(code);

        let mut out_path = root_path.clone();
        out_path.push("serverness-cli");
        out_path.push("src");
        out_path.push("generated_cli.rs");

        std::fs::write(out_path, &contents).unwrap();
    }

    if nu {
        let code = generator.nu(&spec, "serverness").unwrap().to_string();
        let contents = format_code(code);

        let mut out_path = root_path.clone();
        out_path.push("serverness-shell");
        out_path.push("src");
        out_path.push("generated_nu.rs");

        std::fs::write(out_path, &contents).unwrap();
    }

    Ok(())
}

fn format_code(code: String) -> String {
    let contents = format!(
        "// The contents of this file are generated; do not modify them.\n\n{}",
        code,
    );

    let contents = rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            normalize_doc_attributes: Some(true),
            wrap_comments: Some(true),
            ..Default::default()
        },
        contents,
    )
    .unwrap();

    newline_converter::dos2unix(&contents).to_string()
}
