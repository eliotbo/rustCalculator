extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::text::Font;
use kiss3d::window::Window;
use kiss3d::event::{Action, WindowEvent, Key};
use kiss3d::scene::PlanarSceneNode;
use na::{Point2, Point3};
use na::{Translation2, Isometry2, Vector2};

use std::f64;
use itertools::*;
use std::str;
use Token::*;

#[derive(Debug)]
pub struct MousePos(f64, f64);
impl MousePos {
    fn x(&self) -> &f64 {
        if let MousePos(x,_) = self {
            x
        } else {
            &0.0
        }
    }

    fn y(&self) -> &f64 {
        if let MousePos(_,y) = self {
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
    pub fn new(label: String , trans_rect_in_pixels: (f64, f64) , rec_size_in_pixels: (f64, f64), window: &mut Window ) -> Button {
        let screen_width =  window.width()  as f64  ;
        let screen_height = window.height() as f64  ;
        let rec_size =  ( rec_size_in_pixels.0 as f64 / screen_width
                        , rec_size_in_pixels.1 as f64 / screen_height
                        );


        let what = 2.0_f64.sqrt(); // why do I even need this
        let trans_rect =    ( trans_rect_in_pixels.0 as f64 / screen_width
                            , trans_rect_in_pixels.1 as f64 / screen_height);
        let button_mid = ((trans_rect.0)*what + 0.5 , (-trans_rect.1)*what + 0.5 ) ;
        let button_size = ((rec_size.0 / what) as f64, (rec_size.1 / what) as f64 );

       
        let color = Point3::new(0.0, 0.5, 0.2);
        let font_size = 240.0;
        
        
        let what_again = 2.0 * 2.0_f32.sqrt(); // I have no idea why I need that either
        let t = Isometry2::new(Vector2::new(    -font_size/2.0*0.475
                                            ,   -font_size/2.0), 0.0);

        let t2 = Isometry2::new( Vector2::new(2.0 * (screen_width / 2.0) as f32 
                                                    + trans_rect_in_pixels.0 as f32 *what_again
                                            ,2.0 * (screen_height / 2.0) as f32 
                                                    - trans_rect_in_pixels.1 as f32 *what_again), 0.0);
        let p3 = Point2::new(0.0, 0.0);
        let p4 = t * t2 * p3;

        let mut button = Button {
            mid_position: Point2::new(button_mid.0, button_mid.1),
            size:(button_size.0, button_size.1),
            font_size: font_size,
            label: label,
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
            &self.label,
            &self.text_point,
            self.font_size,
            &font,
            &Point3::new(1.0, 1.0, 1.0),
        );

    }

    pub fn click_is_inside_button(&mut self, mouse_pos: &MousePos ) -> bool {
       
        let b =  mouse_pos.x() < &(self.mid_position.x + self.size.0)
              && mouse_pos.x() > &(self.mid_position.x - self.size.0)
              && mouse_pos.y() < &(self.mid_position.y + self.size.1)
              && mouse_pos.y() > &(self.mid_position.y - self.size.1);
        if b {
                self.set_color2(Point3::new(0.15, 0.2, 0.04));
        }
        b
    }
}

// #[derive( Clone)]
pub struct TextField{
    text: Box < String  >,
    pos: Point2<f32>,
}

impl TextField {
    fn change_field(&mut self, digit_or_op: &str) {
        let mut tt = TextField::get(&self);
        tt.push_str(digit_or_op);
        self.text = Box::new(tt.to_string());
    }

    fn get(& self) ->  String {
        match self {
            TextField { text, pos: _ } => { (*text).to_string() },
        }
    }

    fn print_text(& self, window: &mut Window)  {
        window.draw_text(
            &*self.text,
            &self.pos,
            120.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        )
    }
}

fn draw_stuff(  solve_button: &mut Button, 
                buttons: &mut Vec<Button>, 
                text_field: &mut TextField, 
                answer_field: &mut TextField,
                mut window: &mut Window)        {

    for button in buttons {
        Button::draw_text( button, &mut window);
    }
    TextField::print_text( & text_field, &mut window);
    TextField::print_text( & answer_field, &mut window);
    Button::draw_text( solve_button, &mut window);
}

fn main() {
    let mut window = Window::new("Kiss3d: rusty calculator");

    let mut equation_field = TextField {
                                text: Box::new( String::from("=") ),
                                pos : Point2::new(100.0, 50.0),
    };

    let mut answer_field   = TextField {
                                text: Box::new( String::from("---") ),
                                pos : Point2::new(100.0, 150.0),
    };
    
    let screen_width =  window.width()  as f64 ;
    let screen_height = window.height() as f64 ;

    //initialization
    let mut mouse_pos: MousePos = MousePos(0.0,0.0);

    // Buttons
    let square_side = 100.0;
    
    let mut buttons: Vec<Button> = {
        let mut v: Vec<Button> = Vec::new();
        let mut button_digit0;
        let trans_rect_in_pixels_base =  (110.0, 110.0);
        let mut digits = 1..10;
        for j in -1..2 {
            for i in -1..2 {
                let trans_rect_in_pixels0 = 
                    ( trans_rect_in_pixels_base.0 * (i as f64)
                    , trans_rect_in_pixels_base.1 * (j as f64) );
                button_digit0 = Button::new(  String::from( digits.next().unwrap().to_string() )
                                            , trans_rect_in_pixels0
                                            , (square_side, square_side), &mut window);
                v.push(button_digit0);
            };
        };
        v
    };

    buttons.push( Button::new(String::from("0"), (-110.0, -220.0), (square_side, square_side), &mut window) );
    buttons.push( Button::new(String::from("+"), (-220.0, 110.0),  (square_side, square_side), &mut window) );
    buttons.push( Button::new(String::from("-"), (-220.0, 0.0),    (square_side, square_side), &mut window) );
    buttons.push( Button::new(String::from("x"), (-220.0, -110.0), (square_side, square_side), &mut window) );
    buttons.push( Button::new(String::from("/"), (-220.0, -220.0), (square_side, square_side), &mut window) );

    let mut solve_button = Button::new(String::from("="), (55.0, -220.0), (210.0, 100.0), &mut window) ;

    while window.render() {
        draw_stuff(
            &mut solve_button, 
            &mut buttons, 
            &mut equation_field, 
            &mut answer_field,
            &mut window
            );

        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    println!("You pressed the mouse button: {:?}", button);
                    if button == Key::Back {
                        equation_field.text.pop();
                    }
                }
                WindowEvent::Key(button, Action::Release, _) => {
                    println!("You pressed the mouse button: {:?}", button);
                }

                WindowEvent::MouseButton(_button, Action::Press, _) => {
                    for b in &mut buttons {
                        if Button::click_is_inside_button( b, &mouse_pos) {
                            equation_field.change_field(&b.label );
                            // equation_field.text = Box::new(b.label)
                        };
                    }

                    if Button::click_is_inside_button( &mut solve_button, &mouse_pos )  {
                        // eval( equation_field.get() );
                        println!("{}", equation_field.text.to_string() );
                        let answer = eval( equation_field.text.to_string() );
                        answer_field.text = Box::new( answer.to_string() );
                    }
                }

                WindowEvent::MouseButton(_button, Action::Release, _) => {
                    // rect.set_color(0.15, 0.5, 0.04);
                    for b in &mut buttons {
                        Button::set_color2( b, Point3::new(0.15, 0.5, 0.04));
                    }
                    Button::set_color2( &mut solve_button, Point3::new(0.15, 0.5, 0.04));
                }

                WindowEvent::CursorPos(x, y, _) => {
                    mouse_pos = MousePos(x / screen_width, y  / screen_height) ;
                }

                _ => {}
            }
        }
    }
}


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
        "*" | "x" => MUL,
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

