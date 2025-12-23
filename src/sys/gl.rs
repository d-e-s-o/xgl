// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::error::Error as StdError;
use std::fmt::Debug;


pub(super) mod protected {
  pub trait Sealed {}

  impl Sealed for u16 {}
}


pub trait BuiltinType<G>: protected::Sealed
where
  G: ?Sized + Gl,
{
  fn as_type() -> G::Type;
}


pub trait Gl: protected::Sealed {
  type Error: StdError;

  // Constant-y types.
  type Capability: Copy + Debug;
  type ClearMask: Copy + Debug;
  type CullFace: Copy + Debug;
  type Factor: Copy + Debug;
  type FrontFace: Copy + Debug;
  type Func: Copy + Debug;
  type Primitive: Copy + Debug + Eq;
  type ShaderType: Copy + Debug;
  type TextureCompareMode: Copy + Debug;
  type TextureFilter: Copy + Debug;
  type TextureFilterType: Copy + Debug;
  type TextureInternalFormat: Copy + Debug;
  type TexturePixelFormat: Copy + Debug;
  type TextureTarget: Copy + Debug;
  type TextureWrap: Copy + Debug;
  type Type: Copy + Debug;
  type VertexBufferTarget: Copy + Debug;
  type VertexBufferUsage: Copy + Debug;

  // Object types.
  type Framebuffer: Debug;
  type Program: Debug;
  type Shader: Debug;
  type Texture: Debug;
  type VertexArrayObject: Debug;
  type VertexBufferObject: Debug;

  // Misc types.
  type FramebufferStatus: Debug + Eq;
  type UniformLocation: Debug;


  fn error(&self) -> Result<(), Self::Error>;

  fn enable(&self, capability: Self::Capability);
  fn disable(&self, capability: Self::Capability);

  fn set_depth_func(&self, func: Self::Func);
  fn set_blend_func(&self, src_factor: Self::Factor, dst_factor: Self::Factor);
  fn set_front_face(&self, face: Self::FrontFace);
  fn set_cull_face(&self, face: Self::CullFace);

