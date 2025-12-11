// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::iter::once;
use std::ops::Deref;

use anyhow::ensure;
use anyhow::Context as _;
use anyhow::Result;

use crate::sys;


/// Information about an image to be used as a texture.
#[derive(Clone, Copy, Debug)]
pub struct TextureInfo {
  /// The texture's width.
  pub width: u32,
  /// The texture's height.
  pub height: u32,
  /// The texture's internal format.
  pub intern_format: sys::TextureInternalFormat,
  /// The texture's pixel format.
  pub pixel_format: sys::TexturePixelFormat,
  /// The texture's color format/color channel type.
  pub color_format: sys::Type,
}

impl AsRef<Self> for TextureInfo {
  #[inline]
  fn as_ref(&self) -> &Self {
    self
  }
}


/// Populate the currently bound texture with the given image data.
fn populate_texture(
  context: &sys::Context,
  target: sys::TextureTarget,
  data: &[u8],
  info: &TextureInfo,
) -> Result<()> {
  let () = context
    .set_texture_image_2d(
      target,
      info.intern_format,
      info.pixel_format,
      info.color_format,
      info.width,
      info.height,
      Some(data),
    )
    .context("failed to populate texture")?;

  Ok(())
}

fn populate_3d_texture(
  context: &sys::Context,
  idx: u32,
  target: sys::TextureTarget,
  data: &[u8],
  info: &TextureInfo,
) -> Result<()> {
  let x = 0;
  let y = 0;
  let z = idx;
  let () = context
    .set_texture_sub_image_3d(
      target,
      info.pixel_format,
      info.color_format,
      x,
      y,
      z,
      info.width,
      info.height,
      data,
    )
    .with_context(|| "failed to initialize 3D texture (index = {idx})")?;
  Ok(())
}


/// Builder infrastructure for a texture.
#[derive(Debug, Default)]
pub struct Builder<C = ()> {
  /// The OpenGL context.
  context: C,
  /// Whether or not to create mipmaps.
  mipmaps: bool,
}

impl Builder<()> {
  /// Set whether or not to use mipmaps for the texture.
  pub fn set_mipmaps(mut self, mipmaps: bool) -> Self {
    self.mipmaps = mipmaps;
    self
  }

  /// Set the texture's OpenGL context.
  pub fn set_context(self, context: &sys::Context) -> Builder<sys::Context> {
    let Self {
      context: (),
      mipmaps,
    } = self;

    Builder {
      context: context.clone(),
      mipmaps,
    }
  }
}

impl Builder<sys::Context> {
  /// Apply a certain texture state to the currently bound texture
  /// before it is being populated.
  ///
  /// Note that this function modifies global state and won't reset it.
  /// The modus operandi here is for everything that is required to be
  /// set unconditionally and if everybody does that there is no need to
  /// set and restore.
  fn apply_pre_texture_state(&self, texture: &Texture) {
    let Self {
      context: _,
      mipmaps,
    } = self;
    let target = texture.target();
    let () = self.context.set_texture_filter(
      target,
      sys::TextureFilterType::Magnify,
      sys::TextureFilter::Linear,
    );

    let filter = if *mipmaps {
      sys::TextureFilter::LinearMipmapLinear
    } else {
      sys::TextureFilter::Linear
    };
    let () = self
      .context
      .set_texture_filter(target, sys::TextureFilterType::Minimize, filter);
    let () = self
      .context
      .set_texture_wrap(target, sys::TextureWrap::ClampToEdge);

    // TODO: Probably not the full story.
    let () = self.context.set_pixel_unpack_alignment(1);
  }

  /// Apply a certain texture state to the currently bound texture
  /// after it has been populated.
  fn apply_post_texture_state(&self, texture: &Texture) {
    let Self {
      context: _,
      mipmaps,
    } = self;
    let target = texture.target();

    if *mipmaps {
      let () = self.context.generate_mipmaps(target);
    }
  }

