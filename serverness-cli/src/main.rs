#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::print_stderr))]

use std::io;
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;

use anyhow::Result;
use async_trait::async_trait;
use cli_builder::NewCli;
use context::Context;
use generated_cli::CliConfig;
use serverness::Client;
use url::Url;

mod cli_builder;
mod context;

#[allow(unused_mut)]
#[allow(unused)] // TODO
#[allow(unused_must_use)] // TODO
#[allow(clippy::clone_on_copy)]
mod generated_cli;
#[macro_use]
mod print;

#[async_trait]
pub trait RunnableCmd: Send + Sync {
    async fn run(&self, ctx: &Context) -> Result<()>;
    fn is_subtree() -> bool {
        true
    }
}

#[async_trait]
pub trait AuthenticatedCmd: Send + Sync {
    async fn run(&self, client: &Client) -> Result<()>;
    fn is_subtree() -> bool {
        true
    }
}

#[async_trait]
impl<T: AuthenticatedCmd> RunnableCmd for T {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let client = Client::new_authenticated_config(ctx.client_config())?;
        self.run(&client).await
    }
    fn is_subtree() -> bool {
        <T as AuthenticatedCmd>::is_subtree()
    }
}

pub fn make_cli() -> NewCli<'static> {
    NewCli::default()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let new_cli = make_cli();

    if let Err(e) = new_cli.run().await {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            if io_err.kind() == io::ErrorKind::BrokenPipe {
                return;
            }
        }

        eprintln_nopipe!("{e:#}");
        std::process::exit(1)
    }
}

#[derive(Default)]
struct ServernessOverride {
    needs_comma: AtomicBool,
}

impl ServernessOverride {}

impl CliConfig for ServernessOverride {
    fn success_item<T>(&self, value: &serverness::ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        let s = serde_json::to_string_pretty(std::ops::Deref::deref(value))
            .expect("failed to serialize return to json");
        println_nopipe!("{}", s);
    }

    fn success_no_item(&self, _: &serverness::ResponseValue<()>) {}

    fn error<T>(&self, _value: &serverness::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln_nopipe!("error");
    }

    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.needs_comma
            .store(false, std::sync::atomic::Ordering::Relaxed);
        print_nopipe!("[");
    }

    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        let s = serde_json::to_string_pretty(&[value]).expect("failed to serialize result to json");
        if self.needs_comma.load(std::sync::atomic::Ordering::Relaxed) {
            print_nopipe!(", {}", &s[4..s.len() - 2]);
        } else {
            print_nopipe!("\n{}", &s[2..s.len() - 2]);
        };
        self.needs_comma
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        if self.needs_comma.load(std::sync::atomic::Ordering::Relaxed) {
            println_nopipe!("\n]");
        } else {
            println_nopipe!("]");
        }
    }

    fn list_end_error<T>(&self, _value: &serverness::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.list_end_success::<T>()
    }
}
