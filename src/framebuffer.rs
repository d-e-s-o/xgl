// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use anyhow::ensure;
use anyhow::Context as _;
use anyhow::Result;

use crate::sys;
use crate::sys::Gl as _;
use crate::Texture;


/// A framebuffer.
#[derive(Debug)]
pub struct Framebuffer {
  /// The OpenGL context.
  context: sys::Context,
  /// The framebuffer object.
  fbo: sys::Framebuffer,
}

impl Framebuffer {
  /// Create a framebuffer using the provided texture as depth map (and
  /// nothing else).
  pub fn with_depth_map(texture: &Texture, context: &sys::Context) -> Result<Self> {
    let slf = Self {
      context: context.clone(),
      fbo: context
        .create_framebuffer()
        .context("failed to create framebuffer object")?,
    };

    let () = slf.bind();
    let () = context.set_framebuffer_depth_texture(texture.target(), texture);
    // Make it clear to OpenGL that we don't intend to use this
    // framebuffer for anything related to color.
    let () = context.unset_draw_buffer();
    let () = context.unset_read_buffer();
    let status = context.check_framebuffer_status();
    let () = slf.unbind();

    ensure!(
      status == sys::FramebufferStatus::Complete,
      "failed to complete framebuffer: status: {status}"
    );
    Ok(slf)
  }

  /// Bind the framebuffer.
  pub fn bind(&self) {
    let () = self.context.bind_framebuffer(Some(&self.fbo));
  }

  /// Unbind the framebuffer, effectively binding the default one
  /// instead.
  pub fn unbind(&self) {
    let () = self.context.bind_framebuffer(None);
  }
}

impl Drop for Framebuffer {
  fn drop(&mut self) {
    let () = self.context.delete_framebuffer(&self.fbo);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use test_fork::fork;

  use crate::winit::with_opengl_context;


  /// Check that we can create a framebuffer object.
  #[fork]
  #[test]
  fn framebuffer_creation() {
    with_opengl_context(|| {
      let gl_context = sys::Context::default();
      let depth_map = Texture::builder()
        .set_context(&gl_context)
        .new_depth_map(512, 512)
        .unwrap();
      let _framebuffer = Framebuffer::with_depth_map(&depth_map, &gl_context).unwrap();
    })
  }
}
