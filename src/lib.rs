// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod matrices;
#[cfg(not(target_arch = "wasm32"))]
mod opengl;
mod program;
mod shader;
mod stack;
mod texture;
#[cfg(target_arch = "wasm32")]
mod webgl;
#[cfg(test)]
mod winit;

pub mod sys {
  #[cfg(not(target_arch = "wasm32"))]
  pub use crate::opengl::*;
  #[cfg(target_arch = "wasm32")]
  pub use crate::webgl::*;
}

pub use crate::matrices::MatrixStack;
pub use crate::program::Program;
pub use crate::shader::Shader;
pub use crate::texture::Builder as TextureBuilder;
pub use crate::texture::Texture;
pub use crate::texture::TextureInfo;
