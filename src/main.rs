use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = "tithe.csv", global = true)]
    file: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new expense or income
    Add(AddArgs),
    /// Show a summary of expenses and incomes
    Summary(SummaryArgs),
}

#[derive(Args, Debug)]
struct AddArgs {
    amount: isize,
    #[arg(short, long)]
    month: time::Month,
    #[arg(short, long)]
    year: usize,
    #[arg(short, long)]
    group: String,
    #[arg(short, long)]
    subgroup: String,
    #[arg(short, long, default_value = "")]
    note: String,
}

#[derive(Args, Debug)]
struct SummaryArgs {
    #[arg(short, long)]
    month: Option<time::Month>,
    #[arg(short, long)]
    year: Option<usize>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Entry {
    month: time::Month,
    year: usize,
    group: String,
    subgroup: String,
    amount: isize,
    note: String,
}

impl From<AddArgs> for Entry {
    fn from(add_args: AddArgs) -> Self {
        Self {
            month: add_args.month,
            year: add_args.year,
            group: add_args.group,
            subgroup: add_args.subgroup,
            amount: add_args.amount,
            note: add_args.note,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    println!("{args:?}");

    let file_exists = Path::new(&args.file).exists();

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(&args.file)?;

    match args.command {
        Commands::Add(add_args) => add(file, add_args, file_exists)?,
        Commands::Summary(summary_args) => summary(file, summary_args)?,
    }
    Ok(())
}

fn add(file: File, add_args: AddArgs, file_exists: bool) -> anyhow::Result<()> {
    let entry: Entry = add_args.into();

    let mut csv_writer = csv::WriterBuilder::new()
        // assumes that if the file exists, it already has headers
        .has_headers(!file_exists)
        .from_writer(file);

    csv_writer.serialize(&entry)?;
    csv_writer.flush()?;

    Ok(())
}

fn summary(file: File, _summary_args: SummaryArgs) -> anyhow::Result<()> {
    let mut csv_reader = csv::Reader::from_reader(file);
    let entries: Result<Vec<Entry>, csv::Error> = csv_reader.deserialize().collect();
    let entries = entries?;
    println!("{entries:#?}");

    Ok(())
}
