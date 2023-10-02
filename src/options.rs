use ::clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    pub input: String,
    #[arg(default_value = "-")]
    pub output: String,
    #[arg(short = 'f')]
    pub font_path: String,
    #[arg(short = 'F')]
    pub font_size: f32,
    #[arg(short = 'W')]
    pub width: u32,
    #[arg(short = 'H')]
    pub height: u32,
    #[arg(short = 's')]
    pub single_frame: bool,
}
