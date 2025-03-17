use std::{any::TypeId, collections::BTreeMap, marker::PhantomData, net::IpAddr, path::PathBuf};

use anyhow::{bail, Result};
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command, CommandFactory, FromArgMatches};
use log::LevelFilter;

use crate::{
    context::Context,
    generated_cli::{Cli, CliCommand},
    RunnableCmd, ServernessOverride,
};
use serverness::ClientConfig;

/// Control an Serverness environment
#[derive(clap::Parser, Debug, Clone)]
#[command(name = "serverness", verbatim_doc_comment)]
struct ServernessCli {
    /// Enable debug output
    #[clap(long)]
    pub debug: bool,

    /// Serverness API host
    #[clap(long, global = true, help_heading = "Global Options")]
    pub host: Option<String>,

    /// Serverness API token
    #[clap(long, global = true, help_heading = "Global Options")]
    pub token: Option<String>,

    /// Modify name resolution
    #[clap(long, value_name = "HOST:PORT:ADDR")]
    pub resolve: Option<ResolveValue>,

    /// Specify a trusted CA cert
    #[clap(long, value_name = "FILE")]
    pub cacert: Option<PathBuf>,

    /// Disable certificate validation and hostname verification
    #[clap(long)]
    pub insecure: bool,

    /// Timeout value for individual API invocations
    #[clap(long, value_name = "SECONDS")]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolveValue {
    pub host: String,
    pub port: u16,
    pub addr: IpAddr,
}

impl std::str::FromStr for ResolveValue {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values = s.splitn(3, ':').collect::<Vec<_>>();
        let [host, port, addr] = values.as_slice() else {
            return Err(r#"value must be "host:port:addr"#.to_string());
        };

        let host = host.to_string();
        let port = port
            .parse()
            .map_err(|_| format!("error parsing port '{}'", port))?;

        // `IpAddr::parse()` does not accept enclosing brackets on IPv6
        // addresses; strip them off if they exist.
        let addr = addr
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(addr);
        let addr = addr
            .parse()
            .map_err(|_| format!("error parsing address '{}'", addr))?;

        Ok(Self { host, port, addr })
    }
}

#[async_trait]
trait RunIt: Send + Sync {
    async fn run_cmd(&self, matches: &ArgMatches, ctx: &Context) -> Result<()>;
    fn is_subtree(&self) -> bool;
}

#[derive(Default)]
struct CommandBuilder<'a> {
    children: BTreeMap<&'a str, CommandBuilder<'a>>,
    cmd: Option<Box<dyn RunIt>>,
    terminal: bool,
}

pub struct NewCli<'a> {
    parser: Command,
    runner: CommandBuilder<'a>,
}

impl<'a> Default for NewCli<'a> {
    fn default() -> Self {
        let mut parser = ServernessCli::command()
            .name("serverness")
            .subcommand_required(true);
        let mut runner = CommandBuilder::default();
        for op in CliCommand::iter() {
            let Some(path) = xxx(op) else { continue };
            runner.add_cmd(path, GeneratedCmd(op));

            let cmd = Cli::<ServernessOverride>::get_command(op);
            let cmd = match op {
                _ => cmd,
            };

            parser = parser.add_subcommand(path, cmd);
            // print_cmd(&parser, 0);
        }

        Self { parser, runner }
    }
}

#[async_trait]
impl<C> RunIt for CustomCmd<C>
where
    C: Send + Sync + FromArgMatches + RunnableCmd,
{
    async fn run_cmd(&self, matches: &ArgMatches, ctx: &Context) -> Result<()> {
        let cmd = C::from_arg_matches(matches).unwrap();
        cmd.run(ctx).await
    }

    fn is_subtree(&self) -> bool {
        <C as RunnableCmd>::is_subtree()
    }
}

