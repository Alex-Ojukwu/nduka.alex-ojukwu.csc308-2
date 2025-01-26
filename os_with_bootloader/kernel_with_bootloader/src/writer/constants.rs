use noto_sans_mono_bitmap::{get_raster_width, FontWeight, RasterHeight};

/// Constants for the usage of the `noto_sans_mono_bitmap` crate.
pub mod font_constants {
    use super::*;

    /// Height of each char raster, determining line height.
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    /// Width of each single symbol in the monospaced font.
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    /// Backup character used if a desired symbol is unavailable in the font.
    pub const BACKUP_CHAR: char = '�';

    /// Font weight for rasterized characters.
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

    /// Representation of a backspace character.
    pub const BACKSPACE: char = '\u{0008}';
}
