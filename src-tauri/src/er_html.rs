use crate::tokenize;
use crate::tokenize::TokenType;

pub fn er_html(tokens: Vec::<tokenize::Token>) -> String {
  dbg!("{:?}", tokens.to_vec());

  let mut svg = String::new();
  svg = format!("{}", "<svg width='100' height='1000' id='rels'>");

  let mut str = String::new();
  str = format!("{}", "<div id='er'>".to_owned());
  let startX = 60;
  let startY = 50;
  let w = 105;
  let h = 10;
  let mut br = 0;
  let mut x = 5; // /*startX +*/ w;
  let mut y = 65; // /*startY +*/ (h/2);
  for token in tokens {
    match token.tokType {
      TokenType::CRT => {
        str = format!("{}<br/><br/><input type='text' value='{}' style='font-weight:bold;' disabled='disabled'>", str.to_string(), token.tokValue);
        y = y + (h/2) + br + br;
        br = 13; //48;
      },
      TokenType::CRT_COL => {
        str = format!("{}<br/><input type='text' value=' + {}' style='font-style:italic;background-color:white;' disabled='disabled'>", str.to_string(), token.tokValue);
        // todo: redo with tokens (from id relationships noted in sql comments)
        if token.tokValue.starts_with("Gid") {
          svg = format!("{}<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1'/>", svg, x, y, x+50, y);
          svg = format!("{}<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1'/>", svg, x+50, y, x+50, y+10);
        }
        y = y + h + br; //(h/2);
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

  return str.to_string(); //"".to_owned();
}
