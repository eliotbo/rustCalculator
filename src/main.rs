
// #![feature(box_syntax, box_patterns)]

use std::io;
// use std::io::prelude::v1::*;
// use std::fmt::Display;

use self::Token::*;
use std::f64;
use itertools::*;
use std::str;
use Token::*;

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
            NUMBER(_) => Some((40,0)),
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

pub fn to_tokens(c: &str) -> Token {
    match c {
        "(" => LPAREN,
        ")" => RPAREN,
        "+"  => ADD,
        "-" => SUB,
        "*" => MUL,
        "/" => DIV,
        "^" => CARET,
        "=" => EQUALS,
        "E" => EOF,
        x => NUMBER( x.parse::<f64>().unwrap()  ),
    }
}



pub fn is_eof(t: &Token) -> bool{
    match t {
        &EOF => true,
        _ => false
    }
}

pub fn splitOnTokens(equation: Vec<&str>) -> Vec<String> {
    let groups: Vec<String> = equation.iter()
        .group_by(|elt| elt.chars() .next().unwrap().is_digit(10) )
        .into_iter()
        .map(|(_, mut group)|  group . join("")  )
        .collect();
    return groups

}


#[derive(PartialEq,Debug)]
struct Node<'a> {
    val:  (&'a Token, &'a usize) ,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}
impl<'a> Node<'a> {

    // pub fn deleteRoot(&mut self) -> Node  {
    //     self=  &mut self.r ;
    //     match target_node {
            
    //         &mut Some(ref mut Node {val: v, l:l, r:r} ) => { 
    //             *self = Some{ subnode }; 
    //         },
    //         _ => { self; },
    //     }

     
    // } //expected `()`, found `&mut Node<'a>`


    pub fn insert(&mut self, new_val:  (&'a Token, &'a usize) ) {

        let target_node = if new_val.1 < self.val.1 { &mut self.l } else { &mut self.r };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node { val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    pub fn eval(&self) -> f64 {
        println!("{:?}",self);


        match (self.val, self.l.as_ref(), self.r.as_ref()) {
            ( (op,_) 
            , Some( box_n1 )
            , Some( box_n2 ) ) => {
                match ( box_n1.as_ref(), box_n2.as_ref() ) {

                    ( Node {val: (NUMBER(n1),_), l: None, r:None}, Node {val: (NUMBER(n2),_), l: None, r:None} )   => {
                        let result = match op {
                            ADD =>  { (n1 + n2) }
                            SUB =>  { (n1 - n2) }
                            MUL =>  { (n1 * n2) }
                            DIV =>  { (n1 / n2) }
                            CARET => { (n1.powf(*n2)) }
                            _ => { 0.0 }
                        };
                        return result
                    }

                    ( Node {val: (NUMBER(n1),_), l: None, r:None},  r )   => {
                        let temp = r.eval();
                        let result = match op {
                            ADD =>  { n1 + temp }
                            SUB =>  { n1 - temp }
                            MUL =>  { n1 * temp }
                            DIV =>  { n1 / temp }
                            CARET => {  n1.powf(temp)  }
                            _ => { 0.0 }
                        };
                        return result
                    }

                    ( l, Node {val: (NUMBER(n2),_), l: None, r:None} )   => {
                        let r = match op {
                            ADD =>  { (l.eval() + n2) }
                            SUB =>  { (l.eval() - n2) }
                            MUL =>  { (l.eval() * n2) }
                            DIV =>  { (l.eval() / n2) }
                            CARET => { (l.eval().powf(*n2)) }
                            _ => { 0.0 }
                        };
                        return r
                    }

                    (  l, r )   => {
                        let r = match op {
                            ADD =>  {  l.eval() + r.eval()  }
                            SUB =>  {  l.eval() - r.eval()  }
                            MUL =>  { l.eval() * r.eval()  }
                            DIV =>  { l.eval() / r.eval()  }
                            CARET => { l.eval().powf( r.eval() )  }
                            _ => { 0.0 }
                        };
                        return r
                    }

                    _ => { 0.0 }
                }
            }
            _ => { 0.0 }
                
            
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

// pub fn buildTree(tokenList, tree: Node, priority: usize) {
//     for i in &w {
//         if let Some(a,b) = i.info() {
//             if a == priority {
//                 tree.insert(i);
//             }
//         } 
//     }
// }

pub fn main() {
    // let data = vec!["+", "2", "3", "-", "1"];
    let mut v = String::from("3^3+3/3");
    let v2: Vec<&str> = v.split(""). collect::<Vec<&str>>();
    let  v3 = &v2[1..(v2.len()-1)];
    println!("{:?}",v3);
    let qq = splitOnTokens(v3.to_vec());
    let  w :  Vec<Token> =  qq.iter().map(|s| to_tokens(s)).collect();

    let  j: Vec<usize> = (1..w.len()+1).collect() ;
    println!("len is {:?}",j);
    let  tokenList: Vec<(&Token, &usize)> = w.iter().zip( j.iter() ).collect();
    println!("tokenlist: {:?}",tokenList);
    let mut  tree: Node = Node {val: (&Token::EQUALS, &1), l: None, r: None} ;

    // tree.insert(&ADD);

    for i in &tokenList {
        if let Some((a,b)) = i.0.info() {
            if a == 10 {
                tree.insert(*i);
            }
        } 
    }

    for i in &tokenList {
        if let Some((a,b)) = i.0.info() {
            if a == 20 {
                tree.insert(*i);
            }
        } 
    }

    for i in &tokenList {
        if let Some((a,b)) = i.0.info() {
            if a == 30 {
                tree.insert(*i);
            }
        } 
    }

    for i in &tokenList {
        if let Some((a,b)) = i.0.info() {
            if a == 40 {
                tree.insert(*i);
            }
        } 
    }

    println!("{:?}",tree.r.unwrap().as_ref().eval() );


}