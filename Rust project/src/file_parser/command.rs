#[derive(Debug)]
pub struct Command {
    function: String,
    parameters: Vec<String>,
}

impl Command {
    pub fn new(function: String, parameters: Vec<String>) -> Command {
        Command {
            function,
            parameters,
        }
    }
}