
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType,size};
use crossterm::{queue,style::Print, Command};
use crossterm::cursor::{MoveTo,Show,Hide};
use std::io::stdout;


use std::io::{Write};
use core::fmt::Display;


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
        Self::queue_command(Hide)?;
        Ok(())

    }

    pub fn show_cursor() -> Result<(),std::io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn write<T: Display>(string: T) -> Result<(),std::io::Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_to_cursor(position: Position) -> Result<(), std::io::Error> {       
        Self::queue_command(MoveTo(position.x,position.y))?;
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
    fn queue_command<T:Command>(command: T) -> Result<(), std::io::Error> {
        queue!(stdout(),command)?;
        Ok(())
    } 

}
    

