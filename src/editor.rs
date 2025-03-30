use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};


pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor {should_quit: false}
    }
    pub fn run(&mut self) {
        if let Err(err) = self.repl() { 
            panic!("{err:#?}"); 
        }
        print!("Goodbye.\r\n");
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                let Char(c) = event.code else {
                    panic!("Could not get a character");
                };
                let mod_1 = event.modifiers;

                if mod_1 == KeyModifiers::CONTROL && c == 'q' {
                    println!("Hello");
                    self.should_quit = true;
                }
            }

            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
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