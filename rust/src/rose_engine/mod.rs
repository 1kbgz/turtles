// Rose Engine Lathe - Generic guilloché pattern generator
//
// This module implements a virtual rose engine lathe that can create
// various guilloché patterns by simulating the mechanical process of
// cutting decorative patterns into materials.

pub mod config;
pub mod cutting_bit;
pub mod lathe;
pub mod rosette;

// Re-export main types for convenience
pub use config::RoseEngineConfig;
pub use cutting_bit::{BitShape, CuttingBit};
pub use lathe::{Arc, RenderedOutput, RoseEngineLathe, ToolPathOutput};
pub use rosette::RosettePattern;
