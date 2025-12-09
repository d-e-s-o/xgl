// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::ops::Deref;

use anyhow::anyhow;
use anyhow::Context as _;
use anyhow::Result;

use crate::sys;
use crate::Shader;


/// A program object.
#[derive(Debug)]
pub struct Program {
  /// The OpenGL context.
  context: sys::Context,
  /// The program.
  program: sys::Program,
}

impl Program {
  pub fn new(shaders: &[Shader], context: &sys::Context) -> Result<Self> {
    let program = context
      .create_program()
      .context("failed to create program object")?;
    let slf = Self {
      context: context.clone(),
      program,
    };

    let () = shaders
      .iter()
      .for_each(|shader| context.attach_shader(&slf.program, shader));

    let () = context.link_program(&slf.program).map_err(|log| {
      // TODO: Should use `String::from_utf8_lossy_owned` once stable.
      let log = String::from_utf8_lossy(log.as_slice());
      anyhow!("failed to link program: {log}")
    })?;

    let () = shaders
      .iter()
      .for_each(|shader| context.detach_shader(&slf.program, shader));

    // Also perform some basic validation to check whether the program
    // could actually execute. In a more complex program we may want to
    // do that at more points in time (potentially before every "draw"
    // call).
    let () = context.validate_program(&slf.program).map_err(|log| {
      // TODO: Should use `String::from_utf8_lossy_owned` once stable.
      let log = String::from_utf8_lossy(log.as_slice());
      anyhow!("failed to validate program: {log}")
    })?;

    Ok(slf)
  }

  pub fn query_attrib_location(&self, attrib: &str) -> Result<u32> {
    let idx = self
      .context
      .attrib_location(&self.program, attrib)
      .with_context(|| format!("failed to query `{attrib}` attribute location"))?;
    Ok(idx)
  }

  pub fn query_uniform_location(&self, uniform: &str) -> Result<sys::UniformLocation> {
    let location = self
      .context
      .uniform_location(&self.program, uniform)
      .with_context(|| format!("failed to query `{uniform}` uniform location"))?;
    Ok(location)
  }

  #[inline]
  pub fn bind(&self) {
    let () = self.context.use_program(&self.program);
  }
}

impl Deref for Program {
  type Target = sys::Program;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.program
  }
}

impl Drop for Program {
  #[inline]
  fn drop(&mut self) {
    let () = self.context.delete_program(&self.program);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use test_fork::fork;

  use crate::winit::with_opengl_context;


  /// Check that we can create a program.
  #[fork]
  #[test]
  fn program_creation() {
    let vertex_shader = format!(
      r#"
#version {glsl_version}

in vec4 position;

void main() {{
   gl_Position = position;
}}
"#,
      glsl_version = Shader::glsl_version()
    );

    let fragment_shader = format!(
      r#"
#version {glsl_version}

out vec4 color;

void main() {{
   color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}}
"#,
      glsl_version = Shader::glsl_version()
    );

    with_opengl_context(|| {
      let gl_context = sys::Context::default();
      let vertex_shader =
        Shader::new(sys::ShaderType::Vertex, &vertex_shader, &gl_context).unwrap();
      let fragment_shader =
        Shader::new(sys::ShaderType::Fragment, &fragment_shader, &gl_context).unwrap();
      let program = Program::new(&[vertex_shader, fragment_shader], &gl_context).unwrap();

      assert!(program.query_attrib_location("position").is_ok());
      assert!(program.query_attrib_location("foobar").is_err());
    })
  }
}
