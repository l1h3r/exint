use std::str;

pub const fn trim(string: &'static str) -> &'static str {
  let mut bytes: &[u8] = string.as_bytes();

  while let [b' ' | b'\t' | b'\n', tail @ ..] = bytes {
    bytes = tail;
  }

  while let [head @ .., b' ' | b'\t' | b'\n'] = bytes {
    bytes = head;
  }

  unsafe { str::from_utf8_unchecked(bytes) }
}
