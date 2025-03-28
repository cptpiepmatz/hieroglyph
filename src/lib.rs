pub mod fonts {
    include!(concat!(env!("OUT_DIR"), "/fonts.rs"));
}

mod draw;
mod segments;

pub use draw::*;
pub use segments::*;
