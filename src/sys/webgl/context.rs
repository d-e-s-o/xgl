// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::mem::size_of_val;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::slice;

use wasm_bindgen::prelude::JsValue;

use web_sys::js_sys::Float32Array;
use web_sys::js_sys::Uint32Array;
use web_sys::WebGl2RenderingContext;

pub use web_sys::WebGlBuffer as VertexBufferObject;
pub use web_sys::WebGlFramebuffer as Framebuffer;
pub use web_sys::WebGlProgram as Program;
pub use web_sys::WebGlShader as Shader;
pub use web_sys::WebGlTexture as Texture;
pub use web_sys::WebGlUniformLocation as UniformLocation;
pub use web_sys::WebGlVertexArrayObject as VertexArrayObject;

use crate::sys::BuiltinType;
use crate::sys::Gl;
use crate::sys::Sealed;


#[derive(Debug, Eq, PartialEq)]
pub struct Error(u32);

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let err = match self.0 {
      WebGl2RenderingContext::NO_ERROR => "no error",
      WebGl2RenderingContext::INVALID_ENUM => "invalid enum (INVALID_ENUM)",
      WebGl2RenderingContext::INVALID_VALUE => "invalid value (INVALID_VALUE)",
      WebGl2RenderingContext::INVALID_OPERATION => "invalid operation (INVALID_OPERATION)",
      WebGl2RenderingContext::INVALID_FRAMEBUFFER_OPERATION => {
        "invalid framebuffer operation (INVALID_FRAMEBUFFER_OPERATION)"
      },
      WebGl2RenderingContext::OUT_OF_MEMORY => "out of memory (OUT_OF_MEMORY)",
      WebGl2RenderingContext::CONTEXT_LOST_WEBGL => "lost WebGL context (CONTEXT_LOST_WEBGL)",
      _ => return write!(f, "WebGL error: {:#x}", self.0),
    };
    write!(f, "WebGL error: {err}")
  }
}

