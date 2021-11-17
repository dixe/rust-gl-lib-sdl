use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::na,
};

use failure;
use std::path::Path;

mod game;

use game::*;


#[derive(Debug, Clone)]
pub enum Message {
    Restart,
    Click(game::Point),
}

fn main() -> Result<(), failure::Error> {
    let width = 600;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("MineSweeper", width, height, font).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();


    let mut state = GameLogic::default();

    while !window.should_quit() {
        window.update(&mut state);
    }

    Ok(())
}



#[derive(Debug, Clone)]
struct GameLogic {
    initialized: bool,
    mines: [Point; 10],
    tiles: [Tile; 9*9],
}

impl Default for GameLogic {
    fn default() -> Self {
        Self {
            initialized: false,
            mines: Default::default(),
            tiles: [Tile::Hidden; 9*9],
        }
    }
}




impl gls::State<Message> for GameLogic {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Restart => { },
            Message::Click(p) => {

                if !self.initialized {
                    self.initialized = true;
                    self.mines[0] = Point::new(2,7);
                    self.mines[1] = Point::new(5,2);
                    self.mines[2] = Point::new(6,7);
                    self.mines[3] = Point::new(7,1);
                    self.mines[4] = Point::new(7,2);
                    self.mines[5] = Point::new(7,5);
                    self.mines[6] = Point::new(7,7);
                    self.mines[7] = Point::new(8,1);
                    self.mines[8] = Point::new(8,4);
                    self.mines[9] = Point::new(8,7);

                }

                for m in &self.mines {
                    if p == m {
                        println!("BOMB at {}", p);
                    }
                }

                let i = p.x * 9 + p.y;

                self.tiles[i] = Tile::UnCovered;

            },
        }
    }


    fn view(&self) -> gls::layout::node::Node<Message> {
        use gls::layout::row::*;
        use gls::layout::column::*;
        use gls::layout::element::*;
        use gls::layout::attributes::*;
        use gls::layout::button::*;

        use Length::*;

        let col = Column::new()
            .add(Row::new()
                 .padding(5.0)
                 .width(Fill)
                 .add(Button::new("Time", Some(Message::Restart))
                      .height(Px(50))
                 )
                 .add(Button::new("Restart", Some(Message::Restart))
                      .height(Px(50))
                      .align_center()
                 )
                 .add(Button::new("Score", Some(Message::Restart))
                      .height(Px(50))
                      .align_right()
                 ))
            .add(GameLayout::new(self.tiles.clone(), Message::Click)
                 .height(Fill));
        col.into()
    }
}
