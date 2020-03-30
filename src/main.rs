// extern crate kiss3d;
// extern crate nalgebra as na;

// use kiss3d::text::Font;
// use kiss3d::window::Window;
// use kiss3d::event::{Action, WindowEvent};
// use kiss3d::scene::PlanarSceneNode;
// use na::{Point2, Point3};
// use na::{Translation2, UnitComplex, Isometry2, Vector2};

// #[derive(Debug)]
// pub struct Mouse_pos(f64, f64);
// impl Mouse_pos {
//     fn x(&self) -> &f64 {
//         if let Mouse_pos(x,_) = self {
//             x
//         } else {
//             &0.0
//         }
//     }

//     fn y(&self) -> &f64 {
//         if let Mouse_pos(_,y) = self {
//             y
//         } else {
//             &0.0
//         }
//     }
// }


// #[allow(dead_code)]
// pub struct Button {

//     mid_position: Point2<f64>,
//     size: (f64, f64),
//     font_size: f32,
//     color: Point3<f32>,
//     label: String,
//     rect: PlanarSceneNode,
//     text_point: Point2<f32>,
// } 

// impl Button {
//     pub fn new(trans_rect_in_pixels: (f64, f64) , rec_size_in_pixels: (f64, f64), mut window: Window ) -> (Button, Window) {
//         let screen_width = (window.width() as f64  ) ;
//         let screen_height = (window.height() as f64 ) ;
//         let rec_size =  ( rec_size_in_pixels.0 as f64 / screen_width
//                     , rec_size_in_pixels.1 as f64 / screen_height
//                     );

//         // I have no idea why I need that
//         let what = 2.0_f64.sqrt(); // wtf, why do I even need this
//         let trans_rect =    ( trans_rect_in_pixels.0 as f64 / screen_width
//                             , trans_rect_in_pixels.1 as f64 / screen_height);
//         let button_mid = ((trans_rect.0)*what + 0.5 , (-trans_rect.1)*what + 0.5 ) ;
//         let button_size = ((rec_size.0 / what) as f64, (rec_size.1 / what) as f64 );

       
//         let color = Point3::new(0.0, 0.5, 0.2);
//         let font_size = 240.0;
        
//         // I have no idea why I need that either
//         let what_again = 2.0 * 2.0_f32.sqrt();
//         let t = Isometry2::new(Vector2::new(    -font_size/2.0*0.475
//                                             ,   -font_size/2.0), 0.0);

//         let t2 = Isometry2::new( Vector2::new(2.0 * (screen_width / 2.0) as f32 
//                                                     + trans_rect_in_pixels.0 as f32 *what_again
//                                             ,2.0 * (screen_height / 2.0) as f32 
//                                                     - trans_rect_in_pixels.1 as f32 *what_again), 0.0);
//         let p3 = Point2::new(0.0, 0.0);
//         let p4 = (t * t2 * p3);

//         let mut button = Button {
//             mid_position: Point2::new(button_mid.0, button_mid.1),
//             size:(button_size.0, button_size.1),
//             font_size: font_size,
//             label: String::from("6"),
//             color: color,
//             rect: window.add_rectangle( 
//                         rec_size_in_pixels.0 as f32
//                     ,   rec_size_in_pixels.1 as f32     
//                     ),
//             text_point: p4,
//         };

//         button.rect.append_translation(&Translation2::new   
//                                             (   trans_rect_in_pixels.0 as f32, 
//                                                 trans_rect_in_pixels.1 as f32   ));
//         button.set_color2(color);
//         return (button, window)
//     }


//     pub fn set_color2(&mut self, color: Point3<f32>) {
//         self.rect.set_color( color.x, color.y, color.z);//Point3::new(0.15, 0.3, 0.05);
//     }

//     pub fn draw_text(&mut self, mut window: Window) -> Window {
//         let font = Font::default();
//         window.draw_text(
//             "5",
//             &self.text_point,
//             self.font_size,
//             &font,
//             &Point3::new(1.0, 1.0, 1.0),
//         );
//         return window;
//     }

