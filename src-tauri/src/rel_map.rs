use core::fmt::Error;

#[cfg(windows)]
const EOL: &'static str = "\r\n";
#[cfg(not(windows))]
const EOL: &'static str = "\n";

#[derive(Clone, Debug)]
pub struct Point {
  pub x: usize,
  pub y: usize
}

#[derive(Clone, Debug)]
pub enum RelTerminiType {
  One,
  Many
}

#[derive(Clone, Debug)]
pub struct RelEnd {
  pub point: Point,
  pub id: String // table.col
}

#[derive(Clone, Debug)]
pub struct RelTermini {
  pub r#type: RelTerminiType,
  pub rels: Vec<RelEnd>
}

/*

RelMapping:

  <--  A B C
  -->> D E F

 A         D
  \       /
B--fl-//-fr--E
  /       \
 C         F

  cinch = //
  fan_left (fl) RelTermini (left one)    = <--  A B C
  fan_right (fr) RelTermini (right many) = -->> D E F
  A = RelEnd (same for B-F)

*/
#[derive(Clone, Debug)]
pub struct RelMapping {
  pub fan_left: RelTermini,
  pub fan_right: RelTermini,
  pub cinch: Point
}

// -> (total_table_count, total_attribute_count)
pub fn counts(table: String, attribute: String, tokens: Vec<crate::tokenize::Token>) -> (usize, usize) {
  let mut table_done = false;
  let mut tcnt = 0;
  let mut acnt = 0;
  for token in tokens {
    if table_done && token.tokType == crate::tokenize::TokenType::CRT_COL && token.tokValue.split(" ").nth(0).unwrap() == attribute {
      acnt += 1;
      break;
    }
    else if table_done && token.tokType == crate::tokenize::TokenType::CRT_COL {
      acnt += 1;
    }
    else if table_done {
      continue;
    }
    else if token.tokType == crate::tokenize::TokenType::CRT_COL {
      acnt += 1;
    }
    else if token.tokType == crate::tokenize::TokenType::CRT {
      if token.tokValue == table {
        table_done = true;
      }
      tcnt += 1;
    }
  }
  (tcnt, acnt)
}

fn clean_rel_str_line(rel_str_line: &str) -> (String, usize) {
  (rel_str_line
    .trim() /* -> &str */
    .replace("--", "") /* -> String */
    .trim() /* -> &str */
    .to_string()
  , rel_str_line.len())
}

