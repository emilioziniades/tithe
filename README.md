# tithe

A rust CLI application for inputting and viewing expenditure and income data. This probably should have been an Excel spreadsheet but I am a developer and so here we are.

I wanted to get better at tracking my money, and thought this would be a great excuse to write an application to do it for me. This is intended for my personal use, so admittedly it's not fast or pretty.

## Install

```
cargo install --git https://github.com/emilioziniades/tithe
```

## Usage

Add an entry, or generate a summary.

```
tithe add
    --file FILE
    --month MONTH
    --year YEAR
    --group GROUP
    --subgroup SUBGROUP
    --note NOTE
    (+/-)AMOUNT

tithe summary
    --file FILE
    --month MONTH
    --year YEAR
```

`tithe` stores its data in csv format. You may specify the filename, but it defaults to `./tithe.csv`. The csv has the following headers.

```
month, year, group, subgroup, amount, note
```

## TODO

- [ ] Calculate percentages for each group/subgroup.
- [ ] Make summary output prettier.
- [ ] Split out expenditure and income.