//     pub fn click_is_inside_button(&mut self, mouse_pos: &Mouse_pos ) -> bool {
       
//         let b =  mouse_pos.x() < &(self.mid_position.x + self.size.0)
//               && mouse_pos.x() > &(self.mid_position.x - self.size.0)
//               && mouse_pos.y() < &(self.mid_position.y + self.size.1)
//               && mouse_pos.y() > &(self.mid_position.y - self.size.1);
//         if b {
//                 self.set_color2(Point3::new(0.15, 0.2, 0.04));
//         }
//         println!("{:?}, {:?}",mouse_pos, self.mid_position.x + self.size.0);
//         b
//     }
// }

// // #[derive( Clone)]
// pub enum TextField<'a> {
//      Text('a Box<String>),
// }

// impl<'a> TextField<'a> {
//     // fn change_field(self) -> &str {
//     //     if let TextField::Text(texto) = self {
//     //         return TextField::Text(texto.push_str(mut "123"));
//     //     } else {
//     //         return ""
//     //     }
//     // }

//     fn get(& self) -> &  String {
//         if let TextField::Text(texto) = self {
//             return texto
//         } else {
//             return &String::from("")
//         }
        
//     }

//     fn print_text(& self, mut window: Window) -> (Self, Window) {
//         let font = Font::default();
//         if let TextField::Text(ref texto) = self  {
//             let font = Font::default();
//             window.draw_text(
//                 &texto,
//                 &Point2::new(100.0,100.0),
//                 120.0,
//                 &font,
//                 &Point3::new(1.0, 1.0, 1.0),
//             );
//         }
//         return (*self, window);
        
//     }
// }

// fn main() {
//     let window = Window::new("Kiss3d: rusty calculator");

//     // let mut equation = String::from("-");
//     let mut equation = Box::new(String::from("-"));
//     let mut equation_field = TextField::Text(equation);
    

//     let screen_width = (window.width() as f64  ) ;
//     let screen_height = (window.height() as f64 ) ;

//     // Button
//     let square_side = 100.0;
//     let trans_rect_in_pixels =  (250.0, -200.0);
//     let (mut button_digit, mut window) = Button::new(trans_rect_in_pixels, (square_side, square_side), window);

//     //initialization
//     let mut mouse_pos: Mouse_pos = Mouse_pos(0.0,0.0);
//     let mut mx = 0.0;
//     let mut my = 0.0;
//     let mut a_button_has_been_pressed = &false;

//     while window.render() {
//         // need to call draw_text at every frame because kiss3d erases the text at every frame
//         window = Button::draw_text(&mut button_digit, window);
//         let q = TextField::print_text( & equation_field, window);
//         // let q2 = TextField::get(& equation_field);
//         // TextField::print_text( & equation_field, window);
        
//         // equation_field = q.0;
//         window = q.1;
//         // 
        
//         for mut event in window.events().iter() {
//             match event.value {
//                 WindowEvent::Key(button, Action::Press, _) => {
//                     println!("You pressed the mouse button: {:?}", button);
//                 }
//                 WindowEvent::Key(button, Action::Release, _) => {
//                     println!("You pressed the mouse button: {:?}", button);
//                 }
//                 WindowEvent::MouseButton(button, Action::Press, _) => {
//                     // Button::set_color2(&mut button_digit, Point3::new(0.22,0.33,0.44));
//                     if Button::click_is_inside_button(&mut button_digit, &mouse_pos) {
//                         a_button_has_been_pressed = &true;

//                         equation = String::from("4");
//                         // window = TextField::print_text(&mut equation_field, window);
//                     };

//                     println!("You pressed the mouse button: {:?}", button);

//                 }
//                 WindowEvent::MouseButton(button, Action::Release, _) => {
//                     // rect.set_color(0.15, 0.5, 0.04);
//                     Button::set_color2(&mut button_digit, Point3::new(0.15, 0.5, 0.04));
//                     println!("You released the mouse button: {:?}", button);
//                     // println!("{}", q.0)

