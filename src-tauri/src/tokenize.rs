use std::iter::Peekable;
use core::fmt::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
  COM_CLS,
  COM_OPN,
  CRT,
  CRT_COL,
  CRT_CLS,
  CRT_OPN,
  REL,
  STM_CLS
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

fn can_col(tok: Token) -> bool {
  return tok.tokType == TokenType::CRT_OPN || tok.tokType == TokenType::CRT_COL;
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

fn get_next_token_string<'a>(str: &'a str, tokens: &mut Vec<Token>, rel_map: &mut Vec<crate::rel_map::RelMapping>) -> Option<(&'a str, &'a str)> {
  // dbg!("get_next_token_string");
  for (i,c) in str.chars().enumerate() {
    // dbg!(i, c);
    if ws(c) && (zero(tokens.to_vec()) || opn(last(tokens.to_vec()))) {
      // dbg!("ws detected");
      return Some((&str[..i+1], &str[i+1..]));
    }
    else if zero(tokens.to_vec()) && c == '/' && (str.len()>1/*&& &str[i+1..i+2]=="*"*/) {
      // dbg!("start comment detected");
      tokens.push(Token{tokType:TokenType::COM_OPN,tokValue:"/*".to_owned()});
      return Some((&str[..i], &str[i+2..]));
    }
    else if c == '<' && gt0(tokens.to_vec()) && (opn(last(tokens.to_vec()))||rel(last(tokens.to_vec()))) {
      // dbg!("relationship detected");
      // let mut rel_tokens = Vec::<Token>::new();
      // tokenize_rel(chars, &mut rel_tokens);
      // tokens.append(&mut rel_tokens);
      // ==> For now skipping comment containing table col rels
      for (j, c2) in str.chars().enumerate() {
        if c2 == '*' && str.len() > j+1 && &str[j+1..j+2] == "/" {
          // dbg!(&str[..j]);
          crate::rel_map::rel_mappings(&str[..j], rel_map);
          return Some((&str[..j], &str[j+2..]))
        }
      }
    }
    else if str.len() > "create table".len() && str[i.."create table".len()].to_lowercase() == "create table" {
      // dbg!("detected create statement");
      // tokens.push(Token{tokType:TokenType::CRT,tokValue:str[i.."create table".len()].to_string()/*str.to_owned()*/});
      for (j, c2) in str.chars().enumerate() {
        if c2 == '(' {
          tokens.push(Token{tokType:TokenType::CRT,tokValue:str[i+"create table".len()+1..j].trim().to_string()/*str.to_owned()*/});
          tokens.push(Token{tokType:TokenType::CRT_OPN,tokValue:"(".to_owned()});
          return Some((&str[..j], &str[j+1..]));
        }
      }
      
    }
    else if can_col(last(tokens.to_vec())) {
      // dbg!("detected column in create statement");
      for (j, c2) in str.chars().enumerate() {
        if c2 == ')' {
          //tokens.push(Token{tokType:TokenType::CRT_COL,tokValue:col.to_owned()});
          tokens.push(Token{tokType:TokenType::CRT_COL,tokValue:str[i..j].trim().to_string()});
          tokens.push(Token{tokType:TokenType::CRT_CLS,tokValue:")".to_owned()});
        }
        else if c2 == ';' {
          tokens.push(Token{tokType:TokenType::STM_CLS,tokValue:";".to_owned()});
          return Some((&str[..j+1], &str[j+1..]));
          // return None;
        }
        else if c2 == ',' {
          tokens.push(Token{tokType:TokenType::CRT_COL,tokValue:str[i..j].trim().to_string()});
          return Some((&str[..j], &str[j+1..]));
        }
        /*else if j<str.len()-1 {
          return Some((&str[..j+1], &str[j+1..]));
        }
        else {
          return None;
        }*/
      }
    }
    else if ws(c) && i<str.len()-1 {
      return Some((&str[..i+1], &str[i+1..]));
    }
    else {
      // dbg!(c);
      // dbg!(i);
      println!("???");
      return None;
    }
  }
  None
}

pub fn tokenize(chars_in: &mut Peekable<std::str::Chars>, tokens: &mut Vec<Token>, rel_map: &mut Vec<crate::rel_map::RelMapping>) {
  let str: &str = &String::from(chars_in.collect::<String>());
  let mut i = 0;
  let mut str2: &str = str;
  loop {
    // let str2 = str;

    let Ok((prev,str3)) = get_next_token_string(str2, tokens, rel_map).ok_or(Error)
    else {
      break;
    };
//.unwrap(); //.ok_or(Error)?;
    str2 = str3;
    /*dbg!(prev);
    dbg!(str2);
    println!("--");*/
    i = i + 1;
    if i > 10 {
      continue; // break;
    }
  }
  /*while let Some((prev,str)) = get_next_token_string(str, tokens) {
    dbg!(prev);
    dbg!(str);
    println!("--");
    i = i + 1;
    if i > 10 {
      break;
    }
  }*/
}

