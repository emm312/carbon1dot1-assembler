use std::str::FromStr;

use mcschem::{data_version, utils, Block, BlockEntity, Schematic};

pub fn generate_schem<W: std::io::Write>(
    writer: &mut W,
    content: &[u8],
    bytes_per_page: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let pages = content.len() / bytes_per_page;

    let mut schem = Schematic::new(data_version::MC_1_18_2, 16, pages as u16 * 2, 16);

    for (i, page) in content.chunks_exact(bytes_per_page).enumerate() {
        let z = (pages - i - 1) * 2;
        let mut x = 0;
        let mut y = 0;

        for ir in 0..8 {
            for ih in 0..8 {
                schem.set_block_entity(
                    x,
                    y,
                    z,
                    Block::from_str("minecraft:barrel[facing=north,open=false]").unwrap(),
                    BlockEntity::Barrel {
                        items: utils::barrel_ss(
                            ((page[ir] >> (7 - ih))
                                + ((page[ir + 8] >> (7 - ih)) << 1)
                                + ((page[ir + 16] >> (7 - ih)) << 2)
                                + ((page[ir + 24] >> (7 - ih)) << 3))
                                as usize,
                        ),
                    },
                );
                y += 2;
            }

            y = 0;
            x += 2;
        }
    }

    schem.export(writer)?;

    Ok(())
}
