mod app;
mod bus;
mod delta;
mod framebuffer;
mod gpio;
mod panel;
mod render;
mod sysinfo;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
