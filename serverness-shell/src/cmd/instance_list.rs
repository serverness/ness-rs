use std::sync::{Arc, Mutex};

use nu_engine::command_prelude::*;
use nu_engine::get_full_help;
use serverness::Client;

use crate::state::State;

#[derive(Clone)]
pub struct Instance {
    state: Arc<Mutex<State>>,
}

impl Instance {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for Instance {
    fn name(&self) -> &str {
        "instance"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("instance")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Custom("serverness".into()))
    }

    fn description(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce this help message."
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(Value::string(get_full_help(self, engine_state, stack), call.head).into_pipeline_data())
    }
}

#[derive(Clone)]
pub struct InstanceList {
    state: Arc<Mutex<State>>,
}

impl InstanceList {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for InstanceList {
    fn name(&self) -> &str {
        "instance list"
    }

    fn signature(&self) -> Signature {
        Signature::build("instance list")
            // .input_output_types(vec![(Type::Nothing, Type::List)])
            .category(Category::Custom("serverness".into()))
    }

    fn description(&self) -> &str {
        "list active instances"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        let span = call.head;
        let guard = self.state.lock().unwrap();

        let results = guard
            .runtime
            .block_on(execute_instance_list(&guard.client, span))
            .unwrap();

        Ok(Value::list(results, span).into_pipeline_data())
    }
}

async fn execute_instance_list(client: &Client, span: Span) -> anyhow::Result<Vec<Value>> {
    let mut request = client.instance_list();

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
                let val = to_value(value, span).unwrap();

                results.push(val);
            }
        }
    }
}

use serde::Serialize;

/// Convert any serde:Serialize into a `nu_protocol::Value`
pub fn to_value<T>(value: T, span: Span) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(&ValueSerializer { span })
}

struct ValueSerializer {
    span: Span,
}

struct MapSerializer<'a> {
    record: Record,
    serializer: &'a ValueSerializer,
    current_key: Option<String>,
}

impl<'a> serde::Serializer for &'a ValueSerializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;

    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = MapSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::bool(v, self.span))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v.into(), self.span))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v.into(), self.span))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v.into(), self.span))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v, self.span))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v.into(), self.span))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v.into(), self.span))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::int(v.into(), self.span))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        // TODO: how to represent a u64 value a Value<i64>?
        Err(Error::new("the numbers are too big"))
        // Ok(Value::int(v.into(), self.span))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::float(v.into(), self.span))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::float(v, self.span))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::string(v, self.span))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::string(v, self.span))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::binary(v, self.span))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::nothing(self.span))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // TODO: is this OK?
        Ok(Value::nothing(self.span))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        // TODO: is this OK?
        Ok(Value::nothing(self.span))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        // TODO: is this OK?
        Ok(Value::nothing(self.span))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(MapSerializer::new(self))
    }
}

pub struct Error {
    message: String,
}

impl Error {
    pub fn new<T: std::fmt::Display>(msg: T) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::new(msg.to_string())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

//
// maps
impl<'a> MapSerializer<'a> {
    fn new(serializer: &'a ValueSerializer) -> Self {
        Self {
            record: Record::new(),
            current_key: None,
            serializer,
        }
    }
}

impl<'a> serde::ser::SerializeStruct for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.record
            .insert(key.to_owned(), value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::record(self.record, self.serializer.span))
    }
}

impl<'a> serde::ser::SerializeMap for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let value = serde_json::to_value(key).map_err(Error::new)?;
        let key = value
            .as_str()
            .ok_or(Error::new("key must be a string"))?
            .to_string();
        self.current_key = Some(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let key = self.current_key.take().ok_or(Error::new("key expected"))?;
        self.record.insert(key, value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::record(self.record, self.serializer.span))
    }
}

impl<'a> serde::ser::SerializeStructVariant for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.record
            .insert(key.to_owned(), value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::record(self.record, self.serializer.span))
    }
}

//
// sequences
struct SeqSerializer<'a> {
    seq: Vec<Value>,
    serializer: &'a ValueSerializer,
}

impl<'a> SeqSerializer<'a> {
    fn new(serializer: &'a ValueSerializer) -> Self {
        Self {
            seq: Vec::new(),
            serializer,
        }
    }
}

impl<'a> serde::ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.seq.push(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::list(self.seq, self.serializer.span))
    }
}

impl<'a> serde::ser::SerializeTuple for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.seq.push(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::list(self.seq, self.serializer.span))
    }
}

impl<'a> serde::ser::SerializeTupleStruct for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.seq.push(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::list(self.seq, self.serializer.span))
    }
}

impl<'a> serde::ser::SerializeTupleVariant for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.seq.push(value.serialize(self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::list(self.seq, self.serializer.span))
    }
}
