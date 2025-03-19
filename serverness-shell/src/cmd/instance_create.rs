use std::sync::{Arc, Mutex};

use nu_engine::command_prelude::*;
use nu_engine::get_full_help;
use nu_protocol::SyntaxShape;
use serverness::Client;

use crate::state;
use crate::state::State;

#[derive(Clone)]
pub struct InstanceCreate {
    state: Arc<Mutex<State>>,
}

impl InstanceCreate {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for InstanceCreate {
    fn name(&self) -> &str {
        "instance create"
    }

    fn signature(&self) -> Signature {
        Signature::build("instance create")
            .required_named("hostname", SyntaxShape::String, "the machine name", None)
            .required_named("pool", SyntaxShape::String, "pool", None)
            // .input_output_types(vec![(Type::Nothing, Type::List)])
            .category(Category::Custom("serverness".into()))
    }

    fn description(&self) -> &str {
        "create a plan instance"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let guard = self.state.lock().unwrap();

        let results = guard
            .runtime
            .block_on(execute_instance_list(
                engine_state,
                stack,
                call,
                self.state.clone(),
            ))
            .unwrap();

        /* let hostname = call.get_flag::<String>(engine_state, stack, "hostname")?;
        let pool = call.get_flag::<::std::string::String>(engine_state, stack, "pool")?; */

        Ok(Value::list(results, call.head).into_pipeline_data())
    }
}

async fn execute_instance_list<'a>(
    engine_state: &nu_protocol::engine::EngineState,
    stack: &mut nu_protocol::engine::Stack,
    call: &'a nu_protocol::engine::Call<'a>,
    state: Arc<Mutex<State>>,
) -> anyhow::Result<Vec<Value>> {
    let span = call.head;
    let guard = state.lock().unwrap();
    let mut request = guard.client.instance_list();

    let mut stream = futures::StreamExt::take(request.stream(), usize::MAX);
    let mut results = vec![];
    loop {
        match futures::TryStreamExt::try_next(&mut stream).await {
            Err(r) => {
                return Err(anyhow::Error::new(r));
            }

            Ok(None) => {
                return Ok(results);
            }

            Ok(Some(value)) => {
                let val = crate::cmd::to_value(value, span).unwrap();

                results.push(val);
            }
        }
    }
}
