use crate::tokenize;

pub fn er_html(tokens: Vec::<tokenize::Token>) -> String {
  dbg!("{:?}", tokens.to_vec());
  for token in tokens {
    if token.tokType == tokenize::TokenType::CRT {
      return "<div style='color:red;width:100px;height:100px'>test</div>".to_owned();
    }
  }
  return "".to_owned();
}
