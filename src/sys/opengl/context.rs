// Copyright (C) 2025-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::error::Error as StdError;
use std::ffi::CString;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::mem::MaybeUninit;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ptr::null_mut;
use std::slice;

use crate::sys::BuiltinType;
use crate::sys::Gl;
use crate::sys::Sealed;

use super::gl;


#[derive(Debug)]
pub struct Program(u32);

#[derive(Debug)]
pub struct Shader(u32);

#[derive(Debug)]
pub struct Framebuffer(u32);

#[derive(Debug)]
pub struct Texture(u32);

#[derive(Debug)]
pub struct VertexArrayObject(u32);

#[derive(Debug)]
pub struct VertexBufferObject(u32);

#[derive(Debug)]
pub struct UniformLocation(i32);


#[derive(Eq, PartialEq)]
pub struct Error(u32);

impl Debug for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self, f)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let err = match self.0 {
      gl::INVALID_ENUM => "invalid enum",
      gl::INVALID_VALUE => "invalid value",
      gl::INVALID_OPERATION => "invalid operation",
      gl::OUT_OF_MEMORY => "out of memory",
      gl::INVALID_FRAMEBUFFER_OPERATION => "invalid framebuffer operation",
      _ => return write!(f, "OpenGL error: {:#x}", self.0),
    };
    write!(f, "OpenGL error: {err} ({:#x})", self.0)
  }
}

