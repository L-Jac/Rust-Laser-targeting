use class::state_machine;

mod class {
    pub mod correspondence;
    pub mod detector;
    pub mod knn_tracker;
    pub mod parameters_processing;
    pub mod state_machine;
    pub mod webcamstream;
}

fn main() {
    let mut project = state_machine::StateMachine::new();
    loop {
        project.working();
    }
}
