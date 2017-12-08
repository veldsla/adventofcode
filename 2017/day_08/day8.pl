#!/usr/bin/env perl 

use strict;
use warnings;
use utf8;

my %registers = (); 
my $max = 0;
while (my $line = <>) {
	chomp $line;
	$line =~ m/(\w+) (inc|dec) (-?\d+) if (\w+ ([!=><]+) -?\d+)/ or die "Error parsing line: $line\n";
	$registers{$1} += ($2 eq "inc" ? $3: -1 * $3) if testguard($4);
	$max = $registers{$1} if exists $registers{$1} && $registers{$1} > $max;
}

my @skeys = sort {$registers{$b} <=> $registers{$a}} keys %registers;
print "Largest value is $registers{$skeys[0]} in register $skeys[0], maximum value encountered $max\n";

sub testguard {
	my $fun = shift;
	$fun =~ s/^(\w+)/(\$registers{$1} \/\/ 0)/;
	return eval $fun
}
