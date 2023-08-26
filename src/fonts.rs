use crate::state::StateMap;
use std::path::PathBuf;

pub fn render_font(
    message: &str,
    font: PathBuf,
    mut map: StateMap,
) -> Result<StateMap, anyhow::Error> {
    Ok(map)
}