impl StdError for Error {}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Type {
  Float = WebGl2RenderingContext::FLOAT,
  UnsignedByte = WebGl2RenderingContext::UNSIGNED_BYTE,
  UnsignedShort = WebGl2RenderingContext::UNSIGNED_SHORT,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Capability {
  Blend = WebGl2RenderingContext::BLEND,
  CullFace = WebGl2RenderingContext::CULL_FACE,
  DepthTest = WebGl2RenderingContext::DEPTH_TEST,
  ScissorTest = WebGl2RenderingContext::SCISSOR_TEST,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Func {
  LessOrEqual = WebGl2RenderingContext::LEQUAL,
  Greater = WebGl2RenderingContext::GREATER,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Factor {
  SrcAlpha = WebGl2RenderingContext::SRC_ALPHA,
  OneMinusSrcAlpha = WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum FrontFace {
  ClockWise = WebGl2RenderingContext::CW,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum CullFace {
  Back = WebGl2RenderingContext::BACK,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
  Lines = WebGl2RenderingContext::LINES,
  Triangles = WebGl2RenderingContext::TRIANGLES,
  TriangleFan = WebGl2RenderingContext::TRIANGLE_FAN,
  TriangleStrip = WebGl2RenderingContext::TRIANGLE_STRIP,
}


#[derive(Clone, Copy, Debug)]
pub struct ClearMask(u32);

#[expect(non_upper_case_globals)]
impl ClearMask {
  pub const ColorBuffer: Self = Self(WebGl2RenderingContext::COLOR_BUFFER_BIT);
  pub const DepthBuffer: Self = Self(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
}

impl BitOr for ClearMask {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self::Output {
    let mut result = self;
    result |= other;
    result
  }
}

impl BitOrAssign for ClearMask {
  #[inline]
  fn bitor_assign(&mut self, other: Self) {
    self.0 |= other.0;
  }
}


#[derive(Debug, Eq, PartialEq)]
pub struct FramebufferStatus(u32);

#[expect(non_upper_case_globals)]
impl FramebufferStatus {
  pub const Complete: Self = Self(WebGl2RenderingContext::FRAMEBUFFER_COMPLETE);
}

impl Display for FramebufferStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{:#x}", self.0)
  }
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
  Fragment = WebGl2RenderingContext::FRAGMENT_SHADER,
  Vertex = WebGl2RenderingContext::VERTEX_SHADER,
}

impl ShaderType {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Fragment => "fragment",
      Self::Vertex => "vertex",
    }
  }
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum VertexBufferTarget {
  Array = WebGl2RenderingContext::ARRAY_BUFFER,
  ElementArray = WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureTarget {
  Texture2D = WebGl2RenderingContext::TEXTURE_2D,
  Texture2DArray = WebGl2RenderingContext::TEXTURE_2D_ARRAY,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureInternalFormat {
  Gray8 = WebGl2RenderingContext::R8,
  Depth = WebGl2RenderingContext::DEPTH_COMPONENT32F,
  RGB8 = WebGl2RenderingContext::RGB8,
  SRGB8 = WebGl2RenderingContext::SRGB8,
  RGBA8 = WebGl2RenderingContext::RGBA8,
  SRGBA8 = WebGl2RenderingContext::SRGB8_ALPHA8,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TexturePixelFormat {
  Gray = WebGl2RenderingContext::RED,
  Depth = WebGl2RenderingContext::DEPTH_COMPONENT,
  RGB = WebGl2RenderingContext::RGB,
  RGBA = WebGl2RenderingContext::RGBA,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureCompareMode {
  RefToTexture = WebGl2RenderingContext::COMPARE_REF_TO_TEXTURE,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureWrap {
  ClampToEdge = WebGl2RenderingContext::CLAMP_TO_EDGE,
  Repeat = WebGl2RenderingContext::REPEAT,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureFilterType {
  Minimize = WebGl2RenderingContext::TEXTURE_MIN_FILTER,
  Magnify = WebGl2RenderingContext::TEXTURE_MAG_FILTER,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureFilter {
  Linear = WebGl2RenderingContext::LINEAR,
  LinearMipmapLinear = WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR,
}

impl BuiltinType<Context> for u16 {
  fn as_type() -> Type {
    Type::UnsignedShort
  }
}


/// The OpenGL context in use.
///
/// A context is guaranteed to be cheaply cloneable.
#[derive(Clone, Debug)]
pub struct Context(WebGl2RenderingContext);

impl Context {
  #[inline]
  pub fn new(context: WebGl2RenderingContext) -> Self {
    Self(context)
  }

  fn check_program(&self, program: &Program) -> Result<(), Vec<u8>> {
    let result = self.0.get_program_info_log(program);
    match result {
      Some(log) if !log.is_empty() => Err(log.into_bytes()),
      _ => Ok(()),
    }
  }

  #[inline]
  fn set_uniform_matrices_impl(&self, location: &UniformLocation, matrices: &[[f32; 16]]) {
    let transpose = false;
    let matrices =
      unsafe { slice::from_raw_parts(matrices.as_ptr().cast::<f32>(), matrices.len() * 16) };
    let () = self
      .0
      .uniform_matrix4fv_with_f32_array(Some(location), transpose, matrices);
    debug_assert_eq!(self.error(), Ok(()));
  }
}

impl Sealed for Context {}

impl Gl for Context {
  type Error = Error;

  type Capability = Capability;
  type ClearMask = ClearMask;
  type CullFace = CullFace;
  type Factor = Factor;
  type FramebufferStatus = FramebufferStatus;
  type FrontFace = FrontFace;
  type Func = Func;
  type Primitive = Primitive;
  type ShaderType = ShaderType;
  type TextureCompareMode = TextureCompareMode;
  type TextureFilter = TextureFilter;
  type TextureFilterType = TextureFilterType;
  type TextureInternalFormat = TextureInternalFormat;
  type TexturePixelFormat = TexturePixelFormat;
  type TextureTarget = TextureTarget;
  type TextureWrap = TextureWrap;
  type Type = Type;
  type VertexBufferTarget = VertexBufferTarget;

  type Framebuffer = Framebuffer;
  type Program = Program;
  type Shader = Shader;
  type Texture = Texture;
  type VertexArrayObject = VertexArrayObject;
  type VertexBufferObject = VertexBufferObject;

  type UniformLocation = UniformLocation;

  #[inline]
  fn error(&self) -> Result<(), Error> {
    let error = self.0.get_error();
    if error == WebGl2RenderingContext::NO_ERROR {
      Ok(())
    } else {
      Err(Error(error))
    }
  }

  #[inline]
  fn enable(&self, capability: Capability) {
    let () = self.0.enable(capability as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn disable(&self, capability: Capability) {
    let () = self.0.disable(capability as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_depth_func(&self, func: Func) {
    let () = self.0.depth_func(func as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_blend_func(&self, src_factor: Factor, dst_factor: Factor) {
    let () = self.0.blend_func(src_factor as _, dst_factor as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_front_face(&self, face: FrontFace) {
    let () = self.0.front_face(face as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_cull_face(&self, face: CullFace) {
    let () = self.0.cull_face(face as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_viewport(&self, x: i32, y: i32, w: i32, h: i32) {
    let () = self.0.viewport(x, y, w, h);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
    let () = self.0.clear_color(r, g, b, a);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_pixel_unpack_alignment(&self, alignment: u32) {
    let () = self
      .0
      .pixel_storei(WebGl2RenderingContext::UNPACK_ALIGNMENT, alignment as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn clear(&self, mask: ClearMask) {
    let () = self.0.clear(mask.0);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn draw_arrays(&self, primitive: Primitive, count: i32) {
    let () = self.0.draw_arrays(primitive as _, 0, count);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn draw_arrays_instanced(&self, primitive: Primitive, count: i32, instance_count: i32) {
    let () = self
      .0
      .draw_arrays_instanced(primitive as _, 0, count, instance_count);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn draw_elements<T>(&self, primitive: Primitive, count: i32)
  where
    T: BuiltinType<Self>,
  {
    let () = self
      .0
      .draw_elements_with_i32(primitive as _, count, T::as_type() as _, 0);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn create_framebuffer(&self) -> Result<Framebuffer, Error> {
    self
      .0
      .create_framebuffer()
      .ok_or_else(|| self.error().unwrap_err())
  }

  #[inline]
  fn delete_framebuffer(&self, fbo: &Framebuffer) {
    let () = self.0.delete_framebuffer(Some(fbo));
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_framebuffer(&self, fbo: Option<&Framebuffer>) {
    let () = self
      .0
      .bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, fbo);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_framebuffer_depth_texture(&self, texture_target: TextureTarget, texture: &Texture) {
    let mipmap_level = 0;
    let () = self.0.framebuffer_texture_2d(
      WebGl2RenderingContext::FRAMEBUFFER,
      WebGl2RenderingContext::DEPTH_ATTACHMENT,
      texture_target as _,
      Some(texture),
      mipmap_level,
    );
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn unset_draw_buffer(&self) {
    let buffers = JsValue::from(Uint32Array::from([WebGl2RenderingContext::NONE].as_slice()));
    let () = self.0.draw_buffers(&buffers);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn unset_read_buffer(&self) {
    let () = self.0.read_buffer(WebGl2RenderingContext::NONE);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn check_framebuffer_status(&self) -> FramebufferStatus {
    let status = self
      .0
      .check_framebuffer_status(WebGl2RenderingContext::FRAMEBUFFER);
    FramebufferStatus(status)
  }

  #[inline]
  fn create_shader(&self, ty: ShaderType) -> Option<Shader> {
    self.0.create_shader(ty as _)
  }

  #[inline]
  fn delete_shader(&self, shader: &Shader) {
    let () = self.0.delete_shader(Some(shader));
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_shader_source(&self, shader: &Shader, source: &str) {
    let () = self.0.shader_source(shader, source);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn compile_shader(&self, shader: &Shader) -> Result<(), Vec<u8>> {
    let () = self.0.compile_shader(shader);
    let result = self.0.get_shader_info_log(shader);
    match result {
      Some(log) if !log.is_empty() => Err(log.into_bytes()),
      _ => Ok(()),
    }
  }

  #[inline]
  fn attach_shader(&self, program: &Program, shader: &Shader) {
    let () = self.0.attach_shader(program, shader);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn detach_shader(&self, program: &Program, shader: &Shader) {
    let () = self.0.detach_shader(program, shader);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn create_program(&self) -> Option<Program> {
    self.0.create_program()
  }

  #[inline]
  fn delete_program(&self, program: &Program) {
    let () = self.0.delete_program(Some(program));
    debug_assert_eq!(self.error(), Ok(()));
  }

  fn link_program(&self, program: &Program) -> Result<(), Vec<u8>> {
    let () = self.0.link_program(program);
    let () = self.check_program(program)?;
    Ok(())
  }

  fn validate_program(&self, program: &Program) -> Result<(), Vec<u8>> {
    let () = self.0.validate_program(program);
    let () = self.check_program(program)?;
    Ok(())
  }

  #[inline]
  fn use_program(&self, program: &Program) {
    let () = self.0.use_program(Some(program));
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn attrib_location(&self, program: &Program, attrib: &str) -> Option<u32> {
    let idx = self.0.get_attrib_location(program, attrib);
    if idx >= 0 {
      Some(u32::try_from(idx).unwrap())
    } else {
      None
    }
  }

  #[inline]
  fn uniform_location(&self, program: &Program, name: &str) -> Option<UniformLocation> {
    self.0.get_uniform_location(program, name)
  }

  #[inline]
  fn uniform_fv<const N: usize>(&self, program: &Program, location: &UniformLocation) -> [f32; N] {
    let value = self.0.get_uniform(program, location);
    let array = Float32Array::from(value);
    assert!(array.length() as usize == N);

    let mut result = [0f32; N];
    let () = array.copy_to(&mut result);
    result
  }

  #[inline]
  fn set_uniform_1i(&self, location: &UniformLocation, data: i32) {
    let () = self.0.uniform1i(Some(location), data);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_1ui(&self, location: &UniformLocation, data: u32) {
    let () = self.0.uniform1ui(Some(location), data);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_1iv(&self, location: &UniformLocation, data: &[i32]) {
    let () = self.0.uniform1iv_with_i32_array(Some(location), data);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_3f(&self, location: &UniformLocation, data: &[f32; 3]) {
    let () = self
      .0
      .uniform3fv_with_f32_array(Some(location), data.as_slice());
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_4f(&self, location: &UniformLocation, data: &[f32; 4]) {
    let () = self
      .0
      .uniform4fv_with_f32_array(Some(location), data.as_slice());
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_matrices(&self, location: &UniformLocation, matrices: &[[f32; 16]]) {
    self.set_uniform_matrices_impl(location, matrices)
  }

  #[inline]
  fn set_uniform_matrix(&self, location: &UniformLocation, matrix: &[f32; 16]) {
    self.set_uniform_matrices_impl(location, slice::from_ref(matrix))
  }

  #[inline]
  fn create_vertex_buffer(&self) -> Result<VertexBufferObject, Error> {
    self
      .0
      .create_buffer()
      .ok_or_else(|| self.error().unwrap_err())
  }

  #[inline]
  fn delete_vertex_buffer(&self, vbo: &VertexBufferObject) {
    let () = self.0.delete_buffer(Some(vbo));
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_vertex_buffer(&self, target: VertexBufferTarget, vbo: Option<&VertexBufferObject>) {
    let () = self.0.bind_buffer(target as _, vbo);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_vertex_buffer_data<T>(&self, target: VertexBufferTarget, data: &[T]) {
    let ptr = data.as_ptr().cast::<u8>();
    let buf = unsafe { slice::from_raw_parts(ptr, size_of_val(data)) };

    let () =
      self
        .0
        .buffer_data_with_u8_array(target as _, buf, WebGl2RenderingContext::STATIC_DRAW);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn create_vertex_array(&self) -> Result<VertexArrayObject, Error> {
    self
      .0
      .create_vertex_array()
      .ok_or_else(|| self.error().unwrap_err())
  }

  #[inline]
  fn delete_vertex_array(&self, vao: &VertexArrayObject) {
    let () = self.0.delete_vertex_array(Some(vao));
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_vertex_array(&self, vao: Option<&VertexArrayObject>) {
    let () = self.0.bind_vertex_array(vao);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn enable_vertex_attrib_array(&self, idx: u32) {
    let () = self.0.enable_vertex_attrib_array(idx);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_vertex_attrib_pointer(
    &self,
    idx: u32,
    size: i32,
    ty: Type,
    normalize: bool,
    stride: i32,
    offset: i32,
  ) {
    let () = self
      .0
      .vertex_attrib_pointer_with_i32(idx, size, ty as _, normalize, stride, offset);
    debug_assert_eq!(self.error(), Ok(()));
  }

  fn create_texture(&self) -> Result<Texture, Error> {
    self
      .0
      .create_texture()
      .ok_or_else(|| self.error().unwrap_err())
  }

  #[inline]
  fn delete_texture(&self, texture: &Texture) {
    let () = self.0.delete_texture(Some(texture));
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_texture(&self, target: TextureTarget, texture: Option<&Texture>) {
    let () = self.0.bind_texture(target as _, texture);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_active_texture_unit(&self, unit: u32) {
    let () = self
      .0
      .active_texture(WebGl2RenderingContext::TEXTURE0 + unit);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_image_2d(
    &self,
    target: TextureTarget,
    internal_format: TextureInternalFormat,
    pixel_format: TexturePixelFormat,
    channel_type: Type,
    w: u32,
    h: u32,
    pixels: Option<&[u8]>,
  ) -> Result<(), Error> {
    let level = 0;
    let border = 0;

    self
      .0
      .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        target as _,
        level,
        internal_format as _,
        w as _,
        h as _,
        border,
        pixel_format as _,
        channel_type as _,
        pixels,
      )
      .map_err(|_err| self.error().unwrap_err())
  }

  #[inline]
  fn set_texture_image_3d(
    &self,
    target: TextureTarget,
    internal_format: TextureInternalFormat,
    pixel_format: TexturePixelFormat,
    channel_type: Type,
    w: u32,
    h: u32,
    count: u32,
    pixels: Option<&[u8]>,
  ) -> Result<(), Error> {
    let level = 0;
    let border = 0;

    self
      .0
      .tex_image_3d_with_opt_u8_array(
        target as _,
        level,
        internal_format as _,
        w as _,
        h as _,
        count as _,
        border,
        pixel_format as _,
        channel_type as _,
        pixels,
      )
      .map_err(|_err| self.error().unwrap_err())
  }

  #[inline]
  fn set_texture_sub_image_3d(
    &self,
    target: TextureTarget,
    pixel_format: TexturePixelFormat,
    channel_type: Type,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    h: u32,
    pixels: &[u8],
  ) -> Result<(), Error> {
    let level = 0;
    let depth = 1;

    self
      .0
      .tex_sub_image_3d_with_opt_u8_array(
        target as _,
        level,
        x as _,
        y as _,
        z as _,
        w as _,
        h as _,
        depth,
        pixel_format as _,
        channel_type as _,
        Some(pixels),
      )
      .map_err(|_err| self.error().unwrap_err())
  }

  #[inline]
  fn set_texture_filter(
    &self,
    target: TextureTarget,
    ty: TextureFilterType,
    filter: TextureFilter,
  ) {
    let () = self.0.tex_parameteri(target as _, ty as _, filter as _);
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_compare_mode(&self, target: TextureTarget, mode: TextureCompareMode) {
    let () = self.0.tex_parameteri(
      target as _,
      WebGl2RenderingContext::TEXTURE_COMPARE_MODE,
      mode as _,
    );
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_compare_func(&self, target: TextureTarget, func: Func) {
    let () = self.0.tex_parameteri(
      target as _,
      WebGl2RenderingContext::TEXTURE_COMPARE_FUNC,
      func as _,
    );
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_wrap(&self, target: TextureTarget, wrap: TextureWrap) {
    let () = self.0.tex_parameteri(
      target as _,
      WebGl2RenderingContext::TEXTURE_WRAP_S,
      wrap as _,
    );
    let () = self.0.tex_parameteri(
      target as _,
      WebGl2RenderingContext::TEXTURE_WRAP_T,
      wrap as _,
    );
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn generate_mipmaps(&self, target: TextureTarget) {
    let () = self.0.generate_mipmap(target as _);
    debug_assert_eq!(self.error(), Ok(()));
  }
}
