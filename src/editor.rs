
// author : Akheel Sheik
mod terminal;
mod view;
mod buffer;
use crossterm::event::{read, Event::Key,KeyEvent,KeyModifiers,Event};
use crossterm::event::{KeyCode,KeyEventKind};
use terminal::{Terminal,Size,Position};
use view::{View};
use core::cmp::{min,max};


#[derive(Copy,Clone,Default)]
struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}



impl Editor {

    pub fn new() -> Self {
        Editor { should_quit: false, location: Location{x:0,y:0},view:View::default() }
    }

    pub fn check(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(first_arg) = args.get(1) {
            self.view.load(first_arg);
        }
    }

    pub fn run(&mut self) {
        Terminal::initalize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_caret();
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::write("Goodbye!");
        } else {
            Terminal::move_to_caret(Position {col:0,row:0});
            self.view.render();
            Terminal::move_to_caret(Position {col:self.location.x,row:self.location.y});
        }
        Terminal::show_caret();
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
                self.location.x = max(0,self.location.x.saturating_sub(1));
            },
            KeyCode::Right => {
                self.location.x = min(self.location.x.saturating_add(1),width.saturating_sub(1).into());
            },
            KeyCode::Up => {
                self.location.y = max(0,self.location.y.saturating_sub(1));
            },
            KeyCode::Down => {
                self.location.y = min(self.location.y.saturating_add(1),height.saturating_sub(1).into());
            },
            KeyCode::PageUp => {
                self.location.y = 0;
            },
            KeyCode::PageDown => {
                self.location.y = height.saturating_sub(1).into();
            },
            KeyCode::Home => {
                self.location.x = 0;
            },
            KeyCode::End => {
                self.location.x = width.saturating_sub(1).into();
            },
            _ => (),
        }
        Ok(())
    }
}    