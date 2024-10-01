use std::fs::File;
use std::io::BufReader;

use crate::{Args, FileType};

use anyhow::{anyhow, Result};
use idb_rs::IDBParser;

pub fn dump_dirtree_bookmarks_idaplace(args: &Args) -> Result<()> {
    // parse the id0 sector/file
    let id0 = match args.input_type() {
        FileType::Til => return Err(anyhow!("TIL don't contains any ID0 data")),
        FileType::Idb => {
            let input = BufReader::new(File::open(&args.input)?);
            let mut parser = IDBParser::new(input)?;
            let id0_offset = parser
                .id0_section_offset()
                .ok_or_else(|| anyhow!("IDB file don't contains a ID0 sector"))?;
            parser.read_id0_section(id0_offset)?
        }
    };

    let dirtree = id0.dirtree_bookmarks_idaplace()?;
    println!("{:?}", dirtree);

    Ok(())
}
