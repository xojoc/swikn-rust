use std::{
    cmp::{max, min},
    fmt::{self, Display},
};

use serde::ser::Serialize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ParameterType {
    Range,
}

impl Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct ParameterConf {
    pub parameter_type: ParameterType,
    pub name: String,
    pub label: String,
    pub range_min: i64,
    pub range_max: i64,
    pub range_step: i64,
    pub integer_value: i64,
    pub string_value: String,
    pub bool_value: bool,
}

pub struct Parameters {
    pub parameters: Vec<ParameterConf>,
}

impl Parameters {
    fn get_bool(&self, k: &str, def: bool) -> bool {
        for o in &self.parameters {
            if o.name == k {
                return o.bool_value;
            }
        }
        return def;
    }
    pub fn set_parse_string(&mut self, k: &str, v: &str) {
        for o in &mut self.parameters {
            if o.name != k {
                continue;
            }
            match o.parameter_type {
                ParameterType::Range => {
                    o.integer_value = v.parse().unwrap();
                }
            }
        }
    }
    fn set_integer(&mut self, k: &str, v: i64) {
        for o in &mut self.parameters {
            if o.name == k {
                o.integer_value = v;
            }
        }
    }
    fn get_integer(&self, k: &str, def: i64) -> i64 {
        for o in &self.parameters {
            if o.name == k {
                return o.integer_value;
            }
        }
        return def;
    }
    fn get_string(&self, k: &str, def: String) -> String {
        for o in &self.parameters {
            if o.name == k {
                return o.string_value.clone();
            }
        }
        return def;
    }
}

pub fn test_library() -> &'static str {
    "Test"
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InputType {
    TextArea,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum InnerError {
    SerdeJson(serde_json::Error),
}

pub struct TransformationError {
    inner_error: InnerError,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for TransformationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner_error {
            InnerError::SerdeJson(ref err) => err.fmt(f),
        }
    }
}

pub type TransformationResult<T> = Result<T, TransformationError>;

pub trait TransformationTool {
    fn name(self: &Self) -> &str;
    fn slug(self: &Self) -> &str;
    fn input_type(&self) -> InputType;
    fn get_parameters(&self) -> &Parameters;
    fn get_mut_parameters(&mut self) -> &mut Parameters;

    fn transform(&mut self, text: &str) -> TransformationResult<String>;
}

struct JSONPrettyPrinter {
    parameters: Parameters,
}

impl JSONPrettyPrinter {
    fn new() -> Self {
        let parameters = vec![ParameterConf {
            parameter_type: ParameterType::Range,
            name: "indent".to_string(),
            label: "indent".to_string(),
            string_value: "".to_string(),
            integer_value: 4,
            range_min: 1,
            range_max: 64,
            range_step: 1,
            bool_value: false,
        }];
        Self {
            parameters: Parameters { parameters },
        }
    }
}

impl TransformationTool for JSONPrettyPrinter {
    fn name(&self) -> &str {
        "JSON Pretty Printer"
    }

    fn slug(&self) -> &str {
        "json"
    }

    fn transform(&mut self, text: &str) -> TransformationResult<String> {
        let parameters = self.get_parameters();
        let indent = parameters.get_integer("indent", 60);
        let indent = max(min(indent, 64), 1) as usize;

        let v: serde_json::Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(err) => {
                return Err(TransformationError {
                    line: err.line(),
                    column: err.column(),
                    inner_error: InnerError::SerdeJson(err),
                });
            }
        };

        // let v: serde_json::Value = serde_json::from_str(text).unwrap();
        let mut buf = Vec::with_capacity(text.len());
        let indent_b = &vec![b' '; indent];
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_b);
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);

        v.serialize(&mut ser).unwrap();
        Ok(String::from_utf8(buf).unwrap())
    }

    fn get_parameters(&self) -> &Parameters {
        &self.parameters
    }

    fn get_mut_parameters(&mut self) -> &mut Parameters {
        &mut self.parameters
    }

    fn input_type(&self) -> InputType {
        InputType::TextArea
    }
}

pub fn get_all_tools() -> Vec<Box<dyn TransformationTool>> {
    let mut tools: Vec<Box<dyn TransformationTool>> = Vec::new();
    tools.push(Box::new(JSONPrettyPrinter::new()));

    tools
}
