use crate::{
    consts::{GRID_SIZE, PIXEL_HEIGHT, PIXEL_WIDTH, WEEKS},
    state::StateMap,
};
use anyhow::anyhow;
use rusttype::{point, Font, Scale};
use std::path::PathBuf;

pub fn render_font(
    message: &str,
    font: PathBuf,
    mut map: StateMap,
) -> Result<StateMap, anyhow::Error> {
    let font = load_font(font)?;
    let scale = Scale {
        x: PIXEL_WIDTH as f32,
        y: PIXEL_HEIGHT as f32,
    };
    let metrics = font.v_metrics(scale);
    let offset = point(0.0, metrics.ascent);

    let glyphs: Vec<_> = font.layout(message, scale, offset).collect();

    // shamelessly taken from examples
    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;

    if width * PIXEL_HEIGHT > GRID_SIZE {
        return Err(anyhow!("cannot fit into grid"));
    }

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < width as i32 && y >= 0 && y < PIXEL_HEIGHT as i32 {
                    let result = (v * 10.0).ceil() as u8;
                    map.0[x as usize + y as usize * width].1 = result;
                }
            })
        }
    }

    Ok(map)
}

fn load_font<'a>(font: PathBuf) -> Result<Font<'a>, anyhow::Error> {
    let data = std::fs::read(&font).unwrap();
    if let Some(font) = Font::try_from_vec(data) {
        Ok(font)
    } else {
        Err(anyhow!("Cannot load font"))
    }
}
