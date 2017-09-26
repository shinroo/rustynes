#![feature(box_syntax)]

mod parser;
mod rom;
mod ram;
mod bus;
mod cpu;
mod ppu;

use self::cpu::Cpu;
use self::ppu::Ppu;
use self::rom::Rom;
use self::ram::Ram;
use self::bus::cpu_bus::CpuBus;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    program_rom: Rom,
    work_ram: Ram,
    character_ram: Ram,
}

impl Nes {
    pub fn new(buf: &mut [u8]) -> Nes {
        let rom = Box::new(parser::parse(buf).program_rom);
        let cram = Box::new(parser::parse(buf).character_ram);
        Nes {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            program_rom: Rom::new(rom),
            work_ram: Ram::new(Box::new(vec![0; 0x0800])),
            character_ram: Ram::new(cram),
        }
    }

    pub fn reset(&mut self) {
        // TODO: let mut cpu_bus = self.create_bus();
        let mut cpu_bus = CpuBus::new(
            &self.program_rom,
            &mut self.character_ram,
            &mut self.work_ram,
            &mut self.ppu,
        );
        self.cpu.reset(&mut cpu_bus);
    }

    pub fn run(&mut self) {
        let mut cycle = 0;
        let mut cpu_bus = CpuBus::new(
            &self.program_rom,
            &mut self.character_ram,
            &mut self.work_ram,
            &mut self.ppu,
        );
        loop {
            println!("aa");
            cycle += self.cpu.run(&mut cpu_bus);
            println!("{}", cycle);
            if cycle > 300 {
                println!("{}", cycle);
                break;
            }
        }
    }

    fn create_bus(&mut self) -> CpuBus {
        CpuBus::new(
            &self.program_rom,
            &mut self.character_ram,
            &mut self.work_ram,
            &mut self.ppu,
        )
    }
}
