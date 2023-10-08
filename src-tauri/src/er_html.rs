use crate::rel_map::RelTerminiType;
use crate::tokenize;
use crate::tokenize::TokenType;
use crate::rel_map::RelMapping;
use crate::rel_map::RelEnd;
use crate::svg_stack::SvgStack;
use crate::er_card_ind_html::er_card_ind_html;
use crate::rel_map::Point;

fn build_end_line(line_str: String, cinch: Point, end: RelEnd, term_type: RelTerminiType, stack: &mut SvgStack) {
  let x2 = cinch.x;
  let y2 = cinch.y;
  let e = end.clone();
  let x = std::cmp::min(end.point.x, x2) - 10; // <usize as TryInto<i32>>::try_into(std::cmp::min(end.point.x, x2)).unwrap() - 10;
  let y = std::cmp::min(end.point.y, y2) - 10; // <usize as TryInto<i32>>::try_into(std::cmp::min(end.point.y, y2)).unwrap() - 10;
  stack.start_child(x, y);
  er_card_ind_html(x /*as usize*/, y /*as usize*/, cinch, end, term_type, stack);
  stack.push(line_str);
  // stack.end_child(<usize as TryInto<i32>>::try_into(std::cmp::max(e.clone().point.x, x2)).unwrap() + 20, <usize as TryInto<i32>>::try_into(std::cmp::max(e.clone().point.y, y2)).unwrap() + 20);
  stack.end_child(std::cmp::max(e.clone().point.x, x2) + 20, std::cmp::max(e.clone().point.y, y2) + 20);
  // stack.push(line_str);
}

fn er_html_rels(rel_maps: Vec::<RelMapping>, stack: &mut SvgStack) {

  for (i, map) in rel_maps.iter().enumerate() {
    stack.start_child(0, 0);

    let x2 = map.cinch.x;
    let y2 = map.cinch.y;

    for left in map.clone().fan_left.rels {
      let x = std::cmp::min(left.point.x, x2) - 10;
      let y = std::cmp::min(left.point.y, y2) - 10;
      let line_str = format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1' id='{}'/>", left.point.x-x, left.point.y-y, x2-x, y2-y, left.id);
      build_end_line(line_str, map.cinch.clone(), left, map.fan_left.r#type.clone(), stack);
    }
    for right in map.clone().fan_right.rels {
      let x = std::cmp::min(right.point.x, x2) - 10;
      let y = std::cmp::min(right.point.y, y2) - 10;
      let line_str = format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1' id='{}'/>", x2-x, y2-y, right.point.x-x, right.point.y-y, right.id);
      build_end_line(line_str, map.cinch.clone(), right, map.fan_right.r#type.clone(), stack);
    }
    stack.end_child(1000, 1000);
  }
}

pub fn er_html(tokens: Vec::<tokenize::Token>, rel_maps: Vec::<RelMapping>) -> String {
  // dbg!("{:?}", tokens.to_vec());

  let mut stack = SvgStack::new(0, 0, 1000, 1000);
  let mut svg = String::new();

  //svg = format!("{}{}",svg,stack.pop()); // format!("{}", "<svg width='100' height='1000' id='rels'>");

  let mut str = String::new();
  str = format!("{}", "<div id='er'>".to_owned());
  let startX = 60;
  let startY = 50;
  let w = 160; // 105;
  let h = 27; // 10;
  let mut br = 0;
  let mut x = 5; // /*startX +*/ w;
  let mut y = 65; // /*startY +*/ (h/2);
  let mut child_x = 0;
  let mut child_y = 0;
  for token in tokens {
    match token.tokType {
      TokenType::CRT => {
        stack.start_child(x, y);
        stack.push(format!("<text x='{}' y='{}' fill='black'>{}</text>", child_x+9, child_y+18, token.tokValue));
        stack.push(format!("<rect x='{}' y='{}' width='{}' height='{}' stroke='black' fill='#eeeeee' stroke-width='1'/>", child_x, child_y, w, h));
        /*str = format!("{}<br/><br/><input type='text' value='{}' style='font-weight:bold;' disabled='disabled'>", str.to_string(), token.tokValue);*/
        br = 7; //13; //48;
        y = y + (h/2) + br + br;
        child_y = child_y + (h/2) + br + br;
      },
      TokenType::CRT_COL => {
        // str = format!("{}<br/><input type='text' value=' + {}' style='font-style:italic;background-color:white;' disabled='disabled'>", str.to_string(), token.tokValue);
        stack.push(format!("<rect x='{}' y='{}' width='{}' height='{}' stroke='black' fill='transparent' stroke-width='1'/>", child_x, child_y, w, h));
        stack.push(format!("<text x='{}' y='{}' fill='black'> + {}</text>", child_x+9, child_y+18, token.tokValue));
        // todo: redo with tokens (from id relationships noted in sql comments)
        if token.tokValue.starts_with("Gid") {
          svg = format!("{}<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1'/>", svg, x, y, x+50, y);
          svg = format!("{}<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1'/>", svg, x+50, y, x+50, y+10);
        }
        y = y + h + br; //(h/2);
        child_y = child_y + h + br;
      },
      TokenType::CRT_CLS => {
        y = y + h + br;
        stack.end_child(x+w, y);
        child_x = 0;
        child_y = 0;
      },
      _ => println!("unmatched token type {:?}", token.tokType)
    }
    /*if token.tokType == tokenize::TokenType::CRT {
      return "<div style='color:red;width:100px;height:100px'>test</div>".to_owned();
    }*/
  }
  str = format!("{}{}", str, "</div>".to_owned());

  svg = format!("{}{}", svg, "</svg>".to_owned());
  str = format!("{}{}", str, svg);

  //return str.to_string(); //"".to_owned();

  er_html_rels(rel_maps, &mut stack);

  stack.close();
  return stack.to_string()
}
