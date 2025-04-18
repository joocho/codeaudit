fn main() {
  let a = Some(5);
  let b = None;

  let a2 = a.unwrap();
  let a3 = a.expect("A was missing");

  let b2 = b.expect("B was missing");

  let b3 = b.unwrap_or("fallback value");
}