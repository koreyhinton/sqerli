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
  // println!("({}-{})/({}-{})", pt2.y, pt1.y, pt2.x, pt1.x);
  // TODO: rise/0 divide by 0 problem,
  //       and test all x=0 and y=0 scenarios
  let m = (pt2.y as f32 - pt1.y as f32)/(pt2.x as f32 - pt1.x as f32);
  let b = pt2.y as f32 - m*(pt2.x as f32);
  let mut dir = 1 as i32;
  if pt2.x > pt1.x {
    dir = -1;
  }
  pt_a.x = pt2.x;
  pt_a.x += ((dir) *1 as i32)/* as usize*/;
  pt_a.y = (m*(pt_a.x as f32) + b) as i32;
  let mut d = distf(pt_a.x as f32, pt2.x as f32, pt_a.y as f32, pt2.y as f32); // distf(pt_a.x as f32, pt_a.y as f32, pt2.x as f32, pt2.y as f32);
  let max_iter = 30;
  let mut i = 0;
  while d < len {
    pt_a.x += (dir*1); //  as usize;
    pt_a.y = (m*(pt_a.x as f32) + b) as i32;
    d = distf(pt_a.x as f32, pt2.x as f32, pt_a.y as f32, pt2.y as f32);// distf(pt_a.x as f32, pt_a.y as f32, pt2.x as f32, pt2.y as f32);
    i += 1;
    if i>max_iter {
      break;
    }
  }
  // rotate(pt_b.x as f64, pt_b.y as f64, pt_a.x as f64, pt_a.y as f64, sign as f64 * angle)
  rotate(pt_a.x as f64, pt_a.y as f64, pt_b.x as f64, pt_b.y as f64, sign as f64 * angle)
}

fn distf(x1: f32, x2: f32, y1: f32, y2: f32) -> f64 {
  ((
    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)
  ) as f64).sqrt()
} 

