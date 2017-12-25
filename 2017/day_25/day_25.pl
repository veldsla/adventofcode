#!/usr/bin/env perl

use strict;
use warnings;
use utf8;

my $line = <>;
$line =~ m/Begin in state (\w)./ or die "parse error";
my $state = $1;
$line = <>;
$line =~ m/Perform a diagnostic checksum after (\d+) steps./ or die "parse error";
my $diag_at = int $1;

my $states = read_machine();
my $counter = 0;
my $tape = {tape=>[0], position=>0};
while ($counter < $diag_at) {
	my $current = read_tape($tape);
	my $action = $states->{$state}->{$current};
	write_tape($tape, $action->{write});
	move_tape($tape, $action->{move});
	$state = $action->{next_state};
	$counter++;
}
print "25: Checksum after $counter operations is ",checksum_tape($tape) ,"\n";


sub read_machine {
	my %states = ();
	while($line = <> ) {
		if ($line =~ m/In state (\w):/) {
			my $name = $1;
			for (0..1) {
				$line = <>;
				$line =~ /If the current value is (\d):/ or die "Parse error";
				my $if_val = int $1;
				$line = <>;
				$line =~ /- Write the value (\d)./ or die "Parse error";
				my $write_val = int $1;
				$line = <>;
				$line =~ /- Move one slot to the (right|left)./ or die "Parse error";
				my $direction = $1;
				$line = <>;
				$line =~ /- Continue with state (\w)./ or die "Parse error";
				my $next_state = $1;
				$states{$name}->{$if_val} = { write=>$write_val, move=>$direction, next_state=>$next_state };
			}
		}

	}
	return \%states;
}

sub move_tape {
	my $tape = shift;
	my $direction = shift;

	if ($tape->{position} == 0) {
		$tape->{tape} = [(0)x100, @{$tape->{tape}}];
		$tape->{position} = 100;
	}

	if ($tape->{position} == $#{$tape->{tape}}) {
		$tape->{tape} = [@{$tape->{tape}}, (0)x100];
	}

	if ($direction eq "right") {
		$tape->{position}++;
	} else {
		$tape->{position}--;
	}
}

sub read_tape {
	my $tape = shift;
	return $tape->{tape}->[$tape->{position}];
}

sub write_tape {
	my $tape = shift;
	my $value = shift;
	$tape->{tape}->[$tape->{position}] = $value;
}

sub checksum_tape {
	my $tape = shift;
	my $count = 0;
	($_ && ++$count) for @{$tape->{tape}}; 
	return $count;
}
