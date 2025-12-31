// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::ops::Deref;

use anyhow::anyhow;
use anyhow::Context as _;
use anyhow::Result;

use crate::sys;
use crate::sys::Gl as _;


/// A shader object.
#[derive(Debug)]
pub struct Shader {
  /// The GL context.
  context: sys::Context,
  /// The shader.
  shader: sys::Shader,
}

impl Shader {
  /// Retrieve the GL shading language version to use.
  ///
  /// We establish the contract of the shading language version being
  /// aligned with the corresponding OpenGL version, though that may not
  /// necessarily be a requirement.
  pub fn glsl_version() -> String {
    let (major, minor, suffix) = sys::version();
    let version = format!(
      "{major}{minor}0{}",
      suffix
        .map(|suffix| format!(" {suffix}"))
        .unwrap_or_default()
    );
    // Since OpenGL 3.3 both GLSL and OpenGL are versioned in-sync. But
    // for ES it's already at 3.0.
    // See https://en.wikipedia.org/w/index.php?title=OpenGL_Shading_Language&oldid=1269975510#Versions
    assert!(
      (major, minor) >= (3, if suffix == Some("es") { 0 } else { 3 }),
      "unsupported OpenGL version: {version:?}"
    );
    version
  }

  pub fn new(
    shader_type: sys::ShaderType,
    shader_file: &str,
    context: &sys::Context,
  ) -> Result<Self> {
    let shader = context
      .create_shader(shader_type)
      .context("failed to create shader object")?;
    let slf = Self {
      context: context.clone(),
      shader,
    };

    let () = context.set_shader_source(&slf.shader, shader_file);
    let () = context.compile_shader(&slf.shader).map_err(|log| {
      // TODO: Should use `String::from_utf8_lossy_owned` once stable.
      let log = String::from_utf8_lossy(log.as_slice());
      anyhow!("failed to compile {} shader: {log}", shader_type.as_str())
    })?;

    Ok(slf)
  }
}

impl Deref for Shader {
  type Target = sys::Shader;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.shader
  }
}

impl Drop for Shader {
  #[inline]
  fn drop(&mut self) {
    let () = self.context.delete_shader(&self.shader);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use test_fork::fork;

  use crate::winit::with_opengl_context;


  /// Check that we can create a vertex shader.
  #[fork]
  #[test]
  fn vertex_shader_creation() {
    let shader_code = format!(
      r#"#version {glsl_version} core

in vec4 position;

void main() {{
   gl_Position = position;
}}
"#,
      glsl_version = Shader::glsl_version()
    );

    with_opengl_context(|| {
      let gl_context = sys::Context::default();
      let _shader = Shader::new(sys::ShaderType::Vertex, &shader_code, &gl_context).unwrap();
    })
  }

  /// Check that we can create a fragment shader.
  #[fork]
  #[test]
  fn fragment_shader_creation() {
    let shader_code = format!(
      r#"#version {glsl_version} core

out vec4 color;

void main() {{
   color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}}
"#,
      glsl_version = Shader::glsl_version()
    );

    with_opengl_context(|| {
      let gl_context = sys::Context::default();
      let _shader = Shader::new(sys::ShaderType::Fragment, &shader_code, &gl_context).unwrap();
    })
  }

  /// Make sure that we correctly fail creation of a shader when invalid
  /// code is provided.
  #[fork]
  #[test]
  fn shader_creation_failure() {
    let shader_code = r#"
not a valid identifier here
"#;

    with_opengl_context(|| {
      let gl_context = sys::Context::default();
      let err = Shader::new(sys::ShaderType::Vertex, shader_code, &gl_context).unwrap_err();
      // Note: the error messages are not standardized, so we may need
      //       to check something else for different OpenGL
      //       implementations.
      assert!(err.to_string().contains("syntax error"), "{err:#}");
    })
  }
}
