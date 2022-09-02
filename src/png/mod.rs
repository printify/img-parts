pub use self::chunk::PngChunk;
pub use self::image::{Png, CHUNK_EXIF, CHUNK_ICCP, CHUNK_IEND, SIGNATURE};

mod chunk;
mod image;

pub(crate) fn is_png(buf: &[u8]) -> bool {
    buf.starts_with(image::SIGNATURE)
}
