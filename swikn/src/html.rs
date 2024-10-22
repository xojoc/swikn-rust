use super::tools;
use super::tools::InputType;
use super::tools::ParameterType;
use askama::Template;

#[derive(Template)]
#[template(path = "tools.html")]
struct ToolsTemplate {
    tools: Vec<Box<dyn tools::TransformationTool>>,
}

pub fn html_all_tools() -> String {
    let template = ToolsTemplate {
        tools: tools::get_all_tools(),
    };

    // let template = IndexTemplate { tools: &["a"] };
    template.to_string()
}

#[derive(Template)]
#[template(path = "tool.html")]
struct ToolTemplate {
    tool: Box<dyn tools::TransformationTool>,
}

pub fn html_tool(slug: &str) -> String {
    for tool in tools::get_all_tools() {
        if tool.slug() == slug {
            let template = ToolTemplate { tool };
            return template.to_string();
        }
    }
    return "".to_string();
}
