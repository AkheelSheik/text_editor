
// author : Akheel Sheik
mod terminal;
use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers,Event};
use terminal::{Terminal,Size,Position};
pub struct Editor {
    should_quit: bool,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn new() -> Self {
        Editor { should_quit: false }
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
            Self::draw_rows();
            Terminal::move_to_cursor(Position {x:0,y:0});
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
        if let Key(KeyEvent {code,modifiers,..}) = event {
            println!("{code} {modifiers}");
            match *code {
                Char('q') => {
                        if *modifiers == KeyModifiers::CONTROL {
                            self.should_quit = true;
                        }
                    },
                _ => (),
                }
        }
    }

    fn draw_rows() -> Result<(),std::io::Error> {
        let character = "~";
        let Size{height,width} = Terminal::term_size()?;
        let message = format!("{NAME} -- version {VERSION}");
        let message_length = message.len() as u16;
        let half_message:u16 = message_length/2;

        let main_position_x = width/2 - half_message;
        let vertical = (height/3) as u16;
        let first_set_of_space = " ".repeat((main_position_x-1).into());




        
        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::write(character)?;
            
            if i == height/3 {
                Terminal::write(&first_set_of_space)?;
                Terminal::write(&message)?;
            }

            if i + 1 < height {
                Terminal::write("\r\n")?;
            }
        }
       Terminal::flush();
       Ok(())
    }
}    