//                 }
//                 WindowEvent::CursorPos(x, y, _) => {
//                     mouse_pos = Mouse_pos(x / screen_width, y  / screen_height) ;
//                 }

//                 _ => {}
//             }
//         }


//     }
// }







extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::text::Font;
use kiss3d::window::Window;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::scene::PlanarSceneNode;
use na::{Point2, Point3};
use na::{Translation2, UnitComplex, Isometry2, Vector2};

#[derive(Debug)]
pub struct Mouse_pos(f64, f64);
impl Mouse_pos {
    fn x(&self) -> &f64 {
        if let Mouse_pos(x,_) = self {
            x
        } else {
            &0.0
        }
    }

    fn y(&self) -> &f64 {
        if let Mouse_pos(_,y) = self {
            y
        } else {
            &0.0
        }
    }
}


#[allow(dead_code)]
pub struct Button {

    mid_position: Point2<f64>,
    size: (f64, f64),
    font_size: f32,
    color: Point3<f32>,
    label: String,
    rect: PlanarSceneNode,
    text_point: Point2<f32>,
} 

impl Button {
    pub fn new(trans_rect_in_pixels: (f64, f64) , rec_size_in_pixels: (f64, f64), window: &mut Window ) -> Button {
        let screen_width = (window.width() as f64  ) ;
        let screen_height = (window.height() as f64 ) ;
        let rec_size =  ( rec_size_in_pixels.0 as f64 / screen_width
                    , rec_size_in_pixels.1 as f64 / screen_height
                    );

        // I have no idea why I need that
        let what = 2.0_f64.sqrt(); // wtf, why do I even need this
        let trans_rect =    ( trans_rect_in_pixels.0 as f64 / screen_width
                            , trans_rect_in_pixels.1 as f64 / screen_height);
        let button_mid = ((trans_rect.0)*what + 0.5 , (-trans_rect.1)*what + 0.5 ) ;
        let button_size = ((rec_size.0 / what) as f64, (rec_size.1 / what) as f64 );

       
        let color = Point3::new(0.0, 0.5, 0.2);
        let font_size = 240.0;
        
        // I have no idea why I need that either
        let what_again = 2.0 * 2.0_f32.sqrt();
        let t = Isometry2::new(Vector2::new(    -font_size/2.0*0.475
                                            ,   -font_size/2.0), 0.0);

        let t2 = Isometry2::new( Vector2::new(2.0 * (screen_width / 2.0) as f32 
                                                    + trans_rect_in_pixels.0 as f32 *what_again
                                            ,2.0 * (screen_height / 2.0) as f32 
                                                    - trans_rect_in_pixels.1 as f32 *what_again), 0.0);
        let p3 = Point2::new(0.0, 0.0);
        let p4 = (t * t2 * p3);

        let mut button = Button {
            mid_position: Point2::new(button_mid.0, button_mid.1),
            size:(button_size.0, button_size.1),
            font_size: font_size,
            label: String::from("6"),
            color: color,
            rect: window.add_rectangle( 
                        rec_size_in_pixels.0 as f32
                    ,   rec_size_in_pixels.1 as f32     
                    ),
            text_point: p4,
        };

        button.rect.append_translation(&Translation2::new   
                                            (   trans_rect_in_pixels.0 as f32, 
                                                trans_rect_in_pixels.1 as f32   ));
        button.set_color2(color);
        return button
    }


    pub fn set_color2(&mut self, color: Point3<f32>) {
        self.rect.set_color( color.x, color.y, color.z);//Point3::new(0.15, 0.3, 0.05);
    }

    pub fn draw_text(&mut self, window: &mut Window) { //-> Window {
        let font = Font::default();
        window.draw_text(
            "5",
            &self.text_point,
            self.font_size,
            &font,
            &Point3::new(1.0, 1.0, 1.0),
        );

    }

