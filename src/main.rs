use prism::App;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();

    // chon hich surface-i nadarim va adaptor gpu ro amade nakardim chizi neshoon nemidee
    event_loop.run_app(&mut app).unwrap();
}
