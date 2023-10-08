use im::vector;
use std::fmt;

pub struct SvgStack {
  vec: vector::Vector<String>,
  x: i32,
  y: i32,
  w: i32,
  h: i32,
  closed: bool,
  child_x: vector::Vector<i32>,
  child_y: vector::Vector<i32>
}

impl SvgStack {
  pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
    // let str = format!("<svg x='{}' y='{}' width='{}' height='{}'>", x, y, w, h);
    let mut vec = vector::Vector::<String>::new();
    {
      vec.push_back("</svg>".to_string());
      // vec.push_back(str);
    }
    Self { vec, x, y, w, h, closed: false, child_x: vec![0].into(), child_y: vec![0].into() }
  }
  pub fn close(&mut self) {
    if self.closed {
      return;
    }
    self.closed = true;
    self.vec.push_back(format!("<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' version='1.1' x='{}' y='{}' width='{}' height='{}'>", self.x, self.y, self.w, self.h))
  }
  pub fn len(&mut self) -> usize {
    self.vec.len()
  }
  pub fn pop(&mut self) -> String {
    self.vec.pop_back().unwrap()
  }
  pub fn push(&mut self, s: String) {
    self.vec.push_back(s);
  }
  pub fn end_child(&mut self, x: i32, y: i32) {
    let child_x = self.child_x.pop_back().unwrap();
    let child_y = self.child_y.pop_back().unwrap();
    println!("end_child: child.x={},child.y={} x={},y={} . svg = {}",child_x, child_y, x, y, format!("<svg x='{}' y='{}' width='{}' height='{}'>", child_x, child_y, x-child_x, y-child_y));
    self.vec.push_back(format!("<svg x='{}' y='{}' width='{}' height='{}'>", child_x, child_y, x-child_x, y-child_y))
  }
  pub fn start_child(&mut self, x: i32, y: i32) {
    println!("start_child: child_x: {}, child_y: {}",x, y);
    self.child_x.push_back(x);
    self.child_y.push_back(y);
    self.vec.push_back("</svg>".to_string())
  }
}

// to_string trait
impl fmt::Display for SvgStack {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let mut str = "";
    let mut clone = self.vec.clone();
    loop {
      if clone.len() == 0 {
        break;
      }
    //for chunk in &self.vec {
      fmt.write_str(str)?;
      fmt.write_str(/*chunk*/&format!("{}", clone.pop_back().unwrap()).to_owned())?;
      str = "";
    }
    Ok(())
  }
}
