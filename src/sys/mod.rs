// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg(not(target_arch = "wasm32"))]
mod opengl;
#[cfg(target_arch = "wasm32")]
mod webgl;

#[cfg(not(target_arch = "wasm32"))]
pub use self::opengl::*;
#[cfg(target_arch = "wasm32")]
pub use self::webgl::*;