fn get_next_mapping<'a>(rel_str: &'a str, rel_maps: &mut Vec<RelMapping>) -> Option<(&'a str, &'a str)> {
  for (i,c) in rel_str.chars().enumerate() {
    if c == '<' {

      let x: usize = 200;
      let y: usize = 100+400*rel_maps.len();

      let mut left_type = RelTerminiType::One;
      let mut right_type = RelTerminiType::One;
      let mut j = i;
      if &rel_str[j+1..j+2] == "<" {
        j += 1;
        left_type = RelTerminiType::Many;
      }
      j += 1;
      // line here means left-trimmed line (ws was skipped and line starts at <)
      let line_left = rel_str[j..].split("\n").nth(0).unwrap();
      // let columns_left = line_left.trim().replace("--", "").trim().split(" ");
      let (line_left_clean, line_left_len) = clean_rel_str_line(line_left);
      let columns_left = line_left_clean.split(" ");
      j += /*line_left.len()*/ line_left_len + EOL.chars().count();
      while rel_str[j..j+1].chars().next().unwrap().is_whitespace() {
        j += 1;
      }
      j += 2; // surpass '--' in '-->'
      if &rel_str[j+1..j+2] == ">" {
        right_type = RelTerminiType::Many;
        j += 1;
      }
      j += 1;
      let line_right = rel_str[j..].split("\n").nth(0).unwrap();
      let (line_right_clean, line_right_len) = clean_rel_str_line(line_right);
      let columns_right = line_right_clean.split(" ");
      /*let line_right_clean = line_right
        .trim() /* -> &str */
        .replace("--", "") /* -> String */
        ;
      let columns_right: Vec<&str> = line_right_clean
        .trim() /* -> &str */
        .split(' ')
        .collect()
        ;*/
      //let columns_right = columns_right_str.split(" ");
      j += /*line_right.len()*/line_right_len + EOL.chars().count();

      let mut rels_left = Vec::<RelEnd>::new();
      let mut rels_right = Vec::<RelEnd>::new();
      for lcol in columns_left {
        let col = lcol;//.unwrap();
        let end = RelEnd { point: Point { x: 0, y: 0 }, id: col.to_string() };
        rels_left.push(end);
      }
      for rcol in columns_right {
        let col = rcol;//.unwrap();
        let end = RelEnd { point: Point { x: 0, y: 0 }, id: col.to_string() };
        rels_right.push(end);
      }
      let map = RelMapping {
        fan_left: RelTermini { r#type: left_type, rels: rels_left },
        fan_right: RelTermini { r#type: right_type, rels: rels_right },
        cinch: Point { x, y }
      };
      rel_maps.push(map);
      println!("check!: i={:?}, j={:?}, len={:?}", i, j, rel_str.len());
      if j+1 >= rel_str.len() {
        return None
      }
      return Some((&rel_str[..j], &rel_str[j+1..]))
    } // end if '<'
    else if i+1 < rel_str.len() {
      return Some((&rel_str[..i], &rel_str[i+1..]))
    }
    return None
  } // end for rel_str enumerate
  None
}

pub fn rel_mappings(rel_str: &str, rel_maps: &mut Vec<RelMapping>) {
  // dbg!(rel_str);
  let mut str2: &str = rel_str;
  loop {
    let Ok((prev,str3)) = get_next_mapping(str2, rel_maps).ok_or(Error)
    else {
      println!("Consumed (end): {:?}", str2);
      break;
    };
    println!("Consumed: {:?}", prev);
    println!("Remaining: {:?}", str3);
    str2 = str3;
  }
}

pub fn set_points(rel_maps: &mut Vec<RelMapping>, tokens: Vec<crate::tokenize::Token>) {
  // call set_points after all sql tokens have been added to tokens
  let h: usize = 27;
  let lead: usize = 0; //65 - 40;
  for map in rel_maps {
    for i in 0..map.fan_left.rels.len() {
      let mut names = map.fan_left.rels[i].id.split(".");
      // println!("{:?}", map.fan_left.rels[i].id);
      let tbl = names.next().unwrap().to_string();
      let col = names.next().unwrap().to_string();
      // nth actually advances the iterator
      let (tcnt, acnt) = counts(tbl/*names.nth(0).unwrap().to_string()*/, col/*names.nth(1).unwrap().to_string()*/, tokens.clone());
      // println!("id: {}. Table count = {}. Col count = {}.", map.fan_left.rels[i].id, tcnt, acnt);
      let mut lead2: usize = 27;
      if tcnt>1 {
        lead2 += ((tcnt)*27*2);
        lead2 -= 100;
        lead2 += 17;
      }
      map.fan_left.rels[i].point.x = 170;//60;
      map.fan_left.rels[i].point.y = lead2+lead + (tcnt+1)*h + acnt*(h/*/2*/); // tcnt*27 for spacing between tables
    }
    for i in 0..map.fan_right.rels.len() {
      let mut names = map.fan_right.rels[i].id.split(".");
      // println!("{:?}", map.fan_right.rels[i].id);
      let tbl = names.next().unwrap().to_string();
      let col = names.next().unwrap().to_string();
      let (tcnt, acnt) = counts(tbl, col, tokens.clone());
      let mut lead2: usize = 27;
      if tcnt>1 {
        lead2 += ((tcnt)*27*2);
        lead2 -= 100;
        lead2 += 17;
      }
      map.fan_right.rels[i].point.x = 170;//60;
      map.fan_right.rels[i].point.y = lead2+lead + (tcnt+1)*h + acnt*(h/*/2*/);
    }
  }
}
