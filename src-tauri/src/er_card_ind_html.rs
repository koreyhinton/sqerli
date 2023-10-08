use std::f64::consts::PI;

use crate::rel_map::{Point, RelTerminiType};

/*

  |Y
x | z
 \|/
  v
  y

Y: arrow line
x: ccw_pt
y: arrow_pt
z: cw_pt

how to make cw_pt?
  Let Y = arrow line
  Let y = arrow point
  1. create a new line that is the same as Y except unit size extending from y
  2. rotate new line in - direction pivoting on y (e.g., rotate -45deg)
  Let Z = new line
  Let z = non-pivoted end point of the new line
  cw_pt (clockwise point) is z

how to make ccw_pt?
  Let Y = arrow line
  Let y = arrow point
  1. create a new line that is the same as Y except unit size extending from y
  2. rotate new line in + direction pivoting on y (e.g., rotate +45deg)
  Let X = new line
  Let x = non-pivoted end point of the new line
  ccw_pt (counterclockwise point) is x

*/

pub struct ArrowV {
  pub arrow_pt: Point,
  pub cw_pt: Point,
  pub ccw_pt: Point,
  retract_pt: Point  // private
}

fn rotate(cx: f64, cy: f64, x: f64, y: f64, angle: f64) -> Point {
  let rad = (PI / (180 as f64)) * angle;
  let cos = f64::cos(rad);
  let sin = f64::sin(rad);
  Point { 
    x: ((cos * (x - cx)) + (sin * (y - cy)) + cx) as i32 /*as usize*/,
    y: ((cos * (y - cy)) - (sin * (x - cx)) + cy) as i32/*as usize*/
  }
}

fn arrow_point(pt1: Point, pt2: Point, angle: f64, len: f64, sign: i32) -> Point {
  let pt_a = &mut Point { x: 0, y: 0 };
  let pt_b = pt2.clone();
  println!("({}-{})/({}-{})", pt2.y, pt1.y, pt2.x, pt1.x);
  let m = (pt2.y-pt1.y)/(pt2.x-pt1.x);
  let b = pt2.y - m*pt2.x;
  let mut dir = 1 as i32;
  if pt2.x > pt1.x {
    dir = -1;
  }
  pt_a.x = pt2.x;
  pt_a.x += ((dir) *1 as i32)/* as usize*/;
  pt_a.y = m*pt_a.x + b;
  let mut d = distf(pt_a.x, pt_a.y, pt2.x, pt2.y);
  let max_iter = 30;
  let mut i = 0;
  while d < len {
    pt_a.x += (dir*1); //  as usize;
    pt_a.y = m*pt_a.x + b;
    d = distf(pt_a.x, pt_a.y, pt2.x, pt2.y);
    i += 1;
    if i>max_iter {
      break;
    }
  }
  rotate(pt_b.x as f64, pt_b.y as f64, pt_a.x as f64, pt_a.y as f64, sign as f64 * angle)
}

fn distf(x1: i32, x2: i32, y1: i32, y2: i32) -> f64 {
  ((
    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)
  ) as f64).sqrt()
} 

fn shrink_pt(shrink_pt: Point, dir_pt: Point, dist: usize) -> Point {
  let ydiff = (shrink_pt.y - dir_pt.y).abs(); //<usize as TryInto<i32>>::try_into(shrink_pt.y - dir_pt.y).unwrap().abs();
  let xdiff = (shrink_pt.x - dir_pt.x).abs(); //<usize as TryInto<i32>>::try_into(shrink_pt.x - dir_pt.x).unwrap().abs();
 
  if xdiff > dist.try_into().unwrap() {
    for x in 1..xdiff { //<i32 as TryInto<usize>>::try_into(xdiff).unwrap() {
      let mut x1 = dir_pt.x;
      let mut x2 = shrink_pt.x;
      if dir_pt.x > shrink_pt.x {
        x2 += x;
      } else {
        x2 -= x;
      }
      let y1 = dir_pt.y;
      let y2 = shrink_pt.y;
      if distf(x1, x2, y1, y2) >= dist as f64 {
        return Point { x: x2, y: y2 }
      }
    }
  }
  else if ydiff > dist.try_into().unwrap() {
    for y in 1..ydiff { //<i32 as TryInto<usize>>::try_into(ydiff).unwrap() {
      let mut y1 = dir_pt.y;
      let mut y2 = shrink_pt.y;
      if dir_pt.y > shrink_pt.y {
        y2 += y;
      } else {
        y2 -= y;
      }
      let x1 = dir_pt.x;
      let x2 = shrink_pt.x;
      if distf(x1, x2, y1, y2) >= dist as f64 {
        return Point { x: x2, y: y2 }
      }
    }
  }
  return Point{x: shrink_pt.x + 10,y: shrink_pt.y + 10};//should NOT happen, but if it does needs to be visibly different point from shrink point 
}

impl ArrowV {

