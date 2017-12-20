#!/usr/bin/env perl 

use strict;
use warnings;
use utf8;

use Data::Dumper;
use List::Util qw/sum/;

my @particles_a;
my @particles_b;
while (<>) {
	m/p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>/ or die "Error in input";
	push @particles_a, {
		p=>[int $1, int $2, int $3], 
		v=>[int $4, int $5, int $6], 
		a=>[int $7, int $8, int $9],
		dist=>abs($1) + abs($2) + abs($3)
	};
	push @particles_b, {
		p=>[int $1, int $2, int $3], 
		v=>[int $4, int $5, int $6], 
		a=>[int $7, int $8, int $9],
		dist=>abs($1) + abs($2) + abs($3)
	};
}

find_closest();
find_closest_with_collisions();

sub find_closest {
	while(1) {
		my $moving_away = 0;
		for my $i (0..$#particles_a) {
			move_particle($particles_a[$i]);
			$moving_away++ if $particles_a[$i]->{moving_away};
		}

		if ($moving_away == scalar(@particles_a)) {
			# the slowest accelerating particle will stay closest to the origin
			# break ties by current distance
			my @a = sort { 
				my ($a_a, $a_p) = acc_dist($particles_a[$a]);
				my ($b_a, $b_p) = acc_dist($particles_a[$b]);
				$a_a <=> $b_a || $a_p <=> $b_p
			} 0..$#particles_a;
			print "20a: $a[0] remains closest\n";
			last;
		}
	}
}

sub find_closest_with_collisions {
	while(1) {
		my %coords = ();
		for my $i (0..$#particles_b) {
			next if $particles_b[$i]->{destroyed};
			move_particle($particles_b[$i]);
			push @{$coords{join(",", @{$particles_b[$i]->{p}})}}, $i;
		}

		# remove collided
		for my $c (keys %coords) {
			if (scalar(@{$coords{$c}}) > 1) {
				$particles_b[$_]->{destroyed} = 1 for @{$coords{$c}};
			}
		}

		# count destroyed and moving away
		my $n = 0;
		for my $i (0..$#particles_b) {	
			++$n && next if $particles_b[$i]->{destroyed};
			if ( $particles_b[$i]->{moving_away} ) {
				# make sure also moving away from all other particles so no
				# more collisions can occur in the future
				my @towards = map {
					$particles_b[$_]->{moving_away} ? move_towards_each_other($particles_b[$i], $particles_b[$_]) : 1
				} $i+1 .. $#particles_b;
				if (@towards && sum(@towards) > 0) {
					last;
				} else {
					$n++;
				}
			}
		}

		if ($n == scalar(@particles_b)) {
			my $remaining = 0;
			($_->{destroyed} || ++$remaining) for @particles_b;
			print "20b: $remaining remain after resolving collisions, \n";
			last;
		}
	}
}

sub move_particle {
	my $p = shift;
	
	my $newdist = 0;
	for (0..2) {
		$p->{v}->[$_] += $p->{a}->[$_];
		$p->{p}->[$_] += $p->{v}->[$_];
		$newdist += abs($p->{p}->[$_]);
	}

	# a particle is moving consistently further if
	# the direction of 'v' is the same as the direction of 'a'
	# and the next dist > previous dist
	my $checkv = 0;
	$checkv += $_ foreach map { $p->{a}->[$_] == 0 || ( $p->{v}->[$_] != 0 && $p->{a}->[$_] *  $p->{v}->[$_] > 0) } 0..2;
	if ($newdist > $p->{dist} && $checkv == 3) {
		$p->{moving_away} = 1;
	}
	$p->{dist} = $newdist;
	
	return $newdist;
}

sub acc_dist {
	my $p = shift;
	my $a = 0;
	$a += abs($p->{a}->[$_]) for 0..2;

	return ($a, $p->{dist});
}

sub move_towards_each_other {
	my $a = shift;
	my $b = shift;

	return 0 if $a->{destroyed} || $b->{destroyed};

	my $dist_p = 0;
	my $dist_a = 0;
	for (0..2) {
		$dist_p += abs($a->{p}->[$_] - $b->{p}->[$_]);
		$dist_a += abs($a->{p}->[$_] + $a->{v}->[$_] + $a->{a}->[$_] - $b->{p}->[$_] - $b->{v}->[$_] - $b->{a}->[$_]);
	}
	return $dist_p > $dist_a;
}

