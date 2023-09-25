use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use anyhow::bail;
use clap::{Args, Parser, Subcommand};
use itertools::Itertools;
use time::{Date, Month};

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
    /// Amount spent or received. Can be a negative or positive number.
    #[arg(allow_negative_numbers = true)]
    amount: isize,
    #[arg(short, long)]
    month: Month,
    #[arg(short, long)]
    year: i32,
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
    year: Option<i32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Entry {
    month: time::Month,
    year: i32,
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

    let file_exists = Path::new(&args.file).exists();

    let file = OpenOptions::new()
        .create(true)
        .read(true)
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

fn summary(file: File, summary_args: SummaryArgs) -> anyhow::Result<()> {
    let mut csv_reader = csv::Reader::from_reader(file);
    let entries: Result<Vec<Entry>, csv::Error> = csv_reader.deserialize().collect();
    let entries = entries?;

    let mut entries: Vec<Entry> = match (summary_args.month, summary_args.year) {
        (None, None) => entries,
        (Some(_month), None) => bail!("month is specified, but year is not"),
        (None, Some(year)) => entries
            .into_iter()
            .filter(|entry| entry.year == year)
            .collect(),
        (Some(month), Some(year)) => entries
            .into_iter()
            .filter(|entry| entry.year == year && entry.month == month)
            .collect(),
    };

    entries.sort_by_key(|entry| Date::from_calendar_date(entry.year, entry.month, 1).unwrap());

    let entries = entries
        .into_iter()
        .group_by(|entry| (entry.month, entry.year));

    for ((month, year), entries) in &entries {
        let entries: Vec<Entry> = entries.collect();
        println!("{month} {year}");
        for (group, entries) in &entries.into_iter().group_by(|entry| entry.group.clone()) {
            let entries: Vec<Entry> = entries.collect();
            let group_amount: isize = entries.iter().map(|entry| entry.amount).sum();
            println!("\t{group: <20}{:<20}{group_amount:<20}", "");
            for (subgroup, entries) in &entries.into_iter().group_by(|entry| entry.subgroup.clone())
            {
                let entries: Vec<Entry> = entries.collect();
                let subgroup_amount: isize = entries.iter().map(|entry| entry.amount).sum();
                println!("\t\t{subgroup: <20}{subgroup_amount:<20}");
            }
        }
    }

    Ok(())
}
