use std::sync::{Arc, Mutex};

use ::winit::{application::ApplicationHandler, event_loop::EventLoop};
use cocos_core::tracing::error;
use cocos_renderer::{Device, WgpuDevice};
use winit::window::{Window, WindowAttributes};

use crate::app::{App, AppExit};

use super::Executor;

pub struct WinitExecutor {
    event_loop: EventLoop<()>,
}

impl Default for WinitExecutor {
    fn default() -> Self {
        WinitExecutor::new()
    }
}

impl WinitExecutor {
    pub fn new() -> Self {
        WinitExecutor {
            event_loop: EventLoop::new().unwrap(),
        }
    }
}

impl Executor for WinitExecutor {
    fn run(self, mut app: App) {
        let WinitExecutor { event_loop } = self;

        app.set_runner(|app| winit_runner(app, event_loop));
        app.run();
    }
}

pub struct WinitAppRunnerState {
    app: App,
    app_exit: Option<AppExit>,
}

impl WinitAppRunnerState {
    pub fn new(app: App) -> Self {
        WinitAppRunnerState {
            app,
            app_exit: None,
        }
    }
}

pub fn create_window(event_loop: &::winit::event_loop::ActiveEventLoop) -> Window {
    event_loop
        .create_window(WindowAttributes::default())
        .unwrap()
}

pub type FutureRenderResources = Arc<Mutex<Option<Device>>>;

pub fn initialize_graphics_context(
    event_loop: &::winit::event_loop::ActiveEventLoop,
) -> (Device, Arc<Window>) {
    let window = Arc::new(create_window(event_loop));
    let future_render_resources_wrapper: FutureRenderResources = Arc::new(Mutex::new(None));

    let future_render_resources_wrapper_clone = future_render_resources_wrapper.clone();

    let window_clone = window.clone();
    let async_renderer = async move {
        let window = window_clone;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, _queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        let device = Device::new(WgpuDevice { device });

        let mut guard = future_render_resources_wrapper_clone.lock().unwrap();
        *guard = Some(device)
    };

    futures_lite::future::block_on(async_renderer);

    let device = future_render_resources_wrapper
        .lock()
        .unwrap()
        .take()
        .unwrap();

    (device, window)
}

impl ApplicationHandler for WinitAppRunnerState {
    fn resumed(&mut self, event_loop: &::winit::event_loop::ActiveEventLoop) {
        let (device, window) = initialize_graphics_context(event_loop);

        self.app
            .graphics_context
            .initialize_graphics_context(device, window);
    }

    fn window_event(
        &mut self,
        _event_loop: &::winit::event_loop::ActiveEventLoop,
        _window_id: ::winit::window::WindowId,
        _event: ::winit::event::WindowEvent,
    ) {
    }
}

pub fn winit_runner(app: App, event_loop: EventLoop<()>) -> AppExit {
    let mut runner_state = WinitAppRunnerState::new(app);

    if let Err(err) = event_loop.run_app(&mut runner_state) {
        error!("winit event loop returned an error: {err}");
    };

    runner_state.app_exit.unwrap_or_else(|| {
        error!("Failed to receive an app exit code! This is a bug");
        AppExit::error()
    })
}
