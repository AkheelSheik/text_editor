
use super::terminal::{Terminal,Size};
use super::buffer::Buffer;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer
}


impl View {

    pub fn render(&self) -> Result<(),std::io::Error> {
        let Size { height, .. } = Terminal::term_size()?;       
        let vertical = (height/3);

        for current_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.line.get(current_row as usize) {
                // println!("{}",line);
                Terminal::write(line)?;
                Terminal::write("\r\n")?;
                continue;
            } else{
                Self::draw_empty_row()?;
            }
            
            if current_row == vertical {
                if self.buffer.is_empty() == false {
                    Self::draw_welcome_message()?;
                }
            }

            if current_row.saturating_add(1) < height {
                Terminal::write("\r\n")?;
            }
        }
       Terminal::flush();
       Ok(())
    }

    pub fn load(&mut self,file_name: &str) -> Result<(),std::io::Error>{
        let file_contents = std::fs::read_to_string(file_name)?;

        for line in file_contents.lines() {
            self.buffer.line.push(line.to_string());
        }
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