  fn set_viewport(&self, x: i32, y: i32, w: i32, h: i32);
  fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32);

  fn set_pixel_unpack_alignment(&self, alignment: u32);

  fn clear(&self, mask: Self::ClearMask);

  fn draw_arrays(&self, primitive: Self::Primitive, count: i32);
  fn draw_arrays_instanced(&self, primitive: Self::Primitive, count: i32, instance_count: i32);
  fn draw_elements<T>(&self, primitive: Self::Primitive, count: i32)
  where
    T: BuiltinType<Self>;

  fn create_framebuffer(&self) -> Result<Self::Framebuffer, Self::Error>;
  fn delete_framebuffer(&self, fbo: &Self::Framebuffer);
  fn bind_framebuffer(&self, fbo: Option<&Self::Framebuffer>);

  fn check_framebuffer_status(&self) -> Self::FramebufferStatus;
  fn set_framebuffer_depth_texture(
    &self,
    texture_target: Self::TextureTarget,
    texture: &Self::Texture,
  );

  fn unset_draw_buffer(&self);
  fn unset_read_buffer(&self);

  fn create_shader(&self, ty: Self::ShaderType) -> Option<Self::Shader>;
  fn delete_shader(&self, shader: &Self::Shader);
  fn set_shader_source(&self, shader: &Self::Shader, source: &str);
  fn compile_shader(&self, shader: &Self::Shader) -> Result<(), Vec<u8>>;
  fn attach_shader(&self, program: &Self::Program, shader: &Self::Shader);
  fn detach_shader(&self, program: &Self::Program, shader: &Self::Shader);

  fn create_program(&self) -> Option<Self::Program>;
  fn delete_program(&self, program: &Self::Program);
  fn link_program(&self, program: &Self::Program) -> Result<(), Vec<u8>>;
  fn validate_program(&self, program: &Self::Program) -> Result<(), Vec<u8>>;
  fn use_program(&self, program: &Self::Program);

  fn attrib_location(&self, program: &Self::Program, attrib: &str) -> Option<u32>;
  fn uniform_location(
    &self,
    program: &Self::Program,
    uniform: &str,
  ) -> Option<Self::UniformLocation>;

  fn uniform_fv<const N: usize>(
    &self,
    program: &Self::Program,
    location: &Self::UniformLocation,
  ) -> [f32; N];

  fn set_uniform_1i(&self, location: &Self::UniformLocation, data: i32);
  fn set_uniform_1ui(&self, location: &Self::UniformLocation, data: u32);
  fn set_uniform_1iv(&self, location: &Self::UniformLocation, data: &[i32]);
  fn set_uniform_3f(&self, location: &Self::UniformLocation, data: &[f32; 3]);
  fn set_uniform_4f(&self, location: &Self::UniformLocation, data: &[f32; 4]);
  fn set_uniform_matrix(&self, location: &Self::UniformLocation, matrix: &[f32; 16]);
  fn set_uniform_matrices(&self, location: &Self::UniformLocation, matrices: &[[f32; 16]]);

  fn create_vertex_buffer(&self) -> Result<Self::VertexBufferObject, Self::Error>;
  fn delete_vertex_buffer(&self, vbo: &Self::VertexBufferObject);
  fn bind_vertex_buffer(
    &self,
    target: Self::VertexBufferTarget,
    vbo: Option<&Self::VertexBufferObject>,
  );
  fn set_vertex_buffer_data<T>(
    &self,
    target: Self::VertexBufferTarget,
    usage: Self::VertexBufferUsage,
    data: &[T],
  );

  fn create_vertex_array(&self) -> Result<Self::VertexArrayObject, Self::Error>;
  fn delete_vertex_array(&self, vao: &Self::VertexArrayObject);
  fn bind_vertex_array(&self, vao: Option<&Self::VertexArrayObject>);

  fn enable_vertex_attrib_array(&self, idx: u32);
  fn set_vertex_attrib_pointer(
    &self,
    idx: u32,
    size: i32,
    ty: Self::Type,
    normalize: bool,
    stride: i32,
    offset: i32,
  );

  fn create_texture(&self) -> Result<Self::Texture, Self::Error>;
  fn delete_texture(&self, texture: &Self::Texture);
  fn bind_texture(&self, target: Self::TextureTarget, texture: Option<&Self::Texture>);

  fn set_active_texture_unit(&self, unit: u32);

  fn set_texture_image_2d(
    &self,
    target: Self::TextureTarget,
    internal_format: Self::TextureInternalFormat,
    pixel_format: Self::TexturePixelFormat,
    channel_type: Self::Type,
    w: u32,
    h: u32,
    pixels: Option<&[u8]>,
  ) -> Result<(), Self::Error>;

  fn set_texture_image_3d(
    &self,
    target: Self::TextureTarget,
    internal_format: Self::TextureInternalFormat,
    pixel_format: Self::TexturePixelFormat,
    channel_type: Self::Type,
    w: u32,
    h: u32,
    count: u32,
    pixels: Option<&[u8]>,
  ) -> Result<(), Self::Error>;

  fn set_texture_sub_image_3d(
    &self,
    target: Self::TextureTarget,
    pixel_format: Self::TexturePixelFormat,
    channel_type: Self::Type,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    h: u32,
    pixels: &[u8],
  ) -> Result<(), Self::Error>;

  fn set_texture_filter(
    &self,
    target: Self::TextureTarget,
    ty: Self::TextureFilterType,
    filter: Self::TextureFilter,
  );

  fn set_texture_compare_mode(&self, target: Self::TextureTarget, mode: Self::TextureCompareMode);
  fn set_texture_compare_func(&self, target: Self::TextureTarget, func: Self::Func);
  fn set_texture_wrap(&self, target: Self::TextureTarget, wrap: Self::TextureWrap);

  fn generate_mipmaps(&self, target: Self::TextureTarget);
}
