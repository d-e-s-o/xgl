// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod framebuffer;
mod matrices;
#[cfg(not(target_arch = "wasm32"))]
mod opengl;
mod program;
mod shader;
mod stack;
mod texture;
mod vertices;
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

pub use crate::framebuffer::Framebuffer;
pub use crate::matrices::MatrixStack;
pub use crate::program::Program;
pub use crate::shader::Shader;
pub use crate::texture::Builder as TextureBuilder;
pub use crate::texture::Texture;
pub use crate::texture::TextureInfo;
pub use crate::vertices::VertexArray;
pub use crate::vertices::VertexBuffer;

pub mod vertex {
  pub use crate::vertices::Attrib;
  pub use crate::vertices::AttribType;
  pub use crate::vertices::Attribs;
  pub use crate::vertices::VertexP3f as P3f;
  pub use crate::vertices::VertexP3fN3f as P3fN3f;
  pub use crate::vertices::VertexP3fT2f as P3fT2f;
  pub use crate::vertices::VertexP3fT2fN3f as P3fT2fN3f;
}
