use std::path::PathBuf;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    pub enum Column {
        Device,
        Start,
        End,
        Sectors,
        Size,
        Type,
        GUID,
        Attributes,
        Name,
    }
}

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    /// display partitions and exit
    #[structopt(short = "l", long = "list")]
    pub print: bool,

    /// output columns
    #[structopt(
        short = "o",
        long = "output",
        raw(use_delimiter = "true", possible_values = "&Column::variants()")
    )]
    pub columns: Vec<Column>,

    /// device to open
    #[structopt(name = "DEVICE", parse(from_os_str))]
    pub device: PathBuf,
}
