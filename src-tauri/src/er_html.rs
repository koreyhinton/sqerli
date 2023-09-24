use crate::tokenize;
use crate::tokenize::TokenType;
use crate::svg_stack::SvgStack;

pub fn er_html(tokens: Vec::<tokenize::Token>) -> String {
  dbg!("{:?}", tokens.to_vec());

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
  stack.close();
  return stack.to_string()
}
