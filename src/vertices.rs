// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use std::mem::offset_of;

use anyhow::Context as _;
use anyhow::Result;

use crate::sys;
use crate::sys::Gl as _;


#[derive(Debug, Eq, PartialEq)]
pub enum AttribType {
  Position,
  Normal,
  Texture,
  Color,
}

impl Display for AttribType {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let ty = match self {
      Self::Position => "position",
      Self::Normal => "normal",
      Self::Texture => "texture",
      Self::Color => "color",
    };
    f.write_str(ty)
  }
}


#[derive(Debug)]
pub struct Attrib {
  pub size: i32,
  pub type_: sys::Type,
  pub normalize: bool,
  pub stride: i32,
  pub offset: i32,
}

pub trait Attribs {
  const ATTRIBS: &'static [(AttribType, Attrib)];
}


/// A vertex with only position information.
#[derive(Debug, Default)]
#[repr(C)]
pub struct VertexP3f {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Attribs for VertexP3f {
  const ATTRIBS: &'static [(AttribType, Attrib)] = &[(
    AttribType::Position,
    Attrib {
      size: 3,
      type_: sys::Type::Float,
      normalize: false,
      stride: size_of::<Self>() as _,
      offset: 0,
    },
  )];
}


/// A vertex with position and texture information.
#[derive(Debug, Default)]
#[repr(C)]
pub struct VertexP3fT2f {
  // Position
  pub x: f32,
  pub y: f32,
  pub z: f32,
  // Texture
  pub u: f32,
  pub v: f32,
}

impl Attribs for VertexP3fT2f {
  const ATTRIBS: &'static [(AttribType, Attrib)] = &[
    (
      AttribType::Position,
      Attrib {
        size: 3,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: 0,
      },
    ),
    (
      AttribType::Texture,
      Attrib {
        size: 2,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: offset_of!(Self, u) as _,
      },
    ),
  ];
}


/// A vertex with position and normal information.
#[derive(Debug, Default)]
#[repr(C)]
pub struct VertexP3fN3f {
  // Position
  pub x: f32,
  pub y: f32,
  pub z: f32,
  // Normal
  pub nx: f32,
  pub ny: f32,
  pub nz: f32,
}

impl Attribs for VertexP3fN3f {
  const ATTRIBS: &'static [(AttribType, Attrib)] = &[
    (
      AttribType::Position,
      Attrib {
        size: 3,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: 0,
      },
    ),
    (
      AttribType::Normal,
      Attrib {
        size: 3,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: offset_of!(Self, nx) as _,
      },
    ),
  ];
}


/// A vertex with position, texture, and normal information.
#[derive(Debug, Default)]
#[repr(C)]
pub struct VertexP3fT2fN3f {
  // Position
  pub x: f32,
  pub y: f32,
  pub z: f32,
  // Texture
  pub u: f32,
  pub v: f32,
  // Normal
  pub nx: f32,
  pub ny: f32,
  pub nz: f32,
}

impl Attribs for VertexP3fT2fN3f {
  const ATTRIBS: &'static [(AttribType, Attrib)] = &[
    (
      AttribType::Position,
      Attrib {
        size: 3,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: 0,
      },
    ),
    (
      AttribType::Texture,
      Attrib {
        size: 2,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: offset_of!(Self, u) as _,
      },
    ),
    (
      AttribType::Normal,
      Attrib {
        size: 3,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: offset_of!(Self, nx) as _,
      },
    ),
  ];
}


/// A vertex buffer object.
#[derive(Debug)]
pub struct VertexBuffer<T> {
  /// The OpenGL context.
  context: sys::Context,
  /// The OpenGL vertex buffer object.
  vbo: sys::VertexBufferObject,
  /// The "target" to bind to.
  target: sys::VertexBufferTarget,
  /// The number of items in the buffer.
  count: usize,
  /// Phantom data using `T`.
  _phantom: PhantomData<T>,
}

impl<T> VertexBuffer<T>
where
  T: Sized,
{
  #[inline]
  pub fn from_vertices(
    vertices: &[T],
    usage: sys::VertexBufferUsage,
    context: &sys::Context,
  ) -> Result<Self> {
    Self::from_data(sys::VertexBufferTarget::Array, usage, vertices, context)
  }

  #[inline]
  pub fn from_indices(
    indices: &[T],
    usage: sys::VertexBufferUsage,
    context: &sys::Context,
  ) -> Result<Self> {
    Self::from_data(
      sys::VertexBufferTarget::ElementArray,
      usage,
      indices,
      context,
    )
  }

  fn from_data(
    target: sys::VertexBufferTarget,
    usage: sys::VertexBufferUsage,
    data: &[T],
    context: &sys::Context,
  ) -> Result<Self> {
    let vbo = context
      .create_vertex_buffer()
      .context("failed to create vertex buffer")?;

    let slf = Self {
      context: context.clone(),
      vbo,
      target,
      count: data.len(),
      _phantom: PhantomData,
    };
    let () = slf.bind();
    let () = context.set_vertex_buffer_data(target, usage, data);
    let () = slf.unbind();
    Ok(slf)
  }

  #[inline]
  pub fn bind(&self) {
    let () = self
      .context
      .bind_vertex_buffer(self.target, Some(&self.vbo));
  }

  #[inline]
  pub fn unbind(&self) {
    let () = self.context.bind_vertex_buffer(self.target, None);
  }

  #[inline]
  pub fn item_count(&self) -> usize {
    self.count
  }
}

impl<T> Drop for VertexBuffer<T> {
  #[inline]
  fn drop(&mut self) {
    let () = self.context.delete_vertex_buffer(&self.vbo);
  }
}


/// A vertex array object.
#[derive(Debug)]
pub struct VertexArray {
  /// The OpenGL context.
  context: sys::Context,
  /// The OpenGL vertex array object.
  vao: sys::VertexArrayObject,
}

impl VertexArray {
  pub fn new<V>(
    vertex_buffer: &VertexBuffer<V>,
    attrib_indices: &[(u32, AttribType)],
    context: &sys::Context,
  ) -> Result<Self>
  where
    V: Attribs,
  {
    let vertex_array = context.create_vertex_array()?;
    let slf = Self {
      context: context.clone(),
      vao: vertex_array,
    };
    let () = slf.bind();
    let () = vertex_buffer.bind();

    let result = V::ATTRIBS.iter().try_for_each(|(attrib_type, attrib)| {
      let (idx, _) = attrib_indices
        .iter()
        .find(|(_, ty)| ty == attrib_type)
        .with_context(|| format!("failed to find {attrib_type} vertex attribute index"))?;

      let () = context.enable_vertex_attrib_array(*idx);
      let () = context.set_vertex_attrib_pointer(
        *idx,
        attrib.size,
        attrib.type_,
        attrib.normalize,
        attrib.stride,
        attrib.offset,
      );
      Ok(())
    });

    let () = vertex_buffer.unbind();
    let () = slf.unbind();

    result.map(|()| slf)
  }

  #[inline]
  pub fn bind(&self) {
    let () = self.context.bind_vertex_array(Some(&self.vao));
  }

  #[inline]
  fn unbind(&self) {
    let () = self.context.bind_vertex_array(None);
  }
}

impl Drop for VertexArray {
  #[inline]
  fn drop(&mut self) {
    let () = self.context.delete_vertex_array(&self.vao);
  }
}
