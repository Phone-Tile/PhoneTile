use super::{button::{Button, Style, Draw}, colors::{BLACK, WHITE}};

pub struct Keyboard {
    buttons : Vec<Button>,
    entry : String
}


impl Keyboard {
    pub fn new(screen_width : f32, screen_height : f32) -> Self {

        let width = 1.0/(7.0) ; // 3  + 4
        let height = (1.0/2.0) * (1.0 / 9.0) ;

        let style = Style::new(WHITE, BLACK);

        let b1 = Button::new_ratio(1.0 * width , 0.5 + height, width, height, style, Some("1".to_string()), screen_width, screen_height);
        let b2 = Button::new_ratio(3.0 * width , 0.5 + height, width, height, style, Some("2".to_string()), screen_width, screen_height);
        let b3 = Button::new_ratio(5.0 * width , 0.5 + height, width, height, style, Some("3".to_string()), screen_width, screen_height);


        let b4 = Button::new_ratio(1.0 * width , 0.5 + 3.0 * height, width, height, style, Some("4".to_string()), screen_width, screen_height);
        let b5 = Button::new_ratio(3.0 * width , 0.5 + 3.0 * height, width, height, style, Some("5".to_string()), screen_width, screen_height);
        let b6 = Button::new_ratio(5.0 * width , 0.5 + 3.0 * height, width, height, style, Some("6".to_string()), screen_width, screen_height);

        let b7 = Button::new_ratio(1.0 * width , 0.5 + 5.0 *height, width, height, style, Some("7".to_string()), screen_width, screen_height);
        let b8 = Button::new_ratio(3.0 * width , 0.5 + 5.0 *height, width, height, style, Some("8".to_string()), screen_width, screen_height);
        let b9 = Button::new_ratio(5.0 * width , 0.5 + 5.0 *height, width, height, style, Some("9".to_string()), screen_width, screen_height);

        let bdot = Button::new_ratio(1.0 * width , 0.5 + 7.0 * height, width, height, style, Some(".".to_string()), screen_width, screen_height);
        let b0 = Button::new_ratio(3.0 * width , 0.5 + 7.0 * height, width, height, style, Some("0".to_string()), screen_width, screen_height);
        let bdel = Button::new_ratio(5.0 * width , 0.5 + 7.0 * height, width, height, style, Some("<-".to_string()), screen_width, screen_height);
        Keyboard {
            buttons : vec![b1,b2,b3,b4,b5,b6,b7,b8,b9,b0,bdot,bdel],
            entry : "".to_string()
        }
    }

    pub fn get_value(&self) -> String {
        self.entry.clone()
    }

    pub fn reset_value(&mut self) {
        self.entry = "".to_string();
    }

    pub fn update(&mut self) {
        for b in &self.buttons {
            if b.click() {
                match b.get_text() {
                    Some(a) => {
                        if a == "<-" {
                            self.entry.pop();
                        }else{
                            let a_own = a.as_str();
                            self.entry.push_str(a_own)
                        }
                    },
                    None => ()
                }
            }
        }
    }

}

impl Draw for Keyboard {
    fn draw(&self) -> () {
        self.buttons.iter().map(|e| e.draw()).collect()
    }
}
