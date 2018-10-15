use structopt::StructOpt;

use super::Color;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rtex",
    about = "Generate images from LaTeX one-liners"
)]
pub struct Opt {
    /// Set the background colour
    #[structopt(short = "b", long = "background-color", default_value = "White")]
    pub background: Color,

    /// Set the border colour
    #[structopt(short = "B", default_value = "0,0,0")]
    pub border: Color,

    /// Set the output resolution to the specified dpi.
    #[structopt(short = "d", default_value = "0,0,0")]
    pub dpi: usize,

    /// Set the environment
    #[structopt(short = "e", default_value = "\\$")]
    pub environment: String,

    /// The LaTeX formula
    #[structopt(short = "f", default_value = "")]
    pub formula: String,

    /// Set the foreground colour
    #[structopt(short = "F", default_value = "Black")]
    pub foreground: Color,

    /// Input file in header
    #[structopt(short = "H", default_value = "")]
    pub headers: String, // Vec<String>,

    /// Log file
    #[structopt(short = "l")]
    pub log: bool,

    /// Set the margin
    #[structopt(short = "m", default_value = "")]
    pub margin: f32,

    /// Strip meta information
    #[structopt(short = "M")]
    pub no_meta: bool,

    /// Specify the output file name
    #[structopt(short = "o", default_value = "")]
    pub output_file: String, //TODO: Change to AsRef<Path> or whatevs

    /// Optimize the image
    #[structopt(short = "O")]
    pub optimize: bool,

    //XXX: This isn't actually colon separated
    /// A colon separated list of LaTeX package names
    #[structopt(short = "p")]
    pub packages: String, // Vec<String>,

    /// Set the padding
    #[structopt(short = "", default_value = "")]
    pub padding: f32,

    /// Read an image and print the LaTeX formula
    #[structopt(short = "", default_value = "")]
    pub image: String, //TODO: Change to AsRef<Path> or whatevs

    // TODO: Only allow actual LaTeX fonts
    /// Set the font size
    #[structopt(short = "", default_value = "11")]
    pub font_size: f32,

    /// Silent mode: don't print image file name
    #[structopt(short = "")]
    pub silent: bool,
}
