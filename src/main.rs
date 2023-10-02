mod convert_image;

use ::anyhow::Result;
use ::crossterm::execute;
use ::crossterm::queue;
use ::crossterm::cursor::Hide;
use ::crossterm::cursor::MoveTo;
use ::crossterm::cursor::MoveToNextLine;
use ::crossterm::cursor::Show;
use ::crossterm::event::poll as poll_terminal;
use ::crossterm::event::read as read_event;
use ::crossterm::event::Event;
use ::crossterm::style::Color;
use ::crossterm::style::PrintStyledContent;
use ::crossterm::style::Stylize;
use ::crossterm::terminal::EnterAlternateScreen;
use ::crossterm::terminal::LeaveAlternateScreen;
use ::crossterm::terminal::size as terminal_size;
use ::image::GenericImageView;
use ::rscam::Camera;
use ::rscam::Config;
use ::std::io::Write;
use ::std::io::stdout;
use ::std::time::Duration;
use convert_image::*;

const BLACK: Color = Color::Rgb { r: 0, g: 0, b: 0 };

struct Context { _a: () }

impl Context {
    pub fn new() -> Self {
        let _ = execute!(stdout(), EnterAlternateScreen);
        let _ = execute!(stdout(), Hide);
        Context { _a: () }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = execute!(stdout(), Show);
    }
}

fn main() -> Result<()> {
    let mut camera = Camera::new("/dev/video0").unwrap();
    camera.start(&Config {
        interval: (1, 30),
        resolution: (1280, 720),
        format: b"MJPG",
        .. Config::default()
    })?;
    let mut stdout = stdout();
    let (width, height) = terminal_size()?;
    let (mut width, mut height) = (width as u32, height as u32);
    let _context = Context::new();
    loop {
        while poll_terminal(Duration::new(0, 0))? {
            if let Event::Resize(new_width, new_height) = read_event()? {
                width = new_width as u32;
                height = new_height as u32;
            }
        }
        let frame = camera.capture()?;
        let img = ::image::load_from_memory(&frame[..])?.to_rgb8();
        let subimg_width = img.width() / (width as u32);
        let subimg_height = img.height() / (height as u32);
        execute!(stdout, MoveTo(0, 0))?;
        for y in 0..height {
            for x in 0..width {
                let dx = x * subimg_width;
                let dy = y * subimg_height;
                let subimg = img.view(dx, dy, subimg_width, subimg_height)
                    .to_image();
                let ch = image_char(&subimg);
                let fg = image_color(&subimg);
                queue!(stdout, PrintStyledContent(ch.with(fg).on(BLACK)))?;
            }
            queue!(stdout, MoveToNextLine(1))?;
        }
        stdout.flush()?;
    }
}
