// The contents of this file are generated; do not modify them.

use serverness::*;
struct InstanceList {}
impl Command for InstanceList {
    fn name(&self) -> &str {
        stringify!(nu_instance_list)
    }
    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("#nu_fns")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Custom("serverness".into()))
    }
    fn description(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce \
         this help message."
    }
    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        execute_instance_list(engine_state, stack, call, input)
    }
}
pub async fn execute_instance_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
    let mut request = self.client.instance_list();
    if let Some(value) = matches.get_one::<::std::num::NonZeroU32>("limit") {
        request = request.limit(value.clone());
    }
    if let Some(value) = matches.get_one::<types::ExSortMode>("sort") {
        request = request.sort(value.clone());
    }
    self.config.execute_instance_list(matches, &mut request)?;
    self.config.list_start::<types::InstanceResultsPage>();
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
                self.config.list_end_success::<types::InstanceResultsPage>();
                return Ok(());
            }
            Ok(Some(value)) => {
                self.config.list_item(&value);
            }
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum NuCommand {
    InstanceList,
}
impl NuCommand {
    pub fn iter() -> impl Iterator<Item = NuCommand> {
        vec![NuCommand::InstanceList].into_iter()
    }
}
