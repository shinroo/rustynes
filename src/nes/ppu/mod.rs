mod tile;
mod sprite;
mod background;

use self::super::ram::Ram;
// use std::cell::Cell;
use self::sprite::*;
// use self::tile::Tile;
use self::background::*;
// use self::tile::{Tile, TileParams};

// #[derive(Debug)]
// pub struct Context {
// 
// }

#[derive(Debug)]
pub struct PpuConfig {
    pub is_horizontal_mirror: bool,
}

const CYCLES_PER_LINE: usize = 341;

#[derive(Debug)]
pub struct Ppu {
    cycle: usize,
    line: usize,
    vram: Box<Ram>,
    cram: Box<Ram>,
    background: Background,
    config: PpuConfig,
}

pub struct RenderingContext {}

impl Ppu {
    pub fn new(character_ram: Vec<u8>, config: PpuConfig) -> Ppu {
        println!("{:?}", character_ram);
        Ppu {
            cycle: 0,
            line: 0,
            vram: Box::new(Ram::new(vec![0; 0x2000])),
            cram: Box::new(Ram::new(character_ram)),
            background: Background::new(),
            config,
        }
    }

    // The PPU draws one line at 341 clocks and prepares for the next line.
    // While drawing the BG and sprite at the first 256 clocks,
    // it searches for sprites to be drawn on the next scan line.
    // Get the pattern of the sprite searched with the remaining clock.
    pub fn run(&mut self, cycle: usize) -> Option<RenderingContext> {
        let mut cycle = self.cycle + cycle;
        let line = self.line;
        if line == 0 {
            self.background.clear();
            // buildSprites();
        }
        if cycle < CYCLES_PER_LINE {
            self.cycle = cycle;
            return None;
        }
        self.cycle = cycle - CYCLES_PER_LINE;
        self.line = line + 1;

        // if self.hasSpriteHit() {
        //     self.setSpriteHit();
        // }

        if line <= 240 && line % 8 == 0
        /* && self.scrollY <= 240 */
        {
            let mut config = SpriteConfig {
                offset_addr_by_name_table: 0, //TODO: (~~(tileX / 32) % 2) + tableIdOffset;
                offset_addr_by_background_table: 0, // TODO: (registers[0] & 0x10) ? 0x1000 : 0x0000;
                is_horizontal_mirror: self.config.is_horizontal_mirror,
            };
            let tile_y = (line / 8) as u8; // TODO: + scroll_y;
            let scroll_x = 0;
            self.background
                .build_line(&self.vram, &self.cram, tile_y, scroll_x, &mut config);
        }

        if line == 241 {
            // self.setVblank();
            // if (this.hasVblankIrqEnabled) {
            //   this.interrupts.assertNmi();
            // }
        }

        if line == 262 {
            // this.clearVblank();
            // this.clearSpriteHit();
            // this.line = 0;
            // this.interrupts.deassertNmi();
            println!("{:?}", self.background.field);
            return Some(RenderingContext {});
            //   background: this.isBackgroundEnable ? this.background : null,
            //   sprites: this.isSpriteEnable ? this.sprites : null,
            //   palette: this.getPalette(),
            // };
        }
        None
    }

    // fn get_scroll_tile_y(&self) -> u8 {
    //     // self.registers.scroll_y + ((self.registers.name_table_id / 2) * 240)) / 8);
    //     0
    // }
    // 
    // fn get_tile_y(&self) -> u8 {
    //     (self.line / 8) as u8 + self.get_scroll_tile_y()
    // }
}
