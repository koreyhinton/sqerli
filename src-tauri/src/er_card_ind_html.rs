
pub fn er_card_ind_html(end: crate::rel_map::RelEnd) -> String {
  // input: a single endpoint (at the key column position)
  let x: i32 = (end.point.x - 10).try_into().unwrap();
  let y: i32 = (end.point.y - 10).try_into().unwrap();
  let stack = crate::svg_stack::SvgStack::new(x, y, 20, 20);
  // todo: Create cardinality arrow point(s) as svg
  //   It will be the same implementation that is found in:
  //     * act.svg:js/node-draw.js (dwDrawUpdate)
  //     * act.svg:index.js (arrowPoint, rotate)
  return stack.to_string();
}
