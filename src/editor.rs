
// author : Akheel Sheik
mod terminal;
use crossterm::event::{read, Event::Key,KeyEvent,KeyModifiers,Event};
use crossterm::event::{KeyCode,KeyEventKind};
use terminal::{Terminal,Size,Position};
use core::cmp::{min,max};
pub struct Editor {
    should_quit: bool,
    mouse_position: Position
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn new() -> Self {
        Editor { should_quit: false, mouse_position: Position{x:0,y:0} }
    }

    pub fn run(&mut self) {
        Terminal::initalize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        if self.should_quit{
            Terminal::clear_screen()?;
            Terminal::write("Goodbye!");
        } else {
            Terminal::move_to_cursor(Position {x:0,y:0});
            Self::draw_rows();
            Terminal::move_to_cursor(self.mouse_position);
        }
        Terminal::show_cursor();
        Terminal::flush();
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.eval(&event);
        }
        Ok(())
    }
    fn read() -> Result<Event, std::io::Error> {
        return Ok(read()?);
    }

    fn eval(&mut self, event: &Event) {
        if let Key(KeyEvent {code,modifiers,kind: KeyEventKind::Press,..}) = event {
            // println!("{code} {modifiers}");
            match code {
                KeyCode::Char('q') => {
                        if *modifiers == KeyModifiers::CONTROL {
                            self.should_quit = true;
                        }
                    },
                KeyCode::Right 
                | KeyCode::Left
                | KeyCode::Down
                | KeyCode::Up
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => {
                    self.move_point(*code);
                },
                _ => (),
                }
        }
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(),std::io::Error> {
        let Size{height,width} = Terminal::term_size()?;
        match key_code {
            KeyCode::Left => {
                self.mouse_position.x = max(0,self.mouse_position.x.saturating_sub(1));
            },
            KeyCode::Right => {
                self.mouse_position.x = min(self.mouse_position.x.saturating_add(1),width-1);
            },
            KeyCode::Up => {
                self.mouse_position.y = max(0,self.mouse_position.y.saturating_sub(1));
            },
            KeyCode::Down => {
                self.mouse_position.y = min(self.mouse_position.y.saturating_add(1),height-1);
            },
            KeyCode::PageUp => {
                self.mouse_position.y = 0;
            },
            KeyCode::PageDown => {
                self.mouse_position.y = height-1;
            },
            KeyCode::Home => {
                self.mouse_position.x = 0;
            },
            KeyCode::End => {
                self.mouse_position.x = width-1;
            },
            _ => (),
        }
        Ok(())

    }

    fn draw_rows() -> Result<(),std::io::Error> {

        let Size{height,width} = Terminal::term_size()?;
        let character = "~";
        let message = format!("{NAME} -- version {VERSION}");
        let message_length = message.len() as u16;
        let half_message:u16 = message_length/2;
        let main_position_x = width/2 - half_message;
        let vertical = (height/3) as u16;
        let first_set_of_space = " ".repeat((main_position_x-1).into());
        
        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::write(character)?;
            
            if i == vertical {
                Terminal::write(&first_set_of_space)?;
                Terminal::write(&message)?;
            }

            if i.saturating_add(1) < height {
                Terminal::write("\r\n")?;
            }
        }
       Terminal::flush();
       Ok(())
    }
}    