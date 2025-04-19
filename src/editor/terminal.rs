
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType,size};
use crossterm::{queue,style::Print};
use crossterm::cursor::{MoveTo,Show,Hide};
use std::io::stdout;
use std::io::{Write};


#[derive(Copy,Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16
}

#[derive(Copy,Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

pub struct Terminal {}


impl Terminal {

    pub fn default() -> Self {
        Terminal{}
    }

    pub fn initalize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen();
        Self::move_to_cursor(Position {x:0,y:0})?;
        Self::flush()?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(),std::io::Error> {
        queue!(stdout(),Hide)?;
        Ok(())

    }

    pub fn show_cursor() -> Result<(),std::io::Error> {
        queue!(stdout(),Show)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn write(string: &str) -> Result<(),std::io::Error> {
        queue!(stdout(),Print(string))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(),Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(),Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_to_cursor(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(),MoveTo(position.x,position.y))?;
        Ok(())
    }

    pub fn term_size() -> Result<Size, std::io::Error> {
        let (width,height) = size()?;
        Ok(Size {height,width})
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

}
    

