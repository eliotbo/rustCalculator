use std::io;
use std::io::prelude::v1::*;
// use std::fmt::Display;

use self::Token::*;
use std::f64;

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

pub fn is_eof(t: &Token) -> bool{
    match t {
        &EOF => true,
        _ => false
    }
}


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


// comment for commit
fn build_tree(input: &str) -> <f64, String> {
    let xx: Token = EQUALS;
    let mut x = Node { val: &xx, l: None, r: None };
    x.insert(&xx);
    x.insert(&xx);
    x.insert(&xx);
    println!("{:?}",x);
    assert!(x == Node {
        val: &xx,
        l: None,
        r: Some(Box::new(Node {
            val: &xx,
            l: None,
            r: Some(Box::new(Node {
                val: &xx,
                l: None,
                r: Some(Box::new(Node {
                    val: &xx,
                    l: None,
                    r: None,
                })),
            })),
        })),


    });
    (3.0,String::from("hello baby"));
}

pub fn main() {
    use std::f64;
    let mut tree = Hashmap::new()

    let stdin = io::stdin();


   loop {
       print!(">> ");
       io::stdout().flush().ok();

       et mut input = String::new();

       match stdin.read_line(&mut input){
           Ok(_) =>  {
               if input.len() == 0 {
                   println!("");
                   return;
               }

               let expression_text = input.trim_right();


           }
       }

}
