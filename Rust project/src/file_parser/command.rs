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

    pub fn get_command(&self) -> &str {
        &self.function
    }
    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }
}
