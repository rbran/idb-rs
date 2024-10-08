use crate::{dump_dirtree::print_dirtree, get_id0_section, Args};

use anyhow::{ensure, Result};
use idb_rs::id0::{ID0Section, Id0Address, Id0AddressKey};

pub fn dump_dirtree_funcs(args: &Args) -> Result<()> {
    // parse the id0 sector/file
    let id0 = get_id0_section(args)?;

    let dirtree = id0.dirtree_function_address()?;
    print_dirtree(|entry| print_function(&id0, *entry).unwrap(), &dirtree);

    Ok(())
}

pub fn print_function(id0: &ID0Section, address: Id0Address) -> Result<()> {
    let infos = id0.address_info_at(address)?;
    let mut name = None;
    let mut ty = None;
    for info in infos {
        match info? {
            idb_rs::id0::AddressInfo::Comment(_) | idb_rs::id0::AddressInfo::Other { .. } => {}
            idb_rs::id0::AddressInfo::Label(label) => {
                if let Some(_old) = name.replace(label) {
                    panic!("Multiple labels can't be return for address")
                }
            }
            idb_rs::id0::AddressInfo::TilType(addr_ty) => {
                ensure!(
                    matches!(&addr_ty, idb_rs::til::Type::Function(_)),
                    "Type for function at {address:#?} is invalid"
                );
                if let Some(_old) = ty.replace(addr_ty) {
                    panic!("Multiple types can't be return for address")
                }
            }
        }
    }
    print!("{:#x}:", address.as_u64());
    match (name, ty) {
        (Some(name), Some(ty)) => print!("\"{name}\":{ty:?}"),
        (None, None) => print!("NO_INFO"),
        (None, Some(ty)) => print!("UNAMED:{ty:?}"),
        (Some(name), None) => print!("\"{name}\""),
    }
    Ok(())
}
