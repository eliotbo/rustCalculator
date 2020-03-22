#[allow(warnings)]

use std::io;
// use std::io::prelude::v1::*;
// use std::fmt::Display;

use self::Token::*;
use std::f64;
use itertools::*;
use std::str;


#[derive(Debug,PartialEq, Clone,PartialOrd)]
pub enum Token {
    LPAREN,
    RPAREN,
    ADD,
    SUB,
    MUL,
    DIV,
    CARET,
    EQUALS,
    NUMBER(f64),
    SYMBOL(String),
    EOF
}

impl Token {
    /* returns (prec, associativity) where 0 is left and 1 is right*/
    pub fn info(&self) -> Option<(usize, usize)> {
        match *self {
            ADD | SUB => Some((10, 0)),
            MUL | DIV => Some((20, 0)),
            CARET => Some((30, 1)),
            _ => { None }
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            LPAREN => '(',
            RPAREN => ')',
            ADD => '+',
            SUB => '-',
            MUL => '*',
            DIV => '/',
            CARET => '^',
            EQUALS => '=',
            EOF => 'E',
            NUMBER(_) => 'N',
            SYMBOL(_) => 'S',
        }
    }

}

// pub fn to_tokens(c) -> Token {
//     match c {
//         "(" => LPAREN,
//         ")" => RPAREN,
//         "+"  => ADD,
//         "-" => SUB,
//         "*" => MUL,
//         "/" => DIV,
//         "^" => CARET,
//         "=" => EQUALS,
//         "E" => EOF,
//         x => NUMBER( x.parse::<f64>().unwrap()  ),
//     }
// }

pub fn is_eof(t: &Token) -> bool{
    match t {
        &EOF => true,
        _ => false
    }
}


pub fn splitOnTokens(equation: &str) {
    // let splitted: Vec<&str> = equation.split("").collect();
    // let chars  = equation.chars();
    // let r = chars.group_by( |x| x.is_digit(10) )
    //     .into_iter()
    //     .map( |y| y.1.cloned().collect() );
    //     .collect::<Vec< Vec<&str>>();
    // // let rr: Vec<Vec<&str>>  = r.map(|y| y.1).collect();
    // println!("{:?}", r);

    let data = vec!["+", "2", "3", "-", "1"];

    let groups = data.iter()
        .group_by(|elt| elt.chars() .next().unwrap().is_digit(10) )
        .into_iter()

        
        .map(|(_, group)|  group . cloned() . collect() )
        // .map( join("") )
        // .collect::<Vec<&str>>();
        .collect::<Vec<Vec<&str>>>();
    println!("{:?}", groups);
}

// pub fn listOfListToList(listOfList: Vec<Vec<&str>> ) -> Vec<&str> {
//     let mut v = Vec::new();
//     let list = for i in listOfList {
//         v.push_str(i);
//     };
//     return list

// }

// pub fn splitOnTokens(equation: &str, previousDigits: &str) -> (equation, Vec<&str>, &str) {
//     equation.chars()
//     // if (equation.len() > 1) {
//     //     let head: <&str> = equation.first();
//     //     let tail: &str = &v[1..];
//     //     if previousDigits.len() > 0 && head.isDigit() {
//     //         return (tail, previousDigits.push(head)
//     //     }
//     // } else if ( equation.len() == 1 ) {

//     // }
//     // s += match   
// } 


#[derive(PartialEq,Debug)]
struct Node<'a> {
    val: &'a Token,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}
impl<'a> Node<'a> {
    pub fn insert(&mut self, new_val: &'a Token) {
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node { val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}


// // comment for commit
// fn build_tree(input: &str) -> (f64, String) {
//     let xx: Token = EQUALS;
//     let mut x = Node { val: &xx, l: None, r: None };
//     x.insert(&xx);
//     // x.insert(input);
//     x.insert(&xx);
//     println!("{:?}",x);
//     assert!(x == Node {
//         val: &xx,
//         l: None,
//         r: Some(Box::new(Node {
//             val: &xx,
//             l: None,
//             r: Some(Box::new(Node {
//                 val: &xx,
//                 l: None,
//                 r: Some(Box::new(Node {
//                     val: &xx,
//                     l: None,
//                     r: None,
//                 })),
//             })),
//         })),


//     });
//     (3.0,String::from("hello baby"))

// }

pub fn main() {
    splitOnTokens("2-1");
    // let q = splitOnTokens("2-1");
    // println!("{:?}",q);
    // build_tree("2-1");
//     use std::f64;
//     let mut tree = Hashmap::new()

//     let stdin = io::stdin();


//    loop {
//        print!(">> ");
//        io::stdout().flush().ok();
//        io::Write::flush().ok();

//        et mut input = String::new();

//        match stdin.read_line(&mut input: &str){
//            Ok(_) =>  {
//                if input.len() == 0 {
//                    println!("");
//                    return;
//                }
//             err => {}

//             //    let expression_text = input.trim_right();


//            }
//        }

}
