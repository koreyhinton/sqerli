use std::iter::Peekable;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
  COM_CLS,
  COM_OPN,
  CRT,
  CRT_COL,
  CRT_CLS,
  CRT_OPN,
  REL
}

#[derive(Clone,Debug)]
pub struct Token {
  pub tokType: TokenType, // Rel
  pub tokValue: String // TblNm.ColNm<-->>Tbl2Nm.ColNm
}

fn eos(chars: &mut Peekable<std::str::Chars>) -> bool { // end of string
  return chars.peekable().next().is_none();
}

fn opn(tok: Token) -> bool {
  return tok.tokType == TokenType::COM_OPN;
}

fn rel(tok: Token) -> bool {
  return tok.tokType == TokenType::REL;
}

fn ws(ch: char) -> bool {
  return ch.is_whitespace();
}

fn last(tokens: Vec<Token>) -> Token {
//fn last<'a>(tokens: &'a mut Vec<&'a mut Token>) -> &'a mut Token {
  return tokens.iter().last().unwrap().clone(); //.iter()[tokens.len()-1];
}

fn zero(tokens: Vec<Token>) -> bool {
  return tokens.len() == 0;
}

fn gt0(tokens: Vec<Token>) -> bool {
  return tokens.len() > 0;
}

fn has(tokens: Vec<Token>, tokenType: TokenType) -> bool {
  return tokens.iter().any(|t| t.tokType == tokenType);
}

/*fn it_chars(chars: Peekable<std::str::Chars>) -> &mut Peekable<std::str::Chars> {
// fn it_chars<'a>(chars: &'a mut Peekable<std::str::Chars<'a>>) -> &'a mut Peekable<std::str::Chars<'a>> {
//  return &mut 
  let mut str = chars.collect::<String>();
  str = format!("0{}", str);
  let mut chars_out: &mut Peekable<std::str::Chars> = &mut str.chars().collect::<std::str::Chars>().peekable();
  return chars_out;
//&mut str.chars().into_iter().collect::<std::str::Chars>().peekable();
//chars_out.collect(); //format!("0{}", chars.collect::<String>()).chars().peekable().collect();
}*/
fn forward(chars: &mut Peekable<std::str::Chars>) {
  chars.next();
}
fn current(chars: &mut Peekable<std::str::Chars>) -> char {
  return *chars.peek().unwrap();
}
fn peek(chars: &mut Peekable<std::str::Chars>) -> char {
  let str = format!("{}", chars.collect::<String>());
  let mut test = str.chars();
  return test.nth(1).unwrap();
}

fn tokenize_rel(chars: &mut Peekable<std::str::Chars>, tokens: &mut Vec<Token>){
  // for now just skipping
  loop {
    let ch = chars.next().unwrap();
    if ch == '*' && chars.peek() == Some(&'/') {
        tokens.push(Token{tokType:TokenType::COM_CLS,tokValue:"*/".to_owned()});
        chars.next();
        break;
    }
  }
}

//fn tokenize(name: &mut String) -> Vec<String> {
pub fn tokenize(chars_in: &mut Peekable<std::str::Chars>, tokens: &mut Vec<Token>) {
//fn tokenize<'a>(chars: &mut Peekable<std::str::Chars>, tokens: &'a mut Vec<&'a mut Token>) {

  let binding = format!("0{}", chars_in.collect::<String>());
  let chars = &mut binding.chars().peekable();
  //let chars = it_chars(chars_in.collect().chars().peekable());
  //let binding = chars_in.collect::<String>();
  //let chars = it_chars(binding.chars().peekable());
  //let chars = it_chars(chars_in.collect::<String>().chars().peekable());

  loop {
    if eos(chars) {
      break;
    }
    let ch = current(chars); //.next().unwrap();
    dbg!(ch);
    if ch == '/' {
      //dbg!(format!("c{}", chars.collect::<String>().to_string().as_str().to_lowercase()).starts_with("create table"));
      //dbg!(has(tokens.to_vec(),TokenType::COM_CLS));
      dbg!(tokens.to_vec());
      dbg!(ch);
      dbg!(peek(chars));
      dbg!(zero(tokens.to_vec()));
      dbg!(ch == '/');
      dbg!(peek(chars) == '*');
      return; //todo: remove
    }

    if ws(ch) && (zero(tokens.to_vec()) || opn(last(tokens.to_vec()))) {}
    else if zero(tokens.to_vec()) && ch == '/' && *chars.peek().unwrap() == '*' {
      dbg!("start comment detected");
      forward(chars); //.next();
      tokens.push(Token{tokType:TokenType::COM_OPN,tokValue:"/*".to_owned()});
    }
    else if ch == '<' && gt0(tokens.to_vec()) && (opn(last(tokens.to_vec()))||rel(last(tokens.to_vec()))) {
      dbg!("relationship detected");
      let mut rel_tokens = Vec::<Token>::new();
      tokenize_rel(chars, &mut rel_tokens);
      tokens.append(&mut rel_tokens);
    }
    else if (ch == 'c'||ch=='C') && has(tokens.to_vec(),TokenType::COM_CLS) && format!("c{}", chars.collect::<String>().to_string().as_str().to_lowercase()).starts_with("create table") {
      dbg!("detected create statement");
      let mut str = String::new();
      for i in 1..13 { forward(chars);/*.next();*/ }
      while chars.peek() != Some(&'(') {
        
        str = format!("{}{}", str,current(chars)/*.next().unwrap()*/);
        forward(chars);
        println!("test: {}",str);
      }
      tokens.push(Token{tokType:TokenType::CRT,tokValue:str.to_owned()});
      forward(chars);//.next();
      tokens.push(Token{tokType:TokenType::CRT_OPN,tokValue:"(".to_owned()});

      let mut capture = true;
      while chars.peek() != Some(&')') {
        let mut col = String::new();
        while capture && chars.peek() != Some(&' ') {
          let col_ch = current(chars); //.next().unwrap();
          forward(chars);
          col = format!("{}{}", col, col_ch)
        }
        tokens.push(Token{tokType:TokenType::CRT_COL,tokValue:col.to_owned()});
        capture = chars.peek() == Some(&' ');
      }
      forward(chars); //.next();
      tokens.push(Token{tokType:TokenType::CRT_CLS,tokValue:")".to_owned()});
    }
  }
}
