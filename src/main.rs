#[macro_use]
extern crate nom;

use nom::{IResult};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).expect("not enough args");
    let mut fd = File::open(filename).expect("unable to open file");
    let mut buf: [u8; 18] = [0; 18];
    fd.read_exact(&mut buf)
        .expect("unable to read enough bytes from file to make an assertion");

    if let IResult::Done(_, e_type) = elf_type(&buf) {
        println!("ELF type: {:?}", e_type);
        match e_type {
            ELFTYPE::Core => std::process::exit(0),
            _ => std::process::exit(1),
        }        
    } else {
        std::process::exit(2)
    }
}

#[derive(Debug)]
enum ELFTYPE {
    Error,
    Relocatable,
    Executable,
    Shared,
    Core,
}

named!(elf_type<ELFTYPE>,
       map!(
           do_parse!(
               tag!("\x7FELF") >>
                   take!(1) >>
                   ei_data: take!(1) >>
                   take!(10) >>
                   e_type: take!(2) >>
                   (ei_data, e_type)),
           |(ei_data, e_type)| {               
               let typenum: u16 = match ei_data[0] {
                   2 => (e_type[0] as u16) << 8 | e_type[1] as u16, // big-endian
                   _ => e_type[0] as u16 | (e_type[1] as u16) << 8,  // little-endian
               };
               
               match typenum {
                   1 => ELFTYPE::Relocatable,
                   2 => ELFTYPE::Executable,
                   3 => ELFTYPE::Shared,
                   4 => ELFTYPE::Core,
                   _ => ELFTYPE::Error,
               }
           }
       )
);
