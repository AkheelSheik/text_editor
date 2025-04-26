
use super::terminal::{Terminal,Size};
use super::buffer::Buffer;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}


impl View {

    pub fn render(&mut self) -> Result<(),std::io::Error> {
        let Size { height, .. } = Terminal::term_size()?;       
        let vertical = (height/3);
        self.buffer.line.push(String::from("Hello, text!"));

        for i in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.line.get(i as usize) {
                Terminal::write(line)?;
                Terminal::write("\r\n")?;
            }
            
            if i == vertical {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if i.saturating_add(1) < height {
                Terminal::write("\r\n")?;
            }
        }
       Terminal::flush();
       Ok(())
    }

    fn draw_empty_row() -> Result<(),std::io::Error> {
        Terminal::write("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(),std::io::Error> {
        let width = Terminal::term_size()?.width;
        let message = format!("{NAME} -- version {VERSION}");
        let message_length = message.len() as u16;
        let half_message:u16 = message_length/2;
        let main_position_x = width/2 - half_message;
        let first_set_of_space = " ".repeat((main_position_x-1).into());

        Terminal::write(&first_set_of_space)?;
        Terminal::write(&message)?;
        Ok(())
    }

}