fn shrink_pt(shrink_pt: Point, dir_pt: Point, dist: i32) -> Point {
  let ydiff = (shrink_pt.y - dir_pt.y).abs(); //<usize as TryInto<i32>>::try_into(shrink_pt.y - dir_pt.y).unwrap().abs();
  let xdiff = (shrink_pt.x - dir_pt.x).abs(); //<usize as TryInto<i32>>::try_into(shrink_pt.x - dir_pt.x).unwrap().abs();
 
  if xdiff > dist/*.try_into().unwrap()*/ {

    // solving for x2,y2 which is the cinch point shrunk in the direction of
    // x1,y1 (end point)

    let x1 = shrink_pt.x as f32;
    let y1 = shrink_pt.y as f32;

    let mut x2 = dir_pt.x as f32;
    let mut y2 = dir_pt.y as f32;

    let m = (y2-y1)/(x2-x1); // rise/run slope
    let b = y1 - m*x1;

    for x in 1..xdiff { //<i32 as TryInto<usize>>::try_into(xdiff).unwrap() {

      if shrink_pt.x > dir_pt.x {
        x2 = x2 + (1 as f32);
      } else {
        x2 = x2 - (1 as f32);
      }

      // y = mx + b

      y2 = m*x2 + b;

      if dist==15 {
        //println!("try x: {} in {}. newD {} <= dist {} = {}", x, xdiff, distf(x1, x2, y1, y2), dist as f64, distf(x1, x2, y1, y2) <= dist as f64);
      }

      if distf(x1, x2, y1, y2) <= dist as f64 {
        if dist==15 {
          // println!("found x: {} in xdiff {}, x1={} y1={} x2={} y2={} dist={}", x, xdiff,x1,y1,x2,y2,dist);
        }
        return Point { x: x2 as i32, y: y2 as i32 }
      }
    }
  }
  // TODO: Add back in the else if ydiff condition
  //       because if it ever had cinch points in between rows
  //       then you could have have cinch.x ~= end.point.x
  //       or if cinch points were brought closer so that
  //       cinch.x - end.point.x < dist.
  /*else if ydiff > dist.try_into().unwrap() {
    for y in 1..ydiff { //<i32 as TryInto<usize>>::try_into(ydiff).unwrap() {
      let mut y1 = dir_pt.y;
      let mut y2 = shrink_pt.y;
      if dir_pt.y > shrink_pt.y {
        y2 += y;
      } else {
        y2 -= y;
      }
      //let x1 = dir_pt.x;
      //let x2 = shrink_pt.x;
      if distf(x1, x2, y1, y2) >= dist as f64 {
        return Point { x: x2 as i32, y: y2 as i32 }
      }
    }
  }*/
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


    let new_arrow_pt = shrink_pt(self.arrow_pt.clone(), self.retract_pt.clone(), 7);
    let new_arrow_x = new_arrow_pt.x;
    let new_arrow_y = new_arrow_pt.y;

    let new_shrink_pt = shrink_pt(new_arrow_pt.clone(), self.retract_pt.clone(), 15);
    /*let mut new_x: usize = self.arrow_pt.x + 2;
    if self.arrow_pt.x > self.retract_pt.x {
      new_x = self.arrow_pt.x - 2;
    }
    let new_y: usize = m * new_x + b;*/

    let arrow_arm_len = 11; // 20
    let arrow_arm_angle = 30; // 45

    let cw_pt = arrow_point(new_arrow_pt.clone(), /*self.retract_pt.clone()*/new_shrink_pt.clone(), arrow_arm_angle as f64, arrow_arm_len as f64, 1);
    let ccw_pt = arrow_point(new_arrow_pt, /*self.retract_pt.clone()*/new_shrink_pt.clone(), arrow_arm_angle as f64, arrow_arm_len as f64, -1);
    Self { arrow_pt: Point { x: new_arrow_x, y: new_arrow_y}, cw_pt, ccw_pt, retract_pt: self.retract_pt.clone() }
  }

  pub fn new(discard_pt: Point, arrow_pt: Point) -> Self {

    let arrow_arm_len = 11; // 20
    let arrow_arm_angle = 30; // 45

    // right-angle edge case (which uses a different algorithm then set_line_angle).
    // discard_pt is used only for calculating cw_pt and ccw_pt
    let shrink_pt = shrink_pt(arrow_pt.clone(), discard_pt.clone(), arrow_arm_len);
    let cw_pt: Point = arrow_point(arrow_pt.clone(), /*discard_pt.clone()*/shrink_pt.clone(), arrow_arm_angle as f64, arrow_arm_len as f64, 1);
    let ccw_pt: Point = arrow_point(arrow_pt.clone(), shrink_pt, arrow_arm_angle as f64, arrow_arm_len as f64, -1);
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
  let arrow = ArrowV::new(cinch.clone(), end.point.clone());
  //println!("{} - {}", arrow.ccw_pt.x, x);
  //println!("{} - {}", arrow.ccw_pt.y, y);
  let x1 = arrow.ccw_pt.x - x;
  let y1 = arrow.ccw_pt.y - y;
  let x2 = arrow.arrow_pt.x - x;
  let y2 = arrow.arrow_pt.y - y;
  let x3 = arrow.cw_pt.x - x;
  let y3 = arrow.cw_pt.y - y;

  let dbg_step_1_line = false;
  if dbg_step_1_line {
    // step 1 to create the arrow point is to create a small line that is
    // positioned by the end point before it gets rotated into an angled
    // arrow point (clockwise and counterclockwise points).
    //
    // if things don't look right then turn this back on to see if the small
    // red line exactly overlaps the full line or not.
    //
    let shrink_line_pt = shrink_pt(/*shrink_pt:*/ Point { x: end.point.clone().x-x, y: end.point.clone().y-y}, /*dir_pt:*/ Point { x: cinch.clone().x-x, y: cinch.clone().y-y}, 15);
    // println!("shrink line {},{} {},{}", shrink_line_pt.x, shrink_line_pt.y, end.point.clone().x-x, end.point.clone().y-y);
    stack.push(format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='red' stroke-width='1'/>", shrink_line_pt.x, shrink_line_pt.y, end.point.clone().x-x, end.point.clone().y-y));
  }

  stack.push(format!("<polyline points='{} {} {} {} {} {}' stroke='black' fill='transparent' stroke-width='1'></polyline>", x1, y1, x2, y2, x3, y3));
  match term_type {
    RelTerminiType::Many => {
      let arrow2 = arrow.retracted_clone();
      stack.push(format!("<polyline points='{} {} {} {} {} {}' stroke='black' fill='transparent' stroke-width='1'></polyline>", arrow2.ccw_pt.x - x, arrow2.ccw_pt.y - y, arrow2.arrow_pt.x - x, arrow2.arrow_pt.y - y, arrow2.cw_pt.x - x, arrow2.cw_pt.y - y));

    },
    _ => {}
  }
  
  // todo: Create cardinality arrow point(s) as svg
  //   It will be the same implementation that is found in:
  //     * act.svg:js/node-draw.js (dwDrawUpdate)
  //     * act.svg:index.js (arrowPoint, rotate)
  //return stack;//.to_string();
}
