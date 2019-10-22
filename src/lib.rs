use axgeom;
use ascii_num;
use axgeom::*;
use ascii_num::*;
use ascii_num::*;

use ascii_num::digits_iterator::Digits;



pub struct ButtonPosesIter<'a>{
    poses:core::slice::Iter<'a,Vec2<usize>>,
    topleft:Vec2<f32>,
    spacing:f32
}
impl<'a> Iterator for ButtonPosesIter<'a>{
    type Item=Vec2<f32>;
    fn next(&mut self)->Option<Self::Item>{
        match self.poses.next(){
            None=>None,
            Some(&Vec2{x,y})=>{

                let x=x as f32;
                let y=y as f32;
                
                let dx=self.topleft.x;//self.dim.get_range(axgeom::XAXISS);
                let yx=self.topleft.y;//dim.get_range(axgeom::YAXISS);

                Some(vec2(dx+x*self.spacing,yx+y*self.spacing))
            }
        }
    }
}


pub struct Button{
    symbol:usize, //TODO use new type.
    dim:axgeom::Rect<f32>,
    padding:axgeom::Rect<f32>,
    spacing:f32
}

impl Button{
    pub fn get_dim(&self)->&axgeom::Rect<f32>{
        &self.padding
    }

    ///We need to pass the symbol table so that we can figure out the size of the button
    pub fn new(topleft:Vec2<f32>,symbol:usize,spacing:f32,table:&symbol::SymbolTable)->Button{

        let m=table.lookup(symbol).get().iter().fold(vec2same(0), |acc, &v| {vec2(acc.x.max(v.x),acc.y.max(v.y))});
        
        let dimx=m.x as f32*spacing;
        let dimy=m.y as f32*spacing;
        let k=topleft;//get();
        let dim=axgeom::Rect::new(k.x,k.x+dimx,k.y,k.y+dimy);
        
        let mut padding=dim;
        padding.grow(spacing*2.0);
        Button{symbol,dim,padding,spacing}
    }

    pub fn iter<'a>(&'a self,table:&'a symbol::SymbolTable)->ButtonPosesIter<'a>{
        let topleft=vec2(self.dim.x.left,self.dim.y.left);
        let k=table.lookup(self.symbol);
        ButtonPosesIter{poses:k.into_inner().iter(),topleft,spacing:self.spacing}
    }
}




//Iterates right to left
pub struct DigitIter<'a>{
    table:&'a DigitSymbolTable,
    digit_iter:core::iter::Enumerate<core::iter::Rev<Digits<usize>>>,
    spacing:f32,
    digit_spacing:f32,
    top_right:Vec2<f32>,
}

impl<'a> Iterator for DigitIter<'a>{
    type Item=ButtonPosesIter<'a>;
    fn next(&mut self)->Option<Self::Item>{
        match self.digit_iter.next(){
            None=>None,
            Some((index,digit))=>{
                let spacing=self.spacing;
                let topleft=vec2(self.top_right.x-(index as f32)*self.digit_spacing,self.top_right.y);
                let k=self.table.lookup_digit(digit);
                Some(ButtonPosesIter{poses:k.into_inner().iter(),topleft,spacing})
            }
        }
    }
}


pub struct NumberThing{
    number:usize,
    pixel_spacing:f32,
    digit_spacing:f32,
    top_right:Vec2<f32>
}

impl NumberThing{
    pub fn new(number:usize,digit_spacing:f32,pixel_spacing:f32,top_right:Vec2<f32>)->NumberThing{
        NumberThing{number,pixel_spacing,digit_spacing,top_right}
    }
    pub fn update_number(&mut self,number:usize){
        self.number=number;
    }
    pub fn get_number(&self)->usize{
        self.number
    }
    pub fn iter<'a>(&'a self,table:&'a DigitSymbolTable)->DigitIter<'a>{
        DigitIter{
            table,
            digit_iter:Digits::new(self.number).rev().enumerate(),
            spacing:self.pixel_spacing,
            digit_spacing:self.digit_spacing,
            top_right:self.top_right}
    }
}


use ascii_num::digit::DigitSymbolTable;

pub struct PinDigitIter<'a>{
    digit_iter:core::iter::Enumerate<core::slice::Iter<'a,Option<u8>>>,
    table:&'a DigitSymbolTable,
    spacing:f32,
    digit_spacing:f32,
    top_left:Vec2<f32>,
}
impl<'a> Iterator for PinDigitIter<'a>{
    type Item=ButtonPosesIter<'a>;
    fn next(&mut self)->Option<Self::Item>{
        match self.digit_iter.next(){
            None=>None,
            Some((index,&digit))=>{
                let ff = match digit{
                    Some(digit)=>{
                        self.table.lookup_digit(digit)
                    },
                    None=>{
                        self.table.lookup_digit(10)
                    }
                };
                let spacing=self.spacing;
                let topleft=vec2(self.top_left.x+(index as f32)*self.digit_spacing,self.top_left.y);
                Some(ButtonPosesIter{poses:ff.into_inner().iter(),topleft,spacing})
            },
            _=>{
                unreachable!()
            }
        }
    }
}





#[derive(Copy,Clone)]
pub enum PinEnterResult{
    Open,
    Fail,
    NotDoneYet
}
pub struct PinCode{
    key:[u8;4],
    digits:[Option<u8>;4],
    top_left:Vec2<f32>,
    digit_spacing:f32,
    pixel_spacing:f32
}

impl PinCode{
    pub fn new(top_left:Vec2<f32>,digit_spacing:f32,pixel_spacing:f32)->PinCode{
        PinCode{key:[7,9,8,5],digits:[None,None,None,None],top_left,digit_spacing,pixel_spacing}
    }
    pub fn iter<'a>(&'a self,table:&'a DigitSymbolTable)->PinDigitIter<'a>{
        PinDigitIter{
            table,
            digit_iter:self.digits.iter().enumerate(),
            digit_spacing:self.digit_spacing,
            top_left:self.top_left,
            spacing:self.pixel_spacing}
    }

    pub fn reset(&mut self){
        for a in self.digits.iter_mut(){
           *a=None;
        }
    }

    pub fn add(&mut self,digit:u8)->PinEnterResult{
        assert!(digit>=0 && digit<10);
        for a in self.digits.iter_mut(){
            if a.is_none(){
                *a=Some(digit);
                break;
            }
        }


        if !self.digits.iter().all(|a|a.is_some()){
            return PinEnterResult::NotDoneYet
        }

        if self.digits.iter().zip(self.key.iter()).all(|(a,b)|a.unwrap() ==*b){
            PinEnterResult::Open
        }else{
            PinEnterResult::Fail
        }

    }
}




pub struct Clicker{
    there_was_finger:bool,
    there_is_finger:bool
}
impl Clicker{
    pub fn new()->Clicker{
        Clicker{there_was_finger:false,there_is_finger:false}
    }
    pub fn update(&mut self,dim:&axgeom::Rect<f32>,poses:&[Vec2<f32>])->bool{

        for i in poses.iter(){
            if dim.contains_point(*i){
                self.there_is_finger=true;
            } 
        }
        let ret=if !self.there_was_finger & self.there_is_finger{
            // If the button is pushed and wasn't before change color
            //graphy.set_bot_color(COLS[self.col_counter]);
            //self.col_counter=(self.col_counter+1) % COLS.len();
            true
        }else{
            false
        };
        // Otherwise set stored value to current
        self.there_was_finger = self.there_is_finger;
        // Reset current variable to false
        self.there_is_finger = false;

        ret
    }
}



