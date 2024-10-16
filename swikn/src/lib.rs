struct Parameter {
    description: String,
    enabled: bool,
    range: Option<[i32; 2]>,
}

struct Tool {
    name: String,
    parameters: Vec<Parameter>,
}

pub fn pretty_print_json(s: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(s).unwrap();
    serde_json::to_string_pretty(&v).unwrap()
}

pub fn test_library() -> &'static str {
    "Test"
}

pub trait TransformationTool {
    fn name(&self) -> &str;
    fn transform(&self, text: &str) -> String;
}

struct JSONPrettyPrinter {}

impl TransformationTool for JSONPrettyPrinter {
    fn name(&self) -> &str {
        "JSON Pretty Printer"
    }

    fn transform(&self, text: &str) -> String {
        pretty_print_json(text)
    }
}

struct Dummy {}
impl TransformationTool for Dummy {
    fn name(&self) -> &str {
        "Dummy"
    }
    fn transform(&self, text: &str) -> String {
        text.to_string()
    }
}

pub fn get_all_tools() -> Vec<Box<dyn TransformationTool>> {
    let mut tools: Vec<Box<dyn TransformationTool>> = Vec::new();
    tools.push(Box::new(JSONPrettyPrinter {}));
    tools.push(Box::new(Dummy {}));

    tools
}
