use crate::state::StateMap;
use anyhow::anyhow;
use rusttype::{point, Font, Scale};
use std::path::PathBuf;

pub fn render_font(
    message: &str,
    font: PathBuf,
    mut map: StateMap,
) -> Result<StateMap, anyhow::Error> {
    let font = load_font(font)?;
    let scale = Scale { x: 5.0, y: 7.0 };
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

    println!("width: {}, height: {}", width, 7);

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