  /// Create a new 2D `Texture` suitable for use as a depth map.
  pub fn new_depth_map(&self, width: u32, height: u32) -> Result<Texture> {
    let target = sys::TextureTarget::Texture2D;
    let texture = Texture {
      context: self.context.clone(),
      texture: self
        .context
        .create_texture()
        .context("failed to generate OpenGL texture ID")?,
      target,
    };

    let () = texture.bind();
    let () = self.apply_pre_texture_state(&texture);

    // NB: The compare mode only applies to depth maps, so we don't need
    //     to set it for other types.
    let () = self
      .context
      .set_texture_compare_mode(target, sys::TextureCompareMode::RefToTexture);
    let () = self
      .context
      .set_texture_compare_func(target, sys::Func::Greater);

    let result = self
      .context
      .set_texture_image_2d(
        target,
        sys::TextureInternalFormat::Depth,
        sys::TexturePixelFormat::Depth,
        sys::Type::Float,
        width,
        height,
        None,
      )
      .context("failed to depth map texture");

    if let Ok(()) = result {
      let () = self.apply_post_texture_state(&texture);
    }
    let () = texture.unbind();

    let () = result?;
    Ok(texture)
  }

  /// Create a new 2D `Texture` from the provided image data.
  pub fn from_image(&self, data: &[u8], info: &TextureInfo) -> Result<Texture> {
    let target = sys::TextureTarget::Texture2D;

    let texture = Texture {
      context: self.context.clone(),
      texture: self
        .context
        .create_texture()
        .context("failed to generate OpenGL texture ID")?,
      target,
    };
    let () = texture.bind();
    let () = self.apply_pre_texture_state(&texture);
    let result = populate_texture(&self.context, target, data, info);
    if let Ok(()) = result {
      let () = self.apply_post_texture_state(&texture);
    }
    let () = texture.unbind();

    let () = result?;
    Ok(texture)
  }

  /// Create a new 3D `Texture` using the provided images.
  pub fn from_images<I, D, M>(&self, width: u32, height: u32, mut images: I) -> Result<Texture>
  where
    I: ExactSizeIterator<Item = Result<(D, M)>>,
    D: AsRef<[u8]>,
    M: AsRef<TextureInfo>,
  {
    let count = images.len();
    let (image, info) = images
      .next()
      .expect("3D texture creation requires at least one image")?;
    let target = sys::TextureTarget::Texture2DArray;

    let texture = Texture {
      context: self.context.clone(),
      texture: self
        .context
        .create_texture()
        .context("failed to generate OpenGL texture ID")?,
      target,
    };
    let () = texture.bind();
    let () = self.apply_pre_texture_state(&texture);
    let result = self
      .context
      .set_texture_image_3d(
        target,
        info.as_ref().intern_format,
        info.as_ref().pixel_format,
        info.as_ref().color_format,
        width,
        height,
        count as _,
        None,
      )
      .context("failed to initialize 3D texture");
    let result = result.and_then(|()| {
      once(Ok((image, info)))
        .chain(images)
        .enumerate()
        .try_for_each(|(idx, result)| {
          let (image, info) = result?;
          let image = image.as_ref();
          let info = info.as_ref();
          ensure!(
            info.width <= width && info.height <= height,
            "image {idx} is larger than maximum bounds provided"
          );

          let () = populate_3d_texture(&self.context, idx as _, target, image, info)?;
          Ok(())
        })
    });
    if let Ok(()) = result {
      let () = self.apply_post_texture_state(&texture);
    }
    let () = texture.unbind();
    let () = result?;

    Ok(texture)
  }
}


/// A texture.
#[derive(Debug)]
pub struct Texture {
  /// The OpenGL context.
  context: sys::Context,
  /// The texture ID.
  texture: sys::Texture,
  /// The "target" to bind to.
  target: sys::TextureTarget,
}

impl Texture {
  /// Create a texture builder.
  #[inline]
  pub fn builder() -> Builder {
    Builder::default()
  }

  #[inline]
  pub fn bind(&self) {
    self.context.bind_texture(self.target, Some(&self.texture))
  }

  #[inline]
  pub fn unbind(&self) {
    self.context.bind_texture(self.target, None)
  }

  /// Retrieve the texture's "target".
  #[inline]
  pub fn target(&self) -> sys::TextureTarget {
    self.target
  }
}

impl Deref for Texture {
  type Target = sys::Texture;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.texture
  }
}

impl Drop for Texture {
  #[inline]
  fn drop(&mut self) {
    let () = self.context.delete_texture(&self.texture);
  }
}
