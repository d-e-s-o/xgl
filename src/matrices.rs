// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::stack::Stack;


/// A stack of matrices.
pub struct MatrixStack<M, const N: usize, L> {
  /// The matrix stack.
  stack: Stack<M, N>,
  /// The function used for actually loading a matrix.
  load_matrix: L,
}

impl<M, const N: usize, L> MatrixStack<M, N, L>
where
  L: FnMut(&M) + 'static,
{
  /// Create a new matrix stack.
  ///
  /// The provided function is used for "loading" the new matrix after a
  /// push or pop.
  pub fn new(mut load_matrix: L) -> Self
  where
    M: Default,
  {
    let default = M::default();
    let () = load_matrix(&default);

    Self {
      stack: Stack::new_with(default),
      load_matrix,
    }
  }

  /// Push a new matrix onto the stack.
  ///
  /// The matrix initially is a copy of the current top most one, but
  /// `f` is able to modify before it will be loaded.
  #[inline]
  pub fn push<F>(&mut self, f: F)
  where
    M: Clone,
    F: FnOnce(&mut M),
  {
    let () = self.stack.push();
    let () = f(&mut self.stack);
    let () = (self.load_matrix)(&mut self.stack);
  }

  /// Pop the currently active matrix and load the new top-most one.
  #[inline]
  pub fn pop(&mut self) {
    let () = self.stack.pop();
    let () = (self.load_matrix)(&self.stack);
  }
}

impl<M, const N: usize, L> Debug for MatrixStack<M, N, L> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("MatrixStack").finish()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::rc::Rc;
  use std::sync::atomic::AtomicIsize;
  use std::sync::atomic::Ordering;


  /// Test the push and pop functionality of our [`MatrixStack`] type.
  #[test]
  fn push_pop() {
    let top = Rc::new(AtomicIsize::new(0));
    let clone = Rc::clone(&top);
    let load = move |x: &isize| {
      clone.store(*x, Ordering::Relaxed);
    };

    let mut stack = MatrixStack::<isize, 16, _>::new(load);
    let () = stack.push(|x| {
      assert_eq!(*x, 0);
      *x = 1;
    });
    assert_eq!(top.load(Ordering::Relaxed), 1);

    let () = stack.push(|x| {
      assert_eq!(*x, 1);
      *x = 2;
    });
    assert_eq!(top.load(Ordering::Relaxed), 2);

    let () = stack.pop();
    assert_eq!(top.load(Ordering::Relaxed), 1);
    let () = stack.pop();
    assert_eq!(top.load(Ordering::Relaxed), 0);
  }
}
