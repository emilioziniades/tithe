#!/usr/bin/env bash
# this utility script expects input in the following form
# Group Subgroup Amount
# e.g.
# Car Petrol 500

set -e

echo "reading input from $1"
echo "all entries will be for month $2 and year $3"

month=$2
year=$3

cat $1 | while read -r line; do
	words=($line)
	group=${words[0]}
	subgroup=${words[1]}
	amount=${words[2]}
	tithe add --year $year --month $month --group $group --subgroup $subgroup "$amount"
done
