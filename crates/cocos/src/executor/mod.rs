use crate::app::App;

pub mod winit;

pub use winit::*;

pub trait Executor {
    fn run(self, app: App);
}

pub fn run_executor(executor: impl Executor, app: App) {
    executor.run(app);
}
