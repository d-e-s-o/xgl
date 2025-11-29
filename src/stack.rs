// Copyright (C) 2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;


/// A compile-time sized stack.
#[derive(Debug)]
pub(crate) struct Stack<T, const N: usize> {
  /// Our stack storage.
  ///
  /// The first element is always valid.
  stack: [MaybeUninit<T>; N],
  /// The index of the top-most element.
  top: usize,
}

impl<T, const N: usize> Stack<T, N> {
  /// Create a new stack and add the given initial element.
  pub const fn new_with(init: T) -> Self {
    let mut slf = Self {
      stack: [const { MaybeUninit::uninit() }; N],
      top: 0,
    };

    // TODO: Use `MaybeUninit::write` once stable in const.
    // SAFETY: We know that `slf.stack[slf.top]` represents storage for
    //         a valid object.
    let _ = unsafe { slf.stack[slf.top].as_mut_ptr().write(init) };
    slf
  }

  /// Push a new element onto the stack.
  ///
  /// The element is a copy of the currently top-most one.
  pub fn push(&mut self)
  where
    T: Clone,
  {
    let new = (*self).clone();

    self.top += 1;
    if self.top == N {
      panic!("reached stack limit of {N} elements")
    }

    let _ = self.stack[self.top].write(new);
  }

  /// Pop the top-most item from the stack.
  ///
  /// # Panics
  /// This function panics when called on the last (i.e., first)
  /// element.
  pub fn pop(&mut self) {
    if self.top == 0 {
      panic!("cannot pop initial element")
    }

    self.top -= 1;
  }
}

impl<T, const N: usize> Default for Stack<T, N>
where
  T: Default,
{
  #[inline]
  fn default() -> Self {
    Self::new_with(T::default())
  }
}

impl<T, const N: usize> Deref for Stack<T, N> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    // SAFETY: As per our invariant, the top-most element is always
    //         initialized.
    unsafe { self.stack[self.top].assume_init_ref() }
  }
}

impl<T, const N: usize> DerefMut for Stack<T, N> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    // SAFETY: As per our invariant, the top-most element is always
    //         initialized.
    unsafe { self.stack[self.top].assume_init_mut() }
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check basic workings of a `Stack`.
  #[test]
  fn ops() {
    let mut stack = Stack::<String, 5>::default();
    assert_eq!(*stack, "");

    *stack = "foobar".to_string();
    assert_eq!(*stack, "foobar");

    let () = stack.push();
    assert_eq!(*stack, "foobar");

    *stack = "baz".to_string();
    assert_eq!(*stack, "baz");

    let () = stack.pop();
    assert_eq!(*stack, "foobar");
  }

  /// Check that we panic as expected when popping the last element of a
  /// stack.
  #[test]
  #[should_panic = "cannot pop initial element"]
  fn empty_pop() {
    let mut stack = Stack::<usize, 32>::default();
    let () = stack.pop();
  }

  /// Check that we panic as expected when pushing to an already full
  /// stack.
  #[test]
  #[should_panic = "reached stack limit"]
  fn full_push() {
    let mut stack = Stack::<usize, 1>::default();
    let () = stack.push();
  }
}
