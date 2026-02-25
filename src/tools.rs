use serde::Serialize;

// pub enum ToolKind {
//     READ,
//     WRITE,
//     BASH
// }

#[derive(Serialize)]
pub struct Tool {
    pub name : String,
    pub description: String,
    pub parameters: serde_json::Value
}

pub fn all_tools() -> Vec<serde_json::Value> {
    
    let read_function: Tool = Tool { 
        name: "Read".to_string(),
        description: "Read and return the contents of a file".to_string(), 
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The path to the file to read"
                }
            },
            "required": ["file_path"]
        })
    };
    
    vec![
        serde_json::json!({
            "type": "function",
            "function": read_function
        })
    ]
}