  // instead have it just do a default size and degrees that won't be customized
  // and what we need is to have it save the retract point and add this function to make
  // drawing a 2nd arrow head easy!:
  pub fn retracted_clone(&self) -> Self {

    // d = sqrt( (x2-x1)*(x2-x1) + (y2-y1)*(y2-y1) )
    // d * d = (x2-x1)*(x2-x1) + (y2-y1)*(y2-y1)

    // y = mx + b
    //let m = (self.arrow_pt.y - self.retract_pt.y) / (self.arrow_pt.x - self.retract_pt.x);
    //let b = self.arrow_pt.y - m * self.arrow_pt.x;
    let new_arrow_pt = shrink_pt(self.arrow_pt.clone(), self.retract_pt.clone(), 2);
    let new_arrow_x = new_arrow_pt.x;
    let new_arrow_y = new_arrow_pt.y;
    /*let mut new_x: usize = self.arrow_pt.x + 2;
    if self.arrow_pt.x > self.retract_pt.x {
      new_x = self.arrow_pt.x - 2;
    }
    let new_y: usize = m * new_x + b;*/
    
    // TODO!
    // FIX THIS: point calculation is incorrect
    let cw_pt = arrow_point(new_arrow_pt.clone(), self.retract_pt.clone(), 45 as f64, 10 as f64, 1);
    let ccw_pt = arrow_point(new_arrow_pt, self.retract_pt.clone(), 45 as f64, 10 as f64, -1);
    Self { arrow_pt: Point { x: new_arrow_x, y: new_arrow_y}, cw_pt, ccw_pt, retract_pt: self.retract_pt.clone() }
  }

  // No no.ignore this:
  // The constructor will assign ccw and cw points that are likely to be wrong values.
  // This is for 2 reasons.
  // 1. so construction will be fast.
  // 2. covers edge cases for right-angled point
  pub fn new(discard_pt: Point, arrow_pt: Point) -> Self {
    // TODO!
    // FIX THIS: point calculation is incorrect
  
    // right-angle edge case (which uses a different algorithm then set_line_angle).
    // discard_pt is used only for calculating cw_pt and ccw_pt
    let cw_pt: Point = arrow_point(arrow_pt.clone(), /*discard_pt.clone()*/shrink_pt(arrow_pt.clone(), discard_pt.clone(), 6), 45 as f64, 10 as f64, 1);
    let ccw_pt: Point = arrow_point(arrow_pt.clone(), discard_pt.clone(), 45 as f64, 10 as f64, -1);
    Self { arrow_pt: arrow_pt.clone(), cw_pt, ccw_pt, retract_pt: discard_pt }
  }

}

/*fn arrow_v(other_pt: Point, arrow_pt: Point, deg: f32, len) -> ArrowV {
  let av = ArrowV::new(other_pt, arrow_pt);
}
*/

pub fn er_card_ind_html(x: i32, y: i32, cinch: Point, end: crate::rel_map::RelEnd, term_type: RelTerminiType, stack: &mut crate::svg_stack::SvgStack) {
  // input: a single endpoint (at the key column position)
  /*let x: i32 = (end.point.x - 10).try_into().unwrap();
  let y: i32 = (end.point.y - 10).try_into().unwrap();
  let stack = crate::svg_stack::SvgStack::new(x, y, 20, 20);*/
  let arrow = ArrowV::new(cinch.clone(), end.point);
  println!("{} - {}", arrow.ccw_pt.x, x);
  println!("{} - {}", arrow.ccw_pt.y, y);
  let x1 = arrow.ccw_pt.x - x;
  let y1 = arrow.ccw_pt.y - y;
  let x2 = arrow.arrow_pt.x - x;
  let y2 = arrow.arrow_pt.y - y;
  let x3 = arrow.cw_pt.x - x;
  let y3 = arrow.cw_pt.y - y;

  // TODO: uncomment to re-enable once fixed
  /*stack.push(format!("<polyline points='{} {} {} {} {} {}' stroke='black' fill='transparent' stroke-width='1'></polyline>", x1, y1, x2, y2, x3, y3));
  match term_type {
    RelTerminiType::Many => {
      let arrow2 = arrow.retracted_clone();
      stack.push(format!("<polyline points='{} {} {} {} {} {}' stroke='black' fill='transparent' stroke-width='1'></polyline>", arrow2.ccw_pt.x - x, arrow2.ccw_pt.y - y, arrow2.arrow_pt.x - x, arrow2.arrow_pt.y - y, arrow2.cw_pt.x - x, arrow2.cw_pt.y - y));

    },
    _ => {}
  }*/
  
  // todo: Create cardinality arrow point(s) as svg
  //   It will be the same implementation that is found in:
  //     * act.svg:js/node-draw.js (dwDrawUpdate)
  //     * act.svg:index.js (arrowPoint, rotate)
  //return stack;//.to_string();
}