    pub fn click_is_inside_button(&mut self, mouse_pos: &Mouse_pos ) -> bool {
       
        let b =  mouse_pos.x() < &(self.mid_position.x + self.size.0)
              && mouse_pos.x() > &(self.mid_position.x - self.size.0)
              && mouse_pos.y() < &(self.mid_position.y + self.size.1)
              && mouse_pos.y() > &(self.mid_position.y - self.size.1);
        if b {
                self.set_color2(Point3::new(0.15, 0.2, 0.04));
        }
        println!("{:?}, {:?}",mouse_pos, self.mid_position.x + self.size.0);
        b
    }
}

// #[derive( Clone)]
pub enum TextField{
     Text(Box < String  >),
}

impl TextField {
    // fn change_field(self) -> &str {
    //     if let TextField::Text(texto) = self {
    //         return TextField::Text(texto.push_str(mut "123"));
    //     } else {
    //         return ""
    //     }
    // }

    fn get(& self) ->  String {
        if let TextField::Text(texto) = self {
            return texto.to_string()
        } else {
            return String::from("")
        }
        
    }

    fn print_text(& self, window: &mut Window)  {
        let font = Font::default();
        if let TextField::Text(ref texto) = self  {
            let font = Font::default();
            window.draw_text(
                &texto,
                &Point2::new(100.0,100.0),
                120.0,
                &font,
                &Point3::new(1.0, 1.0, 1.0),
            );
        }
        
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: rusty calculator");

    let mut equation = String::from("-");
    // let mut equation = "-"));
    let mut equation_field = TextField::Text(Box::new(equation));
    equation = String::from("4");
    
    

    let screen_width = (window.width() as f64  ) ;
    let screen_height = (window.height() as f64 ) ;

    // Buttons
    let square_side = 100.0;
    let trans_rect_in_pixels =  (250.0, -200.0);

    // Buttons
    let trans_rect_in_pixels2 =  (-250.0, -200.0);
    // let mut button_digit; 

    //initialization
    let mut mouse_pos: Mouse_pos = Mouse_pos(0.0,0.0);
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut a_button_has_been_pressed = &false;

    // let (mut button_digit, mut window) = Button::new(trans_rect_in_pixels, (square_side, square_side), window);

    
    // let (mut button_digit,  mut window) = Button::new(trans_rect_in_pixels, (square_side, square_side), window);
    let mut button_digit = Button::new(trans_rect_in_pixels, (square_side, square_side), &mut window);

    let mut button_digit2 = Button::new(trans_rect_in_pixels2, (square_side, square_side), &mut window);

    // let (mut buttons,mut window) = {
    //     let mut v: Vec<Button> = Vec::new();
    //     for i in (-1..-1) {
    //         let (mut button_digit, mut window) = Button::new(trans_rect_in_pixels, (square_side, square_side), window);
    //         v.push(button_digit);
    //     };
    //     (v, window)
    // };

    while window.render() {
        // need to call draw_text at every frame because kiss3d erases the text at every frame
        // window = Button::draw_text(&mut buttons[0], window);
        // window = Button::draw_text(&mut button_digit, &mut window);
        Button::draw_text(&mut button_digit, &mut window);
        // window = TextField::print_text( & equation_field, window);

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    println!("You pressed the mouse button: {:?}", button);
                }
                WindowEvent::Key(button, Action::Release, _) => {
                    println!("You pressed the mouse button: {:?}", button);
                }
                WindowEvent::MouseButton(button, Action::Press, _) => {
                    // Button::set_color2(&mut button_digit, Point3::new(0.22,0.33,0.44));
                    if Button::click_is_inside_button(&mut button_digit, &mouse_pos) {
                    //     a_button_has_been_pressed = &true;

                        // equation = String::from("5");
                        // equation_field = TextField::Text(Box::new(equation));

                        // window = TextField::print_text(&mut equation_field, window);
                    };
                    if Button::click_is_inside_button(&mut button_digit2, &mouse_pos) {
                    }

                    println!("You pressed the mouse button: {:?}", button);

                }
                WindowEvent::MouseButton(button, Action::Release, _) => {
                    // rect.set_color(0.15, 0.5, 0.04);
                    Button::set_color2(&mut button_digit, Point3::new(0.15, 0.5, 0.04));
                    Button::set_color2(&mut button_digit2, Point3::new(0.15, 0.5, 0.04));
                    // println!("You released the mouse button: {:?}", button);
                    // // println!("{}", q.0)

                }
                WindowEvent::CursorPos(x, y, _) => {
                    mouse_pos = Mouse_pos(x / screen_width, y  / screen_height) ;
                }

                _ => {}
            }
        }


    }
}




















