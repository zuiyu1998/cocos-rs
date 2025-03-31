use std::num::NonZero;

use crate::GraphicsContext;

type RunnerFn = Box<dyn FnOnce(App) -> AppExit>;

pub enum AppExit {
    Success,
    Error(NonZero<u8>),
}

impl AppExit {
    pub const fn error() -> Self {
        Self::Error(NonZero::<u8>::MIN)
    }
}

pub struct App {
    pub graphics_context: GraphicsContext,

    pub(crate) runner: RunnerFn,
}

pub fn run_once(_app: App) -> AppExit {
    AppExit::Success
}

impl App {
    pub fn empty() -> Self {
        Self {
            runner: Box::new(run_once),
            graphics_context: GraphicsContext::Uninitialized,
        }
    }

    pub fn set_runner(&mut self, f: impl FnOnce(App) -> AppExit + 'static) -> &mut Self {
        self.runner = Box::new(f);
        self
    }

    pub fn run(&mut self) -> AppExit {
        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }
}
