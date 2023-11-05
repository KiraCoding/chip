#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use chip_interpreter::interpreter::{Interpreter, InterpreterEvent};
use pixels::{Pixels, SurfaceTexture};
use rodio::source::SineWave;
use rodio::{OutputStream, Sink};
use tokio::main;
use winit::dpi::LogicalSize;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

#[main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(500, 250);
        let title = env!("CARGO_PKG_NAME");

        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?
    };

    let size = window.inner_size();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new_async(64, 32, surface_texture).await?
    };

    let mut interpreter = Interpreter::default();

    let program = &[0x60, 0x04, 0xF0, 0x18, 0x00, 0xE0, 0x12, 0x04];

    interpreter.load(program);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::NewEvents(StartCause::Poll) => match interpreter.cycle() {
                InterpreterEvent::Audio => {
                    println!("AUDIO");

                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&stream_handle).unwrap();

                    let source = SineWave::new(440.0);
                    sink.append(source);

                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                interpreter
                    .screen_buffer
                    .iter()
                    .zip(pixels.frame_mut().chunks_exact_mut(4))
                    .for_each(|(&pixel, chunk)| {
                        let color = pixel * 255;
                        chunk.copy_from_slice(&[color, color, color, 255]);
                    });

                if let Err(err) = pixels.render() {
                    eprintln!("pxiels: {}", err);
                    *control_flow = ControlFlow::Exit;
                };
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
