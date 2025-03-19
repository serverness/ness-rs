use crate::generated_sdk::types;
use nu_protocol::{FromValue, Value};

impl FromValue for types::SortMode {
    fn from_value(v: Value) -> Result<Self, nu_protocol::ShellError> {
        let x = v.clone();

        match v {
            Value::String { val, .. } => match types::SortMode::try_from(val) {
                Ok(mode) => Ok(mode),
                Err(e) => Err(nu_protocol::ShellError::CantConvert {
                    to_type: Self::expected_type().to_string(),
                    from_type: x.get_type().to_string(),
                    span: x.span(),
                    help: None,
                }),
            },
            v => Err(nu_protocol::ShellError::CantConvert {
                to_type: Self::expected_type().to_string(),
                from_type: v.get_type().to_string(),
                span: v.span(),
                help: None,
            }),
        }
    }

    fn expected_type() -> nu_protocol::Type {
        nu_protocol::Type::String
    }
}
