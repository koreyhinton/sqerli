use crate::tokenize;
use crate::tokenize::TokenType;

pub fn er_html(tokens: Vec::<tokenize::Token>) -> String {
  dbg!("{:?}", tokens.to_vec());
  let mut str = String::new();
  for token in tokens {
    match token.tokType {
      TokenType::CRT => {
        str = format!("{}<br/><br/><input type='text' value='{}' style='font-weight:bold;' disabled='disabled'>", str.to_string(), token.tokValue)
      },
      TokenType::CRT_COL => {
        str = format!("{}<br/><input type='text' value=' + {}' style='font-style:italic;background-color:white;' disabled='disabled'>", str.to_string(), token.tokValue)
      },
      _ => println!("unmatched token type {:?}", token.tokType)
    }
    /*if token.tokType == tokenize::TokenType::CRT {
      return "<div style='color:red;width:100px;height:100px'>test</div>".to_owned();
    }*/
  }
  return str.to_string(); //"".to_owned();
}
