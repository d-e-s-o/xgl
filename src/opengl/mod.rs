// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[expect(clippy::undocumented_unsafe_blocks)]
mod context;
#[expect(non_upper_case_globals, dead_code)]
#[path = "bindings.rs"]
mod gl;

pub use context::Capability;
pub use context::ClearMask;
pub use context::Context;
pub use context::CullFace;
pub use context::DrawType;
pub use context::Factor;
pub use context::Framebuffer;
pub use context::FramebufferStatus;
pub use context::FrontFace;
pub use context::Func;
pub use context::Program;
pub use context::Shader;
pub use context::ShaderType;
pub use context::Texture;
pub use context::TextureCompareMode;
pub use context::TextureFilter;
pub use context::TextureFilterType;
pub use context::TextureInternalFormat;
pub use context::TexturePixelFormat;
pub use context::TextureTarget;
pub use context::TextureWrap;
pub use context::Type;
pub use context::UniformLocation;
pub use context::VertexArrayObject;
pub use context::VertexBufferObject;
pub use context::VertexBufferTarget;


/// Retrieve the OpenGL version to use.
pub const fn version() -> (u8, u8, Option<&'static str>) {
  (3, 3, None)
}
