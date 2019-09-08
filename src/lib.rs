use axgeom;
use ascii_num;
use axgeom::*;
use ascii_num::*;
use ascii_num::*;



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


pub struct Button<'a>{
    poses:symbol::Symbol<'a>,
    dim:axgeom::Rect<f32>,
    padding:axgeom::Rect<f32>,
    spacing:f32
}

impl<'a> Button<'a>{
    pub fn get_dim(&self)->&axgeom::Rect<f32>{
        &self.padding
    }
    pub fn new(topleft:Vec2<f32>,poses:symbol::Symbol<'a>,spacing:f32)->Button{
        let m=poses.get().iter().fold(vec2same(0), |acc, &v| {vec2(acc.x.max(v.x),acc.y.max(v.y))});
        
        let dimx=m.x as f32*spacing;
        let dimy=m.y as f32*spacing;
        let k=topleft;//get();
        let dim=axgeom::Rect::new(k.x,k.x+dimx,k.y,k.y+dimy);
        
        let mut padding=dim;
        padding.grow(spacing*2.0);
        Button{poses,dim,padding,spacing}
    }
    pub fn iter(&self)->ButtonPosesIter{
        let topleft=vec2(self.dim.x.left,self.dim.y.left);
        ButtonPosesIter{poses:self.poses.get().iter(),topleft,spacing:self.spacing}
    }
}





//make it right to left.
pub struct DigitIter<'a>{
    digit_iter:core::iter::Enumerate<core::iter::Rev<digit::DigitIter<'a>>>,
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
                Some(ButtonPosesIter{poses:digit.into_inner().iter(),topleft,spacing})
            }
        }
    }
}


pub struct NumberThing<'a>{
    number:digit::Number<'a>,
    pixel_spacing:f32,
    digit_spacing:f32,
    top_right:Vec2<f32>
}

impl<'a> NumberThing<'a>{
    pub fn new(number:digit::Number<'a>,digit_spacing:f32,pixel_spacing:f32,top_right:Vec2<f32>)->NumberThing<'a>{
        NumberThing{number,pixel_spacing,digit_spacing,top_right}
    }
    pub fn update_number(&mut self,number:usize){
        self.number.update_number(number);
    }
    pub fn get_number(&self)->usize{
        self.number.get_number()
    }
    pub fn iter(&self)->DigitIter{
        DigitIter{
            digit_iter:self.number.iter().rev().enumerate(),
            spacing:self.pixel_spacing,
            digit_spacing:self.digit_spacing,
            top_right:self.top_right}
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



