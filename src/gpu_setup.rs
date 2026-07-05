use std::sync::Arc;

use winit::window::Window;

pub struct GpuSetup {
    instance: wgpu::Instance,
}

impl GpuSetup {
    pub fn init() -> Self {
        Self {
            instance: wgpu::Instance::new(wgpu::InstanceDescriptor {
                // native
                #[cfg(not(target_arch = "wasm32"))]
                backends: wgpu::Backends::PRIMARY,

                // web
                #[cfg(target_arch = "wasm32")]
                backends: wgpu::Backends::GL,

                flags: Default::default(),
                memory_budget_thresholds: Default::default(),
                backend_options: Default::default(),
                display: None,
            }),
        }
    }

    pub fn surface(&self, window: Arc<Window>) -> wgpu::Surface<'static> {
        self.instance.create_surface(window).unwrap()
    }

    pub async fn adapter(&self, surface: &wgpu::Surface<'static>) -> wgpu::Adapter {
        self.instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .unwrap()
    }

    pub async fn device_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap()
    }
}
