impl CPU{
    fn read_opcode(&self) -> u16{
        self.current.operation
    }

    fn run(&mut self){
        let opcode = self.read.opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d){
            (0x8, _, _, 0x4) => self.add_xy(x,y),
            _ => todo!("opcode {:04x}", opcode),       
        }

        fn add_xy(&mut self, x: u8, y: u8){
            self.registers[x as usize] += self.registers[y as usize];
        }
    }
}
j
    