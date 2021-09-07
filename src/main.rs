mod color;
mod pixel_buffer;
use std::fs::File;
use std::io::{BufWriter, Write};

const DEFAULT_PATH_PPM: &'static str = "default_output.ppm";
const DEFAULT_PATH_PAM: &'static str = "default_output.pam";

fn main() {
    println!("saving image");
    save_as_ppm(128, 128, None);
}

pub fn save_as_ppm(width: usize, height: usize, color: Option<u32>) -> std::io::Result<()> {
    const DEFAULT_COLOR: u32 = 0xffffff;
    if (color.is_none()) {
        let mut file = File::create(DEFAULT_PATH_PPM)?;
        //file.write(buf: &[u8])
        write!(file, "P6 {} {} 255\n", width, height)?;

        let mut writer = BufWriter::new(file);
        for x in 0..width {
            for y in 0..height {
                let final_color: [u8; 3] = [
                    (((DEFAULT_COLOR >> 16) & 0xff) as u8),
                    (((DEFAULT_COLOR >> 8) & 0xff) as u8),
                    (((DEFAULT_COLOR >> 0) & 0xff) as u8),
                ];
                writer.write(&final_color)?;
                // write!(file, "{}{}{}", 255, 0, 0)?;
            }
        }
        //fs::write( file path: P, contents: C)
    }

    Ok(())
}
