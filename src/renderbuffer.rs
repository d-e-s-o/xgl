// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::ops::Deref;

use anyhow::Context as _;
use anyhow::Result;

use crate::sys;
use crate::sys::Gl as _;


/// A render buffer.
///
/// Render buffers are generally only useful as
/// [`Framebuffer`][crate::Framebuffer] attachments. Contrary to
/// textures, they cannot be read from directly (but may allow for
/// improved performance in turn).
#[derive(Debug)]
pub struct Renderbuffer {
  /// The GL context.
  context: sys::Context,
  /// The render buffer object.
  rbo: sys::Renderbuffer,
}

impl Renderbuffer {
  /// Create a new render buffer with the provided size and format.
  pub fn new(
    format: sys::TextureInternalFormat,
    w: u32,
    h: u32,
    context: &sys::Context,
  ) -> Result<Self> {
    let slf = Self {
      context: context.clone(),
      rbo: context
        .create_renderbuffer()
        .context("failed to create render buffer object")?,
    };

    let () = slf.bind();
    let () = context
      .set_renderbuffer_storage(format, w, h)
      .context("failed to configure render buffer storage")?;
    let () = slf.unbind();

    Ok(slf)
  }

  /// Bind the render buffer.
  pub fn bind(&self) {
    let () = self.context.bind_renderbuffer(Some(&self.rbo));
  }

  /// Unbind the render buffer, effectively binding the default one
  /// instead.
  pub fn unbind(&self) {
    let () = self.context.bind_renderbuffer(None);
  }
}

impl Deref for Renderbuffer {
  type Target = sys::Renderbuffer;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.rbo
  }
}

impl Drop for Renderbuffer {
  fn drop(&mut self) {
    let () = self.context.delete_renderbuffer(&self.rbo);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use test_fork::fork;

  use crate::winit::with_opengl_context;


  /// Check that we can create a [`Renderbuffer`] object.
  #[fork]
  #[test]
  fn renderbuffer_creation() {
    with_opengl_context(|| {
      let gl_context = sys::Context::default();
      let _renderbuffer =
        Renderbuffer::new(sys::TextureInternalFormat::RGB8, 256, 256, &gl_context).unwrap();
    })
  }
}
