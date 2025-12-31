Unreleased
----------
- Added `sys::Gl::set_uniform_1fv` method
- Added `sys::Type::Short` variant


0.2.0
-----
- Moved functionality into `sys` module
  - Introduced `sys::Gl` trait defining lowest level interface
- Introduced `MatrixStack` type
- Introduced `Shader` and `Program` types
- Introduced `Texture` type and supporting infrastructure
- Introduced `VertexArray` and `VertexBuffer` types and associated
  functionality
- Introduced `Framebuffer` type
- Made various `enum` types non-exhaustive
- Added `sys::VertexBufferUsage` enum and adjusted
  `sys::Context::set_vertex_buffer_data` to require an instance
- Added `sys::Context::set_vertex_buffer_sub_data` method
- Renamed `PrimitiveTy` trait to `BuiltinType`
- Renamed `DrawType` enum to `Primitive`


0.1.0
-----
- Initial release