// // #![feature(box_syntax, box_patterns)]

// use std::io;
// // use std::io::prelude::v1::*;
// // use std::fmt::Display;

// use self::Token::*;
// use std::f64;
// use itertools::*;
// use std::str;
// use Token::*;

// #[derive(Debug,PartialEq, Clone,PartialOrd)]
// pub enum Token {
//     LPAREN,
//     RPAREN,
//     ADD,
//     SUB,
//     MUL,
//     DIV,
//     CARET,
//     EQUALS,
//     NUMBER(f64),
//     SYMBOL(String),
//     EOF
// }

// impl Token {
//     /* returns (prec, associativity) where 0 is left and 1 is right*/
//     pub fn info(&self) -> Option<(usize, usize)> {
//         match *self {
//             ADD | SUB => Some((10, 0)),
//             MUL | DIV => Some((20, 0)),
//             CARET => Some((30, 1)),
//             NUMBER(_) => Some((40,0)),
//             _ => { None }
//         }
//     }

//     pub fn to_char(&self) -> char {
//         match *self {
//             LPAREN => '(',
//             RPAREN => ')',
//             ADD => '+',
//             SUB => '-',
//             MUL => '*',
//             DIV => '/',
//             CARET => '^',
//             EQUALS => '=',
//             EOF => 'E',
//             NUMBER(_) => 'N',
//             SYMBOL(_) => 'S',
//         }
//     }

// }

// pub fn to_tokens(c: &str) -> Token {
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



// pub fn is_eof(t: &Token) -> bool{
//     match t {
//         &EOF => true,
//         _ => false
//     }
// }

// pub fn splitOnTokens(equation: Vec<&str>) -> Vec<String> {
//     let groups: Vec<String> = equation.iter()
//         .group_by(|elt| elt.chars() .next().unwrap().is_digit(10) )
//         .into_iter()
//         .map(|(_, mut group)|  group . join("")  )
//         .collect();
//     return groups

// }


// #[derive(PartialEq,Debug)]
// struct Node<'a> {
//     val:  (&'a Token, &'a usize) ,
//     l: Option<Box<Node<'a>>>,
//     r: Option<Box<Node<'a>>>,
// }
// impl<'a> Node<'a> {

//     // pub fn deleteRoot(&mut self) -> Node  {
//     //     self=  &mut self.r ;
//     //     match target_node {
            
//     //         &mut Some(ref mut Node {val: v, l:l, r:r} ) => { 
//     //             *self = Some{ subnode }; 
//     //         },
//     //         _ => { self; },
//     //     }

     
//     // } //expected `()`, found `&mut Node<'a>`


//     pub fn insert(&mut self, new_val:  (&'a Token, &'a usize) ) {

//         let target_node = if new_val.1 < self.val.1 { &mut self.l } else { &mut self.r };
//         match target_node {
//             &mut Some(ref mut subnode) => subnode.insert(new_val),
//             &mut None => {
//                 let new_node = Node { val: new_val, l: None, r: None };
//                 let boxed_node = Some(Box::new(new_node));
//                 *target_node = boxed_node;
//             }
//         }
//     }

//     pub fn eval(&self) -> f64 {
//         println!("{:?}",self);


//         match (self.val, self.l.as_ref(), self.r.as_ref()) {
//             ( (op,_) 
//             , Some( box_n1 )
//             , Some( box_n2 ) ) => {
//                 match ( box_n1.as_ref(), box_n2.as_ref() ) {

