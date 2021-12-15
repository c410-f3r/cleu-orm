// https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html

use core::{hash::Hasher, ops::BitXor};

const K: usize = 0x517cc1b727220a95;

#[derive(Default)]
pub(crate) struct FxHasher(usize);

impl Hasher for FxHasher {
  #[allow(
    // It does not matter much since this structure is a hasher
    clippy::as_conversions
  )]
  #[inline]
  fn finish(&self) -> u64 {
    self.0 as _
  }

  #[inline]
  fn write(&mut self, bytes: &[u8]) {
    for byte in bytes.iter().copied() {
      let into: usize = byte.into();
      self.0 = self.0.rotate_left(5).bitxor(into).wrapping_mul(K);
    }
  }
}
