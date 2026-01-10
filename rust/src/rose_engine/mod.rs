//! Rose Engine Lathe Module
//!
//! This module implements a comprehensive rose engine lathe system that can create
//! various guilloché patterns similar to those produced by a physical rose engine lathe.
//!
//! # Overview
//!
//! A rose engine lathe is a specialized ornamental turning lathe that uses rosettes (cams)
//! to create intricate patterns in metal, wood, or other materials. This digital
//! implementation simulates the operation of a physical rose engine.
//!
//! # Components
//!
//! - **Rosette Patterns**: Define the radial modulation (circular, elliptical, sinusoidal, multi-lobe, etc.)
//! - **Cutting Bit**: Defines the tool shape and size (V-shaped, flat, round, etc.)
//! - **Configuration**: Parameters for the rose engine (amplitude, base radius, phase, etc.)
//! - **Lathe**: Main implementation that generates tool paths and cut geometry
//!
//! # Example
//!
//! ```
//! use turtles::rose_engine::{RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern};
//! use turtles::ExportConfig;
//!
//! // Create a 12-lobe rose pattern
//! let mut config = RoseEngineConfig::new(20.0, 2.0);
//! config.rosette = RosettePattern::MultiLobe { lobes: 12 };
//!
//! // Use a 30-degree V-bit
//! let bit = CuttingBit::v_shaped(30.0, 1.0);
//!
//! // Create and generate the pattern
//! let mut lathe = RoseEngineLathe::new(config, bit).unwrap();
//! lathe.generate();
//!
//! // Export to SVG
//! lathe.to_svg("rose_pattern.svg").unwrap();
//!
//! // Export to STL for 3D printing/CNC
//! let export_config = ExportConfig::default();
//! lathe.to_stl("rose_pattern.stl", &export_config).unwrap();
//! ```
//!
//! # Preset Patterns
//!
//! The module includes several preset configurations for common patterns:
//!
//! ```
//! use turtles::rose_engine::{RoseEngineLathe, RoseEngineConfig, CuttingBit};
//!
//! // Classic multi-lobe pattern
//! let config = RoseEngineConfig::classic_multi_lobe(20.0, 12, 2.0);
//! let mut lathe = RoseEngineLathe::new(config, CuttingBit::default()).unwrap();
//! lathe.generate();
//!
//! // Sunburst pattern
//! let config = RoseEngineConfig::sunburst(20.0, 24, 1.5);
//! let mut lathe = RoseEngineLathe::new(config, CuttingBit::default()).unwrap();
//! lathe.generate();
//!
//! // Wave pattern
//! let config = RoseEngineConfig::wave(20.0, 8.0, 2.0);
//! let mut lathe = RoseEngineLathe::new(config, CuttingBit::default()).unwrap();
//! lathe.generate();
//! ```

pub mod config;
pub mod cutting_bit;
pub mod lathe;
pub mod lathe_run;
pub mod rosette;

// Re-export main types for convenience
pub use config::RoseEngineConfig;
pub use cutting_bit::{BitShape, CuttingBit};
pub use lathe::{Arc, RoseEngineLathe, RenderedOutput, ToolPathOutput};
pub use lathe_run::RoseEngineLatheRun;
pub use rosette::RosettePattern;