impl StdError for Error {}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Type {
  Float = gl::FLOAT,
  Short = gl::SHORT,
  UnsignedByte = gl::UNSIGNED_BYTE,
  UnsignedShort = gl::UNSIGNED_SHORT,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Capability {
  Blend = gl::BLEND,
  CullFace = gl::CULL_FACE,
  DepthTest = gl::DEPTH_TEST,
  FramebufferSRGB = gl::FRAMEBUFFER_SRGB,
  Multisample = gl::MULTISAMPLE,
  ScissorTest = gl::SCISSOR_TEST,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Func {
  LessOrEqual = gl::LEQUAL,
  Greater = gl::GREATER,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Factor {
  SrcAlpha = gl::SRC_ALPHA,
  OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum FrontFace {
  ClockWise = gl::CW,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum CullFace {
  Back = gl::BACK,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
  Lines = gl::LINES,
  Triangles = gl::TRIANGLES,
  TriangleFan = gl::TRIANGLE_FAN,
  TriangleStrip = gl::TRIANGLE_STRIP,
}


#[derive(Clone, Copy, Debug)]
pub struct ClearMask(u32);

#[expect(non_upper_case_globals)]
impl ClearMask {
  pub const ColorBuffer: Self = Self(gl::COLOR_BUFFER_BIT);
  pub const DepthBuffer: Self = Self(gl::DEPTH_BUFFER_BIT);
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
  pub const Complete: Self = Self(gl::FRAMEBUFFER_COMPLETE);
}

impl Display for FramebufferStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{:#x}", self.0)
  }
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
  Fragment = gl::FRAGMENT_SHADER,
  Vertex = gl::VERTEX_SHADER,
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
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum VertexBufferTarget {
  Array = gl::ARRAY_BUFFER,
  ElementArray = gl::ELEMENT_ARRAY_BUFFER,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum VertexBufferUsage {
  DynamicDraw = gl::DYNAMIC_DRAW,
  StaticDraw = gl::STATIC_DRAW,
  StreamDraw = gl::STREAM_DRAW,
}


#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TextureTarget {
  Texture2D = gl::TEXTURE_2D,
  Texture2DArray = gl::TEXTURE_2D_ARRAY,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TextureInternalFormat {
  Gray8 = gl::R8,
  Depth = gl::DEPTH_COMPONENT,
  RGB8 = gl::RGB8,
  SRGB8 = gl::SRGB8,
  RGBA8 = gl::RGBA8,
  SRGBA8 = gl::SRGB8_ALPHA8,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TexturePixelFormat {
  Gray = gl::RED,
  Depth = gl::DEPTH_COMPONENT,
  RGB = gl::RGB,
  RGBA = gl::RGBA,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TextureCompareMode {
  RefToTexture = gl::COMPARE_REF_TO_TEXTURE,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TextureWrap {
  ClampToEdge = gl::CLAMP_TO_EDGE,
  Repeat = gl::REPEAT,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TextureFilterType {
  Minimize = gl::TEXTURE_MIN_FILTER,
  Magnify = gl::TEXTURE_MAG_FILTER,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TextureFilter {
  Linear = gl::LINEAR,
  LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}


impl BuiltinType<Context> for u16 {
  fn as_type() -> Type {
    Type::UnsignedShort
  }
}


/// The OpenGL context in use.
///
/// A context is guaranteed to be cheaply cloneable.
#[derive(Clone, Debug, Default)]
pub struct Context {}

impl Context {
  fn check_program(&self, program: &Program, status_attrib: u32) -> Result<(), Vec<u8>> {
    let mut status = MaybeUninit::uninit();
    let () = unsafe { gl::GetProgramiv(program.0, status_attrib, status.as_mut_ptr()) };
    let status = unsafe { status.assume_init() };
    if status == i32::from(false) {
      let mut info_len = MaybeUninit::uninit();
      let () = unsafe { gl::GetProgramiv(program.0, gl::INFO_LOG_LENGTH, info_len.as_mut_ptr()) };
      let info_len = unsafe { info_len.assume_init() };

      let mut log = Vec::<u8>::with_capacity(info_len as _);
      let () = unsafe {
        gl::GetProgramInfoLog(program.0, info_len, null_mut(), log.as_mut_ptr().cast());
      };
      let () = unsafe { log.set_len(info_len as _) };
      Err(log)
    } else {
      Ok(())
    }
  }

  #[inline]
  fn set_uniform_matrices_impl(&self, location: &UniformLocation, matrices: &[[f32; 16]]) {
    let transpose = 0;
    let () = unsafe {
      gl::UniformMatrix4fv(
        location.0,
        matrices.len() as _,
        transpose,
        matrices.as_ptr().cast(),
      )
    };
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
  type VertexBufferUsage = VertexBufferUsage;

  type Framebuffer = Framebuffer;
  type Program = Program;
  type Shader = Shader;
  type Texture = Texture;
  type VertexArrayObject = VertexArrayObject;
  type VertexBufferObject = VertexBufferObject;

  type UniformLocation = UniformLocation;

  #[inline]
  fn error(&self) -> Result<(), Error> {
    let error = unsafe { gl::GetError() };
    if error == gl::NO_ERROR {
      Ok(())
    } else {
      Err(Error(error))
    }
  }

  #[inline]
  fn enable(&self, capability: Capability) {
    let () = unsafe { gl::Enable(capability as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn disable(&self, capability: Capability) {
    let () = unsafe { gl::Disable(capability as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_depth_func(&self, func: Func) {
    let () = unsafe { gl::DepthFunc(func as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_blend_func(&self, src_factor: Factor, dst_factor: Factor) {
    let () = unsafe { gl::BlendFunc(src_factor as _, dst_factor as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_front_face(&self, face: FrontFace) {
    let () = unsafe { gl::FrontFace(face as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_cull_face(&self, face: CullFace) {
    let () = unsafe { gl::CullFace(face as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_viewport(&self, x: i32, y: i32, w: i32, h: i32) {
    let () = unsafe { gl::Viewport(x, y, w, h) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
    let () = unsafe { gl::ClearColor(r, g, b, a) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_pixel_unpack_alignment(&self, alignment: u32) {
    let () = unsafe { gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn clear(&self, mask: ClearMask) {
    let () = unsafe { gl::Clear(mask.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn draw_arrays(&self, primitive: Primitive, count: i32) {
    let () = unsafe { gl::DrawArrays(primitive as _, 0, count) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn draw_arrays_instanced(&self, primitive: Primitive, count: i32, instance_count: i32) {
    let () = unsafe { gl::DrawArraysInstanced(primitive as _, 0, count, instance_count) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn draw_elements<T>(&self, primitive: Primitive, count: i32)
  where
    T: BuiltinType<Self>,
  {
    let () = unsafe { gl::DrawElements(primitive as _, count, T::as_type() as _, null_mut()) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn create_framebuffer(&self) -> Result<Framebuffer, Error> {
    let mut fbo = 0;
    let () = unsafe { gl::GenFramebuffers(1, &mut fbo) };
    let () = self.error()?;
    Ok(Framebuffer(fbo))
  }

  #[inline]
  fn delete_framebuffer(&self, fbo: &Framebuffer) {
    let () = unsafe { gl::DeleteFramebuffers(1, &fbo.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_framebuffer(&self, fbo: Option<&Framebuffer>) {
    let fbo = fbo.map(|fbo| fbo.0).unwrap_or(0);
    let () = unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, fbo) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_framebuffer_depth_texture(&self, texture_target: TextureTarget, texture: &Texture) {
    let mipmap_level = 0;
    let () = unsafe {
      gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        texture_target as _,
        texture.0,
        mipmap_level,
      )
    };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn unset_draw_buffer(&self) {
    let () = unsafe { gl::DrawBuffer(gl::NONE) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn unset_read_buffer(&self) {
    let () = unsafe { gl::ReadBuffer(gl::NONE) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn check_framebuffer_status(&self) -> FramebufferStatus {
    let status = unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) };
    FramebufferStatus(status)
  }

  #[inline]
  fn create_shader(&self, ty: ShaderType) -> Option<Shader> {
    let shader = unsafe { gl::CreateShader(ty as _) };
    if shader != 0 {
      Some(Shader(shader))
    } else {
      None
    }
  }

  #[inline]
  fn delete_shader(&self, shader: &Shader) {
    let () = unsafe { gl::DeleteShader(shader.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_shader_source(&self, shader: &Shader, source: &str) {
    let srcs = [source.as_ptr().cast::<u8>()];
    let src_lens = [source.len() as i32];
    let () = unsafe {
      gl::ShaderSource(
        shader.0,
        srcs.len() as _,
        srcs.as_ptr().cast(),
        src_lens.as_ptr(),
      )
    };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn compile_shader(&self, shader: &Shader) -> Result<(), Vec<u8>> {
    let () = unsafe { gl::CompileShader(shader.0) };
    let mut status = MaybeUninit::uninit();
    let () = unsafe { gl::GetShaderiv(shader.0, gl::COMPILE_STATUS, status.as_mut_ptr()) };
    let status = unsafe { status.assume_init() };
    if status == i32::from(false) {
      let mut info_len = MaybeUninit::uninit();
      let () = unsafe { gl::GetShaderiv(shader.0, gl::INFO_LOG_LENGTH, info_len.as_mut_ptr()) };
      let info_len = unsafe { info_len.assume_init() };

      let mut log = Vec::<u8>::with_capacity(info_len as _);
      let () =
        unsafe { gl::GetShaderInfoLog(shader.0, info_len, null_mut(), log.as_mut_ptr().cast()) };
      let () = unsafe { log.set_len(info_len as _) };
      Err(log)
    } else {
      Ok(())
    }
  }

  #[inline]
  fn attach_shader(&self, program: &Program, shader: &Shader) {
    let () = unsafe { gl::AttachShader(program.0, shader.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn detach_shader(&self, program: &Program, shader: &Shader) {
    let () = unsafe { gl::DetachShader(program.0, shader.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn create_program(&self) -> Option<Program> {
    let program = unsafe { gl::CreateProgram() };
    if program != 0 {
      Some(Program(program))
    } else {
      None
    }
  }

  #[inline]
  fn delete_program(&self, program: &Program) {
    let () = unsafe { gl::DeleteProgram(program.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  fn link_program(&self, program: &Program) -> Result<(), Vec<u8>> {
    let () = unsafe { gl::LinkProgram(program.0) };
    let () = self.check_program(program, gl::LINK_STATUS)?;
    Ok(())
  }

  fn validate_program(&self, program: &Program) -> Result<(), Vec<u8>> {
    let () = unsafe { gl::ValidateProgram(program.0) };
    let () = self.check_program(program, gl::VALIDATE_STATUS)?;
    Ok(())
  }

  #[inline]
  fn use_program(&self, program: &Program) {
    let () = unsafe { gl::UseProgram(program.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn attrib_location(&self, program: &Program, attrib: &str) -> Option<u32> {
    // SANITY: Callers have to ensure there are no NUL bytes inside
    //         `attrib`.
    let cattrib = CString::new(attrib).unwrap();
    let idx = unsafe { gl::GetAttribLocation(program.0, cattrib.as_ptr()) };
    if idx >= 0 {
      Some(u32::try_from(idx).unwrap())
    } else {
      None
    }
  }

  #[inline]
  fn uniform_location(&self, program: &Program, uniform: &str) -> Option<UniformLocation> {
    // SANITY: Callers have to ensure there are no NUL bytes inside
    //         `uniform`.
    let cuniform = CString::new(uniform).unwrap();
    let idx = unsafe { gl::GetUniformLocation(program.0, cuniform.as_ptr()) };
    if idx >= -1 {
      Some(UniformLocation(idx))
    } else {
      None
    }
  }

  #[inline]
  fn uniform_fv<const N: usize>(&self, program: &Program, location: &UniformLocation) -> [f32; N] {
    let mut data = MaybeUninit::<[f32; N]>::uninit();
    let () = unsafe { gl::GetUniformfv(program.0, location.0, data.as_mut_ptr().cast()) };
    // SANITY: We know that program and location are valid, so the above
    //         call should be infallible.
    assert_eq!(self.error(), Ok(()));

    // SAFETY: On success, `data` will be written by `GetUniformfv`.
    unsafe { data.assume_init() }
  }

  #[inline]
  fn set_uniform_1i(&self, location: &UniformLocation, data: i32) {
    let () = unsafe { gl::Uniform1i(location.0, data) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_1ui(&self, location: &UniformLocation, data: u32) {
    let () = unsafe { gl::Uniform1ui(location.0, data) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_1iv(&self, location: &UniformLocation, data: &[i32]) {
    let () = unsafe { gl::Uniform1iv(location.0, data.len() as _, data.as_ptr()) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_1fv(&self, location: &UniformLocation, data: &[f32]) {
    let () = unsafe { gl::Uniform1fv(location.0, data.len() as _, data.as_ptr()) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_3f(&self, location: &UniformLocation, data: &[f32; 3]) {
    let () = unsafe { gl::Uniform3fv(location.0, 1, data.as_ptr()) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_4f(&self, location: &UniformLocation, data: &[f32; 4]) {
    let () = unsafe { gl::Uniform4fv(location.0, 1, data.as_ptr()) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_uniform_4fv(&self, location: &UniformLocation, data: &[[f32; 4]]) {
    let () = unsafe { gl::Uniform4fv(location.0, data.len() as _, data.as_ptr().cast()) };
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
    let mut vbo = 0;
    let () = unsafe { gl::GenBuffers(1, &mut vbo) };
    let () = self.error()?;
    Ok(VertexBufferObject(vbo))
  }

  #[inline]
  fn delete_vertex_buffer(&self, vbo: &VertexBufferObject) {
    let () = unsafe { gl::DeleteBuffers(1, &vbo.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_vertex_buffer(&self, target: VertexBufferTarget, vbo: Option<&VertexBufferObject>) {
    let vbo = vbo.map(|vbo| vbo.0).unwrap_or(0);
    let () = unsafe { gl::BindBuffer(target as _, vbo) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_vertex_buffer_data<T>(
    &self,
    target: VertexBufferTarget,
    usage: VertexBufferUsage,
    data: &[T],
  ) {
    let () = unsafe {
      gl::BufferData(
        target as _,
        size_of_val(data) as _,
        data.as_ptr().cast(),
        usage as _,
      )
    };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_vertex_buffer_sub_data<T>(&self, target: VertexBufferTarget, data: &[T], offset: i32) {
    let () = unsafe {
      gl::BufferSubData(
        target as _,
        offset as _,
        size_of_val(data) as _,
        data.as_ptr().cast(),
      )
    };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn create_vertex_array(&self) -> Result<VertexArrayObject, Error> {
    let mut vao = 0;
    let () = unsafe { gl::GenVertexArrays(1, &mut vao) };
    let () = self.error()?;
    Ok(VertexArrayObject(vao))
  }

  #[inline]
  fn delete_vertex_array(&self, vao: &VertexArrayObject) {
    let () = unsafe { gl::DeleteVertexArrays(1, &vao.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_vertex_array(&self, vao: Option<&VertexArrayObject>) {
    let vao = vao.map(|vao| vao.0).unwrap_or(0);
    let () = unsafe { gl::BindVertexArray(vao) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn enable_vertex_attrib_array(&self, idx: u32) {
    let () = unsafe { gl::EnableVertexAttribArray(idx) };
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
    let () = unsafe {
      gl::VertexAttribPointer(
        idx,
        size,
        ty as _,
        normalize as u8,
        stride,
        offset as *const _,
      )
    };
    debug_assert_eq!(self.error(), Ok(()));
  }

  fn create_texture(&self) -> Result<Texture, Error> {
    let mut id = 0;
    let () = unsafe { gl::GenTextures(1, &mut id) };
    let () = self.error()?;
    Ok(Texture(id))
  }

  #[inline]
  fn delete_texture(&self, texture: &Texture) {
    let () = unsafe { gl::DeleteTextures(1, &texture.0) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn bind_texture(&self, target: TextureTarget, texture: Option<&Texture>) {
    let texture = texture.map(|texture| texture.0).unwrap_or(0);
    let () = unsafe { gl::BindTexture(target as _, texture) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_active_texture_unit(&self, unit: u32) {
    let () = unsafe { gl::ActiveTexture(gl::TEXTURE0 + unit) };
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

    let () = unsafe {
      gl::TexImage2D(
        target as _,
        level,
        internal_format as _,
        w as _,
        h as _,
        border,
        pixel_format as _,
        channel_type as _,
        pixels
          .map(|pixels| pixels.as_ptr().cast())
          .unwrap_or_default(),
      )
    };
    let () = self.error()?;
    Ok(())
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

    let () = unsafe {
      gl::TexImage3D(
        target as _,
        level,
        internal_format as _,
        w as _,
        h as _,
        count as _,
        border,
        pixel_format as _,
        channel_type as _,
        pixels
          .map(|pixels| pixels.as_ptr().cast())
          .unwrap_or_default(),
      )
    };
    let () = self.error()?;
    Ok(())
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

    let () = unsafe {
      gl::TexSubImage3D(
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
        pixels.as_ptr().cast(),
      )
    };
    let () = self.error()?;
    Ok(())
  }

  #[inline]
  fn set_texture_filter(
    &self,
    target: TextureTarget,
    ty: TextureFilterType,
    filter: TextureFilter,
  ) {
    let () = unsafe { gl::TexParameteri(target as _, ty as _, filter as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_compare_mode(&self, target: TextureTarget, mode: TextureCompareMode) {
    let () = unsafe { gl::TexParameteri(target as _, gl::TEXTURE_COMPARE_MODE, mode as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_compare_func(&self, target: TextureTarget, func: Func) {
    let () = unsafe { gl::TexParameteri(target as _, gl::TEXTURE_COMPARE_FUNC, func as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn set_texture_wrap(&self, target: TextureTarget, wrap: TextureWrap) {
    let () = unsafe { gl::TexParameteri(target as _, gl::TEXTURE_WRAP_S, wrap as _) };
    let () = unsafe { gl::TexParameteri(target as _, gl::TEXTURE_WRAP_T, wrap as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }

  #[inline]
  fn generate_mipmaps(&self, target: TextureTarget) {
    let () = unsafe { gl::GenerateMipmap(target as _) };
    debug_assert_eq!(self.error(), Ok(()));
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check that the `Debug` impl for the [`Error`] type works as
  /// expected.
  #[test]
  fn error_debug_impl() {
    let err = Error(gl::INVALID_VALUE);
    let s = format!("{err:?}");
    assert_eq!(s, "OpenGL error: invalid value (0x501)");

    let err = Error(0xfff);
    let s = format!("{err:?}");
    assert_eq!(s, "OpenGL error: 0xfff");
  }
}
