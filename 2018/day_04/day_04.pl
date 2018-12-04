#!/usr/bin/env perl

use strict;
use warnings;

open (my $in, "<", "input.txt") or die "$!\n";
my @data = map { chomp; $_ } <$in>;
@data = sort @data;

my %parsed;
for (my $i = 0; $i <= $#data; $i++) {
	$data[$i] =~ m/\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] Guard #(\d+) begins shift/ or die;
	my $month = $1;
	my $day = $2;
	my $hour = $3;
	my $minute = $4;
	my $id = $5;
	my @slept = ();
	while ($i < $#data && $data[$i+1] !~ /begins shift/) {
		$i++;
		$data[$i] =~ m/\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] falls asleep/ or die;
		my $go_to_sleep = $4;
		$i++;
		$data[$i] =~ m/\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] wakes up/ or die;
		my $wake_up = $4;
		push @slept, [$go_to_sleep, $wake_up];
	}

	#print join(", ", $id, @slept) ,"\n";
	push @{$parsed{$id}}, \@slept;
}

# find the laziest id

my @lazier = sort { sumsleep($parsed{$b}) <=> sumsleep($parsed{$a}) } keys %parsed;
print "Laziest = $lazier[0]\n";
#get the most slept minute
my %minutemap;
for my $s (@{$parsed{$lazier[0]}}) {
	for my $t (@$s) {
		$minutemap{$_}++ foreach $t->[0] .. $t->[1]-1
	}
}
my $mm = minutemap($parsed{$lazier[0]});
my @msm = sort {$mm->{$b} <=> $mm->{$a} } keys %minutemap;
print "Most slept minute $msm[0]\n";
print "4a: ", $msm[0] * $lazier[0], "\n";

#now make all minutemaps and find highest
my $maxid = 0;
my $maxasleep = 0;
my $maxasleepm = 0;

for my $id (keys %parsed) {
	my $mm = minutemap($parsed{$id});
	for my $m (keys %$mm) {
		if ( $mm->{$m} > $maxasleep) {
			$maxid = $id;
			$maxasleep = $mm->{$m};
			$maxasleepm = $m;
		}
	}
}
print "Most slept minute = $maxasleepm, with $maxasleep minutes by id: $maxid\n";
print "4b: ", $maxid * $maxasleepm, "\n";

sub sumsleep {
	my @times = @{shift(@_)};
	my $slept = 0;
	for my $s (@times) {
		for my $t (@$s) {
			$slept += $t->[1] - $t->[0];
		}
	}
	return $slept;
}

sub minutemap {
	my @times = @{shift(@_)};
	my %minutemap;
	for my $s (@times) {
		for my $t (@$s) {
			$minutemap{$_}++ foreach $t->[0] .. $t->[1]-1
		}
	}
	return \%minutemap;
}

