use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::na,
};
use failure;
use std::path::Path;
use rand::prelude::*;


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
    tiles: [Tile; 9*9],
    died: bool,
}

impl GameLogic {
    pub fn uncover_cell(&mut self, i: i32) {
        // check adjecent
        let mut bombs = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                let index = i + (c  * 9) + r;
                if index >= 0 && index < 81 {
                    bombs += match self.tiles[index as usize] {
                        Tile::Bomb => 1,
                        _ => 0
                    };
                }
            }
        }

        self.tiles[i as usize] = match bombs {
            0 => Tile::UnCovered,
            x => Tile::Numbered(x)
        };
    }



    fn uncover_cells(&mut self, index: i32, visited: &mut std::collections::HashSet<i32>) {
        visited.insert(index);
        self.uncover_cell(index);

        if self.tiles[index as usize] != Tile::UnCovered {
            return;
        }


        for r in -1..=1 {
            for c in -1..=1 {
                let new_index = index + (c  * 9) + r;
                if new_index >= 0 && new_index < 81 && !visited.contains(&new_index) {
                    self.uncover_cells(new_index, visited);
                }
            }
        }

    }


    fn place_bomb(&mut self, index: usize) {
        self.tiles[index] = Tile::Bomb;
    }

    fn point_to_index(p: Point) -> usize {
        p.x * 9 + p.y
    }

    fn has_won(&self) -> bool {
        for tile in &self.tiles {
            if *tile == Tile::Hidden {
                return false;
            }
        }
        true
    }

    fn clear(&mut self) {
        for tile in self.tiles.iter_mut() {
            *tile = Tile::Hidden;
        }

        self.died = false;
    }

    fn initialize(&mut self, pressed_index: usize) {
        self.clear();
        self.initialized = true;


        let mut rng = rand::thread_rng();
        let mut indexes: Vec<usize> = (1..81).collect();
        indexes.shuffle(&mut rng);

        let mut bombs = 10;
        let mut i = 0;
        while bombs > 0 {

            if indexes[i] != pressed_index {
                println!("Placing at {:?}", indexes[i]);
                self.place_bomb(indexes[i]);
            }

            i += 1;
            bombs -= 1;
        }
    }
}

impl Default for GameLogic {
    fn default() -> Self {
        Self {
            initialized: false,
            tiles: [Tile::Hidden; 9*9],
            died: false
        }
    }
}




impl gls::State<Message> for GameLogic {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Restart => {self.clear();},
            Message::Click(p) => {

                if self.died {
                    return;
                }

                if !self.initialized {
                    self.initialize(Self::point_to_index(*p));
                }

                if self.tiles[Self::point_to_index(*p)] == Tile::Bomb {
                    self.died = true;
                    return;
                }

                self.uncover_cells(GameLogic::point_to_index(*p) as i32, &mut std::collections::HashSet::new());

                if self.has_won() {
                    println!("YOU HAVE WON");
                }

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
                 .add(Button::new("Time", None)
                      .height(Px(50))
                 )
                 .add(Button::new("Restart", Some(Message::Restart))
                      .height(Px(50))
                      .align_center()
                 )
                 .add(Button::new("Score", None)
                      .height(Px(50))
                      .align_right()
                 ))
            .add(GameLayout::new(GameInfo { tiles: self.tiles.clone(), died: self.died }, Message::Click)
                 .height(Fill));
        col.into()
    }
}
