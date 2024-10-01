mod dump_til;
use dump_til::dump_til;
mod dump_id0;
use dump_id0::dump_id0;
//mod split_idb;
//use split_idb::split_idb;
//mod decompress_til;
//use decompress_til::decompress_til;
mod dump_functions;
use dump_functions::dump_functions;
mod dump_segments;
use dump_segments::dump_segments;
mod dump_loader_name;
use dump_loader_name::dump_loader_name;
mod dump_root_info;
use dump_root_info::dump_root_info;
mod dump_addr_info;
use dump_addr_info::dump_addr_info;
mod dump_dirtree_types;
use dump_dirtree_types::dump_dirtree_types;
mod dump_dirtree_structs;
use dump_dirtree_structs::dump_dirtree_structs;
mod dump_dirtree_enums;
use dump_dirtree_enums::dump_dirtree_enums;
mod dump_dirtree_funcs;
use dump_dirtree_funcs::dump_dirtree_funcs;
mod dump_dirtree_names;
use dump_dirtree_names::dump_dirtree_names;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

/// Parse IDA files and output it's data
#[derive(Clone, Debug, Parser)]
struct Args {
    /// input filename to parse
    #[arg(short, long)]
    input: PathBuf,
    /// parse the filename using this format, if not specified use the input file ext, otherwise default to idb 32bits
    #[arg(short, long, value_enum)]
    force_type: Option<FileType>,
    // operation to execute
    #[command(subcommand)]
    operation: Operation,
}

/// File type to parse
#[derive(Clone, Copy, Debug, ValueEnum)]
enum FileType {
    /// IDB file
    Idb,
    // TODO verify if is necessary to parse standalone id0 files
    ///// ID0 database file
    //ID0,
    /// TIL lib types file
    Til,
}

/// File type to parse
#[derive(Clone, Debug, Subcommand)]
enum Operation {
    /// Dump all the TIL type
    DumpTil,
    /// Dump all entries of the ID0 database
    DumpID0,
    //SplitIDB(SplitIDBArgs),
    //DecompressTil(DecompressTilArgs),
    /// Dump all the function information
    DumpFunctions,
    /// Dump all the segments
    DumpSegments,
    /// Dump the loader names
    DumpLoaderNames,
    /// Dump the RootInfo
    DumpRootInfo,
    /// Dump all the address info
    DumpAddressInfo,
    /// Dump all the type from the diretory tree
    DumpDirtreeTypes,
    DumpDirtreeStructs,
    DumpDirtreeEnums,
    DumpDirtreeFuncs,
    DumpDirtreeNames,
}

///// Split the IDB file into it's decompressed sectors. Allow IDB and I64 files.
//#[derive(Clone, Debug, Parser)]
//struct SplitIDBArgs {
//    /// output path, defaults to the input file path
//    output_path: Option<PathBuf>,
//    /// output filename, defatuls to the input filename (without the extension)
//    output_filename: Option<OsString>,
//}

///// Decompress the TIL into a uncompressed version of the TIL. Allow IDB, I64 and TIL files.
//#[derive(Clone, Debug, Parser)]
//struct DecompressTilArgs {
//    /// output filename
//    output: PathBuf,
//}

impl Args {
    pub fn input_type(&self) -> FileType {
        if let Some(input_type) = self.force_type {
            return input_type;
        }
        match self.input.extension().and_then(std::ffi::OsStr::to_str) {
            Some("idb") | Some("i64") => FileType::Idb,
            Some("til") => FileType::Til,
            //Some("id0") => FileType::ID0,
            _ => FileType::Idb,
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.operation {
        Operation::DumpTil => dump_til(&args),
        Operation::DumpID0 => dump_id0(&args),
        //Operation::SplitIDB(split_idbargs) => split_idb(&args, split_idbargs),
        //Operation::DecompressTil(decompress_til_args) => decompress_til(&args, decompress_til_args),
        Operation::DumpFunctions => dump_functions(&args),
        Operation::DumpSegments => dump_segments(&args),
        Operation::DumpLoaderNames => dump_loader_name(&args),
        Operation::DumpRootInfo => dump_root_info(&args),
        Operation::DumpAddressInfo => dump_addr_info(&args),
        Operation::DumpDirtreeTypes => dump_dirtree_types(&args),
        Operation::DumpDirtreeStructs => dump_dirtree_structs(&args),
        Operation::DumpDirtreeEnums => dump_dirtree_enums(&args),
        Operation::DumpDirtreeFuncs => dump_dirtree_funcs(&args),
        Operation::DumpDirtreeNames => dump_dirtree_names(&args),
    }
}
