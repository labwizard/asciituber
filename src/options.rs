use ::clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    /// The input device to read from.
    pub input: String,
    /// The output file to write to ('-' is stdout).
    #[arg(default_value = "-")]
    pub output: String,
    /// The path to the font to draw in.
    #[arg(short = 'f', long = "font-path")]
    pub font_path: String,
    /// The size of the font to draw in.
    #[arg(short = 'F', long = "font-size")]
    pub font_size: f32,
    /// The width of the input and output frames.
    #[arg(short = 'W', long = "width")]
    pub width: u32,
    /// The height of the input and output frames.
    #[arg(short = 'H', long = "height")]
    pub height: u32,
    /// The color to draw the background in.
    #[arg(
        long = "background-color",
        alias = "bg",
        alias = "bg-color",
        default_value = "#000000"
    )]
    pub bg_color: String,
    /// The color to draw the unhighlighted foreground in (if different from the
    /// background).
    #[arg(
        long = "alt-foreground-color",
        alias = "alt-fg",
        alias = "alt-fg-color"
    )]
    pub alt_fg_color: Option<String>,
    /// The color to draw the foreground in.
    #[arg(
        long = "foreground-color",
        alias = "fg",
        alias = "fg-color",
        default_value = "#ffffff"
    )]
    pub fg_color: String,
    /// Whether to stop after writing out one frame.
    #[arg(short = 's', long = "single-frame")]
    pub single_frame: bool,
}
