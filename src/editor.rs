
// author : Akheel Sheik

use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers,Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType,size};
use crossterm::{execute, style::Print};
use crossterm::cursor::{MoveToColumn,MoveToRow,MoveTo,MoveToNextLine,SavePosition,RestorePosition};
use std::io::stdout;


pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Self::initalize().unwrap();
        let result1 = self.draw_tilda();
        let result = self.repl();
        Self::terminate().unwrap();
        result1.unwrap();
        result.unwrap();
    }
    fn initalize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn draw_tilda(&mut self) -> Result<(), std::io::Error> {
        let row_columns = crossterm::terminal::size()?;
        let tilda = "~";
        
        for i in (0..row_columns.1) {
            execute!(stdout(),
            crossterm::cursor::MoveTo(1,i),
            crossterm::style::Print(tilda));;
        }
        Ok(())
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut std_out = stdout();
        execute!(std_out,Clear(ClearType::All))
    }

    fn eval(&mut self, event: &Event) {
        
        if let Key(KeyEvent {code,modifiers,..}) = event {
            println!("{code} {modifiers}");
            match *code {
                Char('q') => {
                    if modifiers == &KeyModifiers::CONTROL {
                        self.should_quit = true;
                    }
                }
                _ =>(),
              }
        }
        
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit{
            Self::clear_screen()?;
            print!("Goodbye");
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.eval(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        
        Ok(())
    }
}

        // for b in io::stdin().bytes() {
        //     match b {
        //         Ok(b) => {
        //             let c = b as char;
        //             match read() {
        //                 Ok(Key(event)) => {
        //                     match event.code {
        //                         Char(c) => {
        //                             println!("Binary: {0:08b} ASCII: {0:#03} \r",b);

        //                             if c == 'q' {
        //                                 break;
        //                             }
        //                         },
        //                         _ => (),
        //                     }
        //                 },
        //                 Err(err) => println!("Error: {err}"),
        //                 _ => ()
        //             }
                    
                    // if c.is_control() {
                    //     println!("Binary: {0:08b} ASCII: {0:#03} \r",b);
                    // } else {
                    // }
                    // if c == 'q' {
                    //     break;
                    // }
//                 }
//                 Err(err) => println!("Error: {err}"),
//             }
//         }

//     }
// }