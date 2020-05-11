use nep::cartridge::*;
use nep::cpu::*;
use nep::prelude::*;

#[test]
fn cpu_test() -> Result<()> {
    const FILE_PATH: &str = "./roms/nestest.nes";
    let cartridge = Cartridge::from_file(FILE_PATH)?;

    Ok(())
}