//                     ( Node {val: (NUMBER(n1),_), l: None, r:None}, Node {val: (NUMBER(n2),_), l: None, r:None} )   => {
//                         let result = match op {
//                             ADD =>  { (n1 + n2) } 
//                             SUB =>  { (n1 - n2) }
//                             MUL =>  { (n1 * n2) }
//                             DIV =>  { (n1 / n2) }
//                             CARET => { (n1.powf(*n2)) }
//                             _ => { 0.0 }
//                         };
//                         return result
//                     }

//                     ( Node {val: (NUMBER(n1),_), l: None, r:None},  r )   => {
//                         let temp = r.eval();
//                         let result = match op {
//                             ADD =>  { n1 + temp }
//                             SUB =>  { n1 - temp }
//                             MUL =>  { n1 * temp }
//                             DIV =>  { n1 / temp }
//                             CARET => {  n1.powf(temp)  }
//                             _ => { 0.0 }
//                         };
//                         return result
//                     }

//                     ( l, Node {val: (NUMBER(n2),_), l: None, r:None} )   => {
//                         let r = match op {
//                             ADD =>  { (l.eval() + n2) }
//                             SUB =>  { (l.eval() - n2) }
//                             MUL =>  { (l.eval() * n2) }
//                             DIV =>  { (l.eval() / n2) }
//                             CARET => { (l.eval().powf(*n2)) }
//                             _ => { 0.0 }
//                         };
//                         return r
//                     }

//                     (  l, r )   => {
//                         let r = match op {
//                             ADD =>  {  l.eval() + r.eval()  }
//                             SUB =>  {  l.eval() - r.eval()  }
//                             MUL =>  { l.eval() * r.eval()  }
//                             DIV =>  { l.eval() / r.eval()  }
//                             CARET => { l.eval().powf( r.eval() )  }
//                             _ => { 0.0 }
//                         };
//                         return r
//                     }

//                     _ => { 0.0 }
//                 }
//             }
//             _ => { 0.0 }
                
            
//         }
//     }
// }


// pub fn main() {
//     // let data = vec!["+", "2", "3", "-", "1"];
//     let mut v = String::from("3+3^2^4*2^4+3/3");
//     let v2: Vec<&str> = v.split(""). collect::<Vec<&str>>();
//     let  v3 = &v2[1..(v2.len()-1)];
//     println!("{:?}",v3);
//     let qq = splitOnTokens(v3.to_vec());
//     let  w :  Vec<Token> =  qq.iter().map(|s| to_tokens(s)).collect();

//     let  j: Vec<usize> = (1..w.len()+1).collect() ;
//     println!("len is {:?}",j);
//     let  tokenList: Vec<(&Token, &usize)> = w.iter().zip( j.iter() ).collect();
//     println!("tokenlist: {:?}",tokenList);
//     let mut  tree: Node = Node {val: (&Token::EQUALS, &1), l: None, r: None} ;

//     // tree.insert(&ADD);

//     for i in &tokenList {
//         if let Some((a,b)) = i.0.info() {
//             if a == 10 {
//                 tree.insert(*i);
//             }
//         } 
//     }

//     for i in &tokenList {
//         if let Some((a,b)) = i.0.info() {
//             if a == 20 {
//                 tree.insert(*i);
//             }
//         } 
//     }

//     for i in &tokenList {
//         if let Some((a,b)) = i.0.info() {
//             if a == 30 {
//                 tree.insert(*i);
//             }
//         } 
//     }

//     for i in &tokenList {
//         if let Some((a,b)) = i.0.info() {
//             if a == 40 {
//                 tree.insert(*i);
//             }
//         } 
//     }

//     // fn unbox<T>(value: Box<T>) -> T {
//     //     *value
//     // }
//     let mut  tree2 = *(tree.r.unwrap());
//     let rrr = println!("tree2: {:?}",tree2 );


// }