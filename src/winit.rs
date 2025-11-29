// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::num::NonZeroU32;

use glutin::config::Config;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
use glutin::context::GlProfile;
use glutin::context::NotCurrentGlContext as _;
use glutin::context::PossiblyCurrentContext;
use glutin::context::Version;
use glutin::display::Display;
use glutin::display::DisplayApiPreference;
use glutin::display::GlDisplay as _;
use glutin::surface::Surface;
use glutin::surface::SurfaceAttributesBuilder;
use glutin::surface::WindowSurface;

use winit::event_loop::EventLoop;
use winit::platform::x11::register_xlib_error_hook;
use winit::platform::x11::EventLoopBuilderExtX11 as _;
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::raw_window_handle::HasWindowHandle as _;
use winit::raw_window_handle::RawDisplayHandle;
use winit::window::Window as WinitWindow;

use crate::sys;


/// An OpenGL context.
#[derive(Debug)]
struct Context {
  /// The OpenGL surface that is used for rendering.
  _surface: Surface<WindowSurface>,
  /// The OpenGL context used for double buffering.
  _context: PossiblyCurrentContext,
}

impl Context {
  /// Create proper [`Display`] object and an associated [`Config`] from
  /// a system display handle.
  fn create_display_and_config(display_handle: RawDisplayHandle) -> (Display, Config) {
    let preference = DisplayApiPreference::Glx(Box::new(register_xlib_error_hook));
    // SAFETY: `display_handle` is a valid platform display and valid
    //         for the entire program.
    let display =
      unsafe { Display::new(display_handle, preference) }.expect("failed to create display object");
    let template = ConfigTemplateBuilder::new().build();
    let display_clone = display.clone();
    // SAFETY: Unclear.
    let mut configs = unsafe { display_clone.find_configs(template) }
      .expect("failed to find OpenGL configurations");
    let config = configs
      .next()
      .expect("failed to find any OpenGL configuration");

    (display, config)
  }

  /// Create a new OpenGL context on the given window.
  fn new(display: &Display, config: &Config, window: &WinitWindow) -> Self {
    let window_handle = window
      .window_handle()
      .expect("failed to retrieve window handle");
    let raw_window_handle = window_handle.into();
    let size = window.inner_size();
    let phys_w = NonZeroU32::new(size.width).expect("window has no width");
    let phys_h = NonZeroU32::new(size.height).expect("window has no height");

    let (major, minor, _suffix) = sys::version();
    let context_attributes = ContextAttributesBuilder::new()
      .with_context_api(ContextApi::OpenGl(Some(Version::new(major, minor))))
      .with_profile(GlProfile::Core)
      .build(Some(raw_window_handle));
    let attrs =
      SurfaceAttributesBuilder::<WindowSurface>::default().build(raw_window_handle, phys_w, phys_h);
    // SAFETY: Unclear.
    let surface = unsafe { display.create_window_surface(config, &attrs) }
      .expect("failed to create window surface");
    // SAFETY: Unclear.
    let context = unsafe { display.create_context(config, &context_attributes) }
      .expect("failed to create context")
      .make_current(&surface)
      .expect("failed to make context current");

    Self {
      _surface: surface,
      _context: context,
    }
  }
}


/// A window with a proper graphics context.
#[derive(Debug)]
struct Window {
  /// The underlying `winit` window.
  _window: WinitWindow,
  /// The OpenGL context associated with the window.
  _context: Context,
}

impl Window {
  /// Create a new window.
  #[expect(deprecated)]
  fn new(event_loop: &EventLoop<()>) -> Self {
    let display_handle = event_loop.display_handle().unwrap();
    let (display, config) = Context::create_display_and_config(display_handle.into());
    let mut attributes = WinitWindow::default_attributes();
    attributes.visible = false;
    let window = event_loop
      .create_window(attributes)
      .expect("failed to create window");
    let context = Context::new(&display, &config, &window);

    Self {
      _window: window,
      _context: context,
    }
  }
}


/// A test helper to run a function with an OpenGL window (and context)
/// present.
///
/// # Notes
/// You most likely want to use this function in a separate process,
/// because of the effectively global OpenGL state.
pub(crate) fn with_opengl_context<F>(f: F)
where
  F: FnOnce(),
{
  let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
  let _window = Window::new(&event_loop);

  f()
}
