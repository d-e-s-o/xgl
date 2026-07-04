// Copyright (C) 2025-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use anyhow::ensure;
use anyhow::Context as _;
use anyhow::Result;

use crate::sys;
use crate::sys::Gl as _;
use crate::Texture;


/// A framebuffer attachment.
#[non_exhaustive]
#[derive(Debug)]
pub enum Attachment<'attchmt> {
  Texture(&'attchmt Texture),
}

impl<'attchmt> From<&'attchmt Texture> for Attachment<'attchmt> {
  fn from(texture: &'attchmt Texture) -> Self {
    Self::Texture(texture)
  }
}


/// Configure the currently bound framebuffer.
fn configure_framebuffer(
  color: &Option<Attachment<'_>>,
  depth: &Option<Attachment<'_>>,
  context: &sys::Context,
) -> Result<()> {
  if let Some(color) = color {
    match color {
      Attachment::Texture(texture) => context.set_framebuffer_texture(
        sys::FramebufferAttachment::Color,
        texture.target(),
        texture,
      ),
    }
  }

  if let Some(depth) = depth {
    match depth {
      Attachment::Texture(texture) => {
        let () = context.set_framebuffer_texture(
          sys::FramebufferAttachment::Depth,
          texture.target(),
          texture,
        );
      },
    }
  }

  if color.is_none() {
    // Make it clear to OpenGL that we don't intend to use this
    // framebuffer for anything related to color.
    let () = context.unset_draw_buffer();
    let () = context.unset_read_buffer();
  }

  let status = context.check_framebuffer_status();
  ensure!(
    status == sys::FramebufferStatus::Complete,
    "failed to complete framebuffer: status: {status}"
  );
  Ok(())
}


/// Builder infrastructure for a [`Framebuffer`].
#[derive(Debug, Default)]
pub struct Builder<'attchmt> {
  color: Option<Attachment<'attchmt>>,
  depth: Option<Attachment<'attchmt>>,
}

impl<'attchmt> Builder<'attchmt> {
  /// Set the framebuffer's color attachment.
  pub fn set_color_attachment<A>(mut self, attachment: A) -> Self
  where
    A: 'attchmt,
    Attachment<'attchmt>: From<A>,
  {
    self.color = Some(Attachment::from(attachment));
    self
  }

  /// Set the framebuffer's depth attachment.
  pub fn set_depth_attachment<A>(mut self, attachment: A) -> Self
  where
    A: 'attchmt,
    Attachment<'attchmt>: From<A>,
  {
    self.depth = Some(Attachment::from(attachment));
    self
  }

  /// Build the [`Framebuffer`] object.
  pub fn build(&self, context: &sys::Context) -> Result<Framebuffer> {
    let fbo = Framebuffer {
      context: context.clone(),
      fbo: context
        .create_framebuffer()
        .context("failed to create framebuffer object")?,
    };

    let Builder { color, depth } = self;

    let () = fbo.bind();
    let result = configure_framebuffer(color, depth, context);
    let () = fbo.unbind();

    let () = result?;
    Ok(fbo)
  }
}


/// A framebuffer.
#[derive(Debug)]
pub struct Framebuffer {
  /// The GL context.
  context: sys::Context,
  /// The framebuffer object.
  fbo: sys::Framebuffer,
}

impl Framebuffer {
  /// Create a framebuffer builder.
  #[inline]
  pub fn builder() -> Builder<'static> {
    Builder::default()
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
      let _framebuffer = Framebuffer::builder()
        .set_depth_attachment(&depth_map)
        .build(&gl_context)
        .unwrap();
    })
  }
}
