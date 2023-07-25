mod module {
    mod correspondence;
    mod detector;
    mod knn_tracker;
    mod parameters_processing;
    pub mod state_machine;
    mod webcamstream;
}

use module::state_machine::StateMachine;

fn main() {
    let mut project = StateMachine::new();
    loop {
        project.working();
    }
}
