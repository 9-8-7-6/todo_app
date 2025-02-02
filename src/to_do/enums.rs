use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING,
}
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            TaskStatus::DONE => "DONE".to_string(),
            TaskStatus::PENDING => "PENDING".to_string(),
        }
    }
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TaskStatus::DONE => {
                write!(f, "DONE")
            }
            TaskStatus::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}
