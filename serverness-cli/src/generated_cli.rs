// The contents of this file are generated; do not modify them.

use serverness::*;
pub struct Cli<T: CliConfig> {
    client: Client,
    config: T,
}
impl<T: CliConfig> Cli<T> {
    pub fn new(client: Client, config: T) -> Self {
        Self { client, config }
    }
    pub fn get_command(cmd: CliCommand) -> ::clap::Command {
        match cmd {
            CliCommand::ApiKeyList => Self::cli_api_key_list(),
            CliCommand::ApiKeyCreate => Self::cli_api_key_create(),
            CliCommand::ApiKeyDescribe => Self::cli_api_key_describe(),
            CliCommand::ApiKeyDestroy => Self::cli_api_key_destroy(),
            CliCommand::InstallList => Self::cli_install_list(),
            CliCommand::InstallCreate => Self::cli_install_create(),
            CliCommand::InstallDescribe => Self::cli_install_describe(),
            CliCommand::InstallDestroy => Self::cli_install_destroy(),
            CliCommand::InstanceList => Self::cli_instance_list(),
            CliCommand::InstanceCreate => Self::cli_instance_create(),
            CliCommand::InstanceDescribe => Self::cli_instance_describe(),
            CliCommand::InstanceDestroy => Self::cli_instance_destroy(),
            CliCommand::PoolList => Self::cli_pool_list(),
            CliCommand::SessionList => Self::cli_session_list(),
            CliCommand::SessionDestroy => Self::cli_session_destroy(),
            CliCommand::SshKeyList => Self::cli_ssh_key_list(),
            CliCommand::SshKeyCreate => Self::cli_ssh_key_create(),
            CliCommand::SshKeyDescribe => Self::cli_ssh_key_describe(),
            CliCommand::SshKeyUpdate => Self::cli_ssh_key_update(),
            CliCommand::SshKeyDestroy => Self::cli_ssh_key_destroy(),
            CliCommand::UserList => Self::cli_user_list(),
            CliCommand::UserCreate => Self::cli_user_create(),
            CliCommand::UserDescribe => Self::cli_user_describe(),
            CliCommand::UserUpdate => Self::cli_user_update(),
            CliCommand::UserDestroy => Self::cli_user_destroy(),
        }
    }
    pub fn cli_api_key_list() -> ::clap::Command {
        ::clap::Command::new("")
    }
    pub fn cli_api_key_create() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("label")
                    .long("label")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_api_key_describe() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_api_key_destroy() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_install_list() -> ::clap::Command {
        ::clap::Command::new("")
    }
    pub fn cli_install_create() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_install_describe() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_install_destroy() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_instance_list() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(::clap::value_parser!(::std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                ::clap::Arg::new("sort")
                    .long("sort")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::SortMode::ByIdAscending.to_string(),
                            types::SortMode::ByIdDescending.to_string(),
                        ]),
                        |s| types::SortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
    }
    pub fn cli_instance_create() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("hostname")
                    .long("hostname")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_instance_describe() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_instance_destroy() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_pool_list() -> ::clap::Command {
        ::clap::Command::new("")
    }
    pub fn cli_session_list() -> ::clap::Command {
        ::clap::Command::new("")
    }
    pub fn cli_session_destroy() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_ssh_key_list() -> ::clap::Command {
        ::clap::Command::new("")
    }
    pub fn cli_ssh_key_create() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("label")
                    .long("label")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("public-key")
                    .long("public-key")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_ssh_key_describe() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_ssh_key_update() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("id")
                    .long("id")
                    .value_parser(::clap::value_parser!(::uuid::Uuid))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_ssh_key_destroy() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_user_list() -> ::clap::Command {
        ::clap::Command::new("")
    }
    pub fn cli_user_create() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("label")
                    .long("label")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("public-key")
                    .long("public-key")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_user_describe() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub fn cli_user_update() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("id")
                    .long("id")
                    .value_parser(::clap::value_parser!(::uuid::Uuid))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }
    pub fn cli_user_destroy() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::uuid::Uuid))
                .required(true),
        )
    }
    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::ApiKeyList => self.execute_api_key_list(matches).await,
            CliCommand::ApiKeyCreate => self.execute_api_key_create(matches).await,
            CliCommand::ApiKeyDescribe => self.execute_api_key_describe(matches).await,
            CliCommand::ApiKeyDestroy => self.execute_api_key_destroy(matches).await,
            CliCommand::InstallList => self.execute_install_list(matches).await,
            CliCommand::InstallCreate => self.execute_install_create(matches).await,
            CliCommand::InstallDescribe => self.execute_install_describe(matches).await,
            CliCommand::InstallDestroy => self.execute_install_destroy(matches).await,
            CliCommand::InstanceList => self.execute_instance_list(matches).await,
            CliCommand::InstanceCreate => self.execute_instance_create(matches).await,
            CliCommand::InstanceDescribe => self.execute_instance_describe(matches).await,
            CliCommand::InstanceDestroy => self.execute_instance_destroy(matches).await,
            CliCommand::PoolList => self.execute_pool_list(matches).await,
            CliCommand::SessionList => self.execute_session_list(matches).await,
            CliCommand::SessionDestroy => self.execute_session_destroy(matches).await,
            CliCommand::SshKeyList => self.execute_ssh_key_list(matches).await,
            CliCommand::SshKeyCreate => self.execute_ssh_key_create(matches).await,
            CliCommand::SshKeyDescribe => self.execute_ssh_key_describe(matches).await,
            CliCommand::SshKeyUpdate => self.execute_ssh_key_update(matches).await,
            CliCommand::SshKeyDestroy => self.execute_ssh_key_destroy(matches).await,
            CliCommand::UserList => self.execute_user_list(matches).await,
            CliCommand::UserCreate => self.execute_user_create(matches).await,
            CliCommand::UserDescribe => self.execute_user_describe(matches).await,
            CliCommand::UserUpdate => self.execute_user_update(matches).await,
            CliCommand::UserDestroy => self.execute_user_destroy(matches).await,
        }
    }
    pub async fn execute_api_key_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.api_key_list();
        self.config.execute_api_key_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_api_key_create(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.api_key_create();
        if let Some(value) = matches.get_one::<::std::string::String>("label") {
            request = request.body_map(|body| body.label(value.clone()))
        }
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ApiKeyCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_api_key_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_api_key_describe(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.api_key_describe();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config
            .execute_api_key_describe(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_api_key_destroy(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.api_key_destroy();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config.execute_api_key_destroy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_install_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.install_list();
        self.config.execute_install_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_install_create(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.install_create();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::InstallCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_install_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_install_describe(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.install_describe();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config
            .execute_install_describe(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_install_destroy(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.install_destroy();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config.execute_install_destroy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_instance_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_list();
        if let Some(value) = matches.get_one::<::std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }
        if let Some(value) = matches.get_one::<types::SortMode>("sort") {
            request = request.sort(value.clone());
        }
        self.config.execute_instance_list(matches, &mut request)?;
        self.config
            .list_start::<types::InstanceListItemResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::InstanceListItemResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }
    pub async fn execute_instance_create(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_create();
        if let Some(value) = matches.get_one::<::std::string::String>("hostname") {
            request = request.body_map(|body| body.hostname(value.clone()))
        }
        if let Some(value) = matches.get_one::<::std::string::String>("pool") {
            request = request.body_map(|body| body.pool(value.clone()))
        }
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_instance_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_instance_describe(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_describe();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config
            .execute_instance_describe(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_instance_destroy(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_destroy();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config
            .execute_instance_destroy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_pool_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.pool_list();
        self.config.execute_pool_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_session_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.session_list();
        self.config.execute_session_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_session_destroy(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.session_destroy();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config.execute_session_destroy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_ssh_key_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ssh_key_list();
        self.config.execute_ssh_key_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_ssh_key_create(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ssh_key_create();
        if let Some(value) = matches.get_one::<::std::string::String>("label") {
            request = request.body_map(|body| body.label(value.clone()))
        }
        if let Some(value) = matches.get_one::<::std::string::String>("public-key") {
            request = request.body_map(|body| body.public_key(value.clone()))
        }
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SshKeyCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_ssh_key_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_ssh_key_describe(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ssh_key_describe();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config
            .execute_ssh_key_describe(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_ssh_key_update(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ssh_key_update();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SshKeyUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_ssh_key_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_ssh_key_destroy(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ssh_key_destroy();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config.execute_ssh_key_destroy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_user_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_list();
        self.config.execute_user_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_user_create(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_create();
        if let Some(value) = matches.get_one::<::std::string::String>("label") {
            request = request.body_map(|body| body.label(value.clone()))
        }
        if let Some(value) = matches.get_one::<::std::string::String>("public-key") {
            request = request.body_map(|body| body.public_key(value.clone()))
        }
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UserCreateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_user_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_user_describe(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_describe();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config.execute_user_describe(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_user_update(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_update();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UserUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }
        self.config.execute_user_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
    pub async fn execute_user_destroy(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_destroy();
        if let Some(value) = matches.get_one::<::uuid::Uuid>("id") {
            request = request.id(value.clone());
        }
        self.config.execute_user_destroy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
}
pub trait CliConfig {
    fn success_item<T>(&self, value: &ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn success_no_item(&self, value: &ResponseValue<()>);
    fn error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_api_key_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ApiKeyList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_api_key_create(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ApiKeyCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_api_key_describe(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ApiKeyDescribe,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_api_key_destroy(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ApiKeyDestroy,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_install_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstallList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_install_create(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstallCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_install_describe(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstallDescribe,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_install_destroy(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstallDestroy,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_instance_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_instance_create(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_instance_describe(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceDescribe,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_instance_destroy(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceDestroy,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_pool_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::PoolList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_session_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SessionList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_session_destroy(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SessionDestroy,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_ssh_key_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SshKeyList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_ssh_key_create(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SshKeyCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_ssh_key_describe(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SshKeyDescribe,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_ssh_key_update(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SshKeyUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_ssh_key_destroy(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::SshKeyDestroy,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_user_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UserList,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_user_create(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UserCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_user_describe(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UserDescribe,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_user_update(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UserUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn execute_user_destroy(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UserDestroy,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    ApiKeyList,
    ApiKeyCreate,
    ApiKeyDescribe,
    ApiKeyDestroy,
    InstallList,
    InstallCreate,
    InstallDescribe,
    InstallDestroy,
    InstanceList,
    InstanceCreate,
    InstanceDescribe,
    InstanceDestroy,
    PoolList,
    SessionList,
    SessionDestroy,
    SshKeyList,
    SshKeyCreate,
    SshKeyDescribe,
    SshKeyUpdate,
    SshKeyDestroy,
    UserList,
    UserCreate,
    UserDescribe,
    UserUpdate,
    UserDestroy,
}
impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::ApiKeyList,
            CliCommand::ApiKeyCreate,
            CliCommand::ApiKeyDescribe,
            CliCommand::ApiKeyDestroy,
            CliCommand::InstallList,
            CliCommand::InstallCreate,
            CliCommand::InstallDescribe,
            CliCommand::InstallDestroy,
            CliCommand::InstanceList,
            CliCommand::InstanceCreate,
            CliCommand::InstanceDescribe,
            CliCommand::InstanceDestroy,
            CliCommand::PoolList,
            CliCommand::SessionList,
            CliCommand::SessionDestroy,
            CliCommand::SshKeyList,
            CliCommand::SshKeyCreate,
            CliCommand::SshKeyDescribe,
            CliCommand::SshKeyUpdate,
            CliCommand::SshKeyDestroy,
            CliCommand::UserList,
            CliCommand::UserCreate,
            CliCommand::UserDescribe,
            CliCommand::UserUpdate,
            CliCommand::UserDestroy,
        ]
        .into_iter()
    }
}
