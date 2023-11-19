use raylib::Rectangle;

pub struct UIwidget {
    loc : Rectangle,
    focusable : Bool,
    childs : &mut Vec<T : Draw>,
}

struct Constraint {
    x : usize,
    y : usize,
    width : usize,
    height : usize
}

impl UIwidget {
   fn new(loc : Rectangle, focusable : Bool, childs : &mut Vec<T : Draw>) -> UIwidget{
       UIwidget { loc, focusable, childs }
   }
}

pub trait UIComponent {
    trait Draw {
        fn draw(&self,contraint : Constraint);
    }

    trait Event {
        fn onClick();
        fn onOver();
        fn endOver();
    }
}

impl Draw for UIwidget {
    fn draw(&self) {
        self.childs.draw()
    }
}