pub fn split_on_tokens(equation: Vec<&str>) -> Vec<String> {
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


        match ( self.val, 
                self.l.as_ref(), 
                self.r.as_ref()) {
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

                }
            }
            _ => { 0.0 }
                
            
        }
    }
}

pub fn eval(equation: String) -> f64 {
    // let data = vec!["+", "2", "3", "-", "1"];
    // let mut v = String::from("3+3^2^4*2^4+3/3");
    let v2: Vec<&str> = equation.split(""). collect::<Vec<&str>>();
    let  v3 = &v2[1..(v2.len()-1)];
    println!("{:?}",v3);
    let qq = split_on_tokens(v3.to_vec());
    let  w :  Vec<Token> =  qq.iter().map(|s| to_tokens(s)).collect();

    let  j: Vec<usize> = (1..w.len()+1).collect() ;
    println!("len is {:?}",j);
    let  token_list: Vec<(&Token, &usize)> = w.iter().zip( j.iter() ).collect();
    println!("tokenlist: {:?}",token_list);
    let mut  tree: Node = Node {val: (&Token::EQUALS, &1), l: None, r: None} ;

    for i in &token_list {
        match i.0.info() {
            Some((a,_b)) if a == 10 => { tree.insert(*i); },
            _ => {},
        }
    }

    for i in &token_list {
        match i.0.info() {
            Some((a,_b)) if a == 20 => { tree.insert(*i); },
            _ => {},
        }
    }

    for i in &token_list {
        match i.0.info() {
            Some((a,_b)) if a == 30 => { tree.insert(*i); },
            _ => {},
        }
    }

    for i in &token_list {
        match i.0.info() {
            Some((a,_b)) if a == 40 => { tree.insert(*i); },
            _ => {},
        }
    }


    let tree2 = *(tree.r.unwrap());
    println!("{}", tree2.eval());
    return tree2.eval()



}