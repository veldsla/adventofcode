#!/usr/bin/env perl

use strict;
use warnings;

my %ids;
my %h = ();
open (my $in, "<", "input.txt") or die "$!\n";
while (my $line = <$in>) {
	die unless $line =~ m/#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$/;
	my $id = int $1;
	my $x0 = int $2;
	my $x1 = $x0 + $4;
	my $y0 = int $3;
	my $y1 = $y0 + $5;

	$ids{$id} = 0;
	#explode coordinates
	for (my $x = $x0; $x <  $x1; $x++) {
		for (my $y = $y0; $y < $y1; $y++) {
			push @{$h{"$x:$y"}}, $id;
			my @idlist = @{$h{"$x:$y"}};
			if ($#idlist > 0) {
				delete $ids{$_} foreach @idlist;
			}
		}
	}

}
my $overlapping = 0;
$#$_ > 0 && ++$overlapping for values %h;
print "Overlapping inches: $overlapping\n";
print "Ok id(s): ", join(", ", keys %ids), "\n";