impl<'a> NewCli<'a> {
    pub fn add_custom<N>(mut self, path: &'a str) -> Self
    where
        N: Send + Sync + FromArgMatches + RunnableCmd + CommandFactory + 'static,
    {
        self.runner.add_cmd(path, CustomCmd::<N>::new());
        self.parser = self.parser.add_subcommand(path, N::command());

        self
    }

    pub async fn run(self) -> Result<()> {
        let Self { parser, runner } = self;
        let matches = parser.get_matches();

        let ServernessCli {
            host,
            token,
            debug,
            resolve,
            cacert,
            insecure,
            timeout,
        } = ServernessCli::from_arg_matches(&matches)
            .expect("failed to parse ServernessCli from args");

        let mut log_builder = env_logger::builder();
        if debug {
            log_builder.filter_level(LevelFilter::Debug);
        }
        log_builder.init();

        let mut client_config = ClientConfig::default();

        if let Some(token) = token {
            client_config =
                client_config.with_auth(host.unwrap_or("https://api.serverness.io".into()), token)
        }

        if let Some(resolve) = resolve {
            client_config = client_config.with_resolve(resolve.host, resolve.addr);
        }

        if let Some(cacert_path) = cacert {
            let extension = cacert_path
                .extension()
                .map(std::ffi::OsStr::to_ascii_lowercase);
            let contents = std::fs::read(cacert_path)?;
            let cert = match extension.as_ref().and_then(|ex| ex.to_str()) {
                Some("pem") => reqwest::tls::Certificate::from_pem(&contents),
                Some("der") => reqwest::tls::Certificate::from_der(&contents),
                _ => bail!("--cacert path must be a 'pem' or 'der' file".to_string()),
            }?;

            client_config = client_config.with_cert(cert);
        }
        client_config = client_config.with_insecure(insecure);
        if let Some(timeout) = timeout {
            client_config = client_config.with_timeout(timeout);
        }

        let ctx = Context::new(client_config)?;

        let mut node = &runner;
        let mut sm = &matches;
        while let Some((sub_name, sub_matches)) = sm.subcommand() {
            node = node
                .children
                .get(sub_name)
                .expect("child subcommand not found");
            sm = sub_matches;
            if node.terminal {
                break;
            }
        }
        node.cmd
            .as_ref()
            .expect("no cmd for node")
            .run_cmd(sm, &ctx)
            .await
    }

    pub fn command(&self) -> &Command {
        &self.parser
    }

    pub fn command_take(self) -> Command {
        self.parser
    }
}

impl<'a> CommandBuilder<'a> {
    pub fn add_cmd(&mut self, path: &'a str, cmd: impl RunIt + 'static) {
        let mut node = self;
        for component in path.split(' ') {
            node = node.children.entry(component).or_default();
        }
        node.terminal = cmd.is_subtree();
        node.cmd = Some(Box::new(cmd));
    }
}

struct GeneratedCmd(CliCommand);
#[async_trait]
impl RunIt for GeneratedCmd {
    async fn run_cmd(&self, matches: &ArgMatches, ctx: &Context) -> Result<()> {
        let client = serverness::Client::new_authenticated_config(ctx.client_config())?;
        let cli = Cli::new(client, ServernessOverride::default());
        cli.execute(self.0, matches).await
    }

    fn is_subtree(&self) -> bool {
        false
    }
}

struct CustomCmd<C> {
    _cmd: PhantomData<C>,
}

impl<C> CustomCmd<C> {
    pub fn new() -> Self {
        Self { _cmd: PhantomData }
    }
}

fn xxx<'a>(command: CliCommand) -> Option<&'a str> {
    match command {
        CliCommand::InstanceList => Some("instance list"),
        CliCommand::InstanceCreate => Some("instance create"),
        CliCommand::InstanceDescribe => Some("instance describe"),
        CliCommand::InstanceDestroy => Some("instance destroy"),
        CliCommand::ApiKeyCreate => Some("api-key create"),
        CliCommand::ApiKeyDescribe => Some("api-key describe"),
        CliCommand::ApiKeyList => Some("api-key list"),
        CliCommand::ApiKeyDestroy => Some("api-key destroy"),
        CliCommand::InstallList => Some("install list"),
        CliCommand::InstallCreate => Some("install create"),
        CliCommand::InstallDescribe => Some("install describe"),
        CliCommand::InstallDestroy => Some("install destroy"),
        CliCommand::PoolList => Some("pool list"),
        CliCommand::SessionList => Some("session list"),
        CliCommand::SessionDestroy => Some("session destroy"),
        CliCommand::SshKeyList => Some("ssh-key list"),
        CliCommand::SshKeyCreate => Some("ssh-key create"),
        CliCommand::SshKeyDescribe => Some("ssh-key describe"),
        CliCommand::SshKeyUpdate => Some("ssh-key update"),
        CliCommand::SshKeyDestroy => Some("ssh-key destroy"),
        CliCommand::UserList => Some("user list"),
        CliCommand::UserCreate => Some("user create"),
        CliCommand::UserDescribe => Some("user describe"),
        CliCommand::UserUpdate => Some("user update"),
        CliCommand::UserDestroy => Some("user destroy"),
    }
}

trait CommandExt {
    fn add_subcommand(self, path: &str, subcmd: impl Into<Command>) -> Self;
}

impl CommandExt for Command {
    fn add_subcommand(self, path: &str, subcmd: impl Into<Command>) -> Self {
        if let Some(index) = path.find(' ') {
            let first = &path[..index];
            let rest = &path[index + 1..];

            let has_subcommand = self.find_subcommand(first).is_some();

            if has_subcommand {
                self.mut_subcommand(first, |cmd| cmd.add_subcommand(rest, subcmd))
            } else {
                self.subcommand(
                    Command::new(first.to_owned())
                        .subcommand_required(true)
                        .display_order(0)
                        .add_subcommand(rest, subcmd),
                )
            }
        } else {
            let new_subcmd = subcmd.into().name(path.to_owned()).display_order(0);
            let has_subcommand = self.find_subcommand(path).is_some();
            if has_subcommand {
                self.mut_subcommand(path, |cmd| {
                    // Replace the subcommand, but retain its subcommands.
                    new_subcmd.subcommands(cmd.get_subcommands())
                })
            } else {
                self.subcommand(new_subcmd)
            }
        }
    }
}
