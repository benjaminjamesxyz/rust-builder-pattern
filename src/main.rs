// Comment

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    InProgress,
    Done,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub done: TaskStatus,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
struct TaskBuilder {
    pub title: String,
    pub done: TaskStatus,
    pub description: Option<String>,
}

impl Task {
    fn change_status(&mut self, new_status: TaskStatus) {
        self.done = new_status;
    }

    fn change_title(&mut self, new_tittle: impl Into<String>) {
        self.title = new_tittle.into();
    }

    fn change_description(&mut self, new_description: impl Into<String>) {
        self.description = Some(new_description.into());
    }
}

impl TaskBuilder {
    fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[allow(dead_code)]
    fn status(mut self, status: TaskStatus) -> TaskBuilder {
        self.done = status;
        self
    }

    fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Task {
        Task {
            title: self.title,
            done: self.done,
            description: self.description,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "title: {}
            \rdone: {}
            \rdescription: {}",
            self.title,
            match self.done {
                TaskStatus::InProgress => "In Progress",
                TaskStatus::Done => "Done",
            },
            match &self.description {
                Some(description) => description,
                None => "None",
            }
        )
    }
}

fn main() {
    let mut task = TaskBuilder::default()
        .title("tak1")
        .description("Initial")
        .build();
    println!("{task}");
    task.change_status(TaskStatus::Done);
    task.change_title("Modified title");
    task.change_description("Modified description");
    println!("{task}");
}
