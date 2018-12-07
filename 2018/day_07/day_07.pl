#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/sum0 max/;
use List::MoreUtils qw/last_index/;
use Storable qw/dclone/;

my $WORKERS = 5;
my $BASECOST = 60;

my %nodes = ();
open(my$in, "<", "input.txt") or die "$!\n";
while(my $line = <$in>) {
	$line =~ m/Step (.) must be finished before step (.) can begin./ or die;
	my $from = $1;
	my $to = $2;
	push @{$nodes{$from}}, $to;
 	$nodes{$to} = [] unless exists $nodes{$to};
}
# sort the targets of all the nodes
for my $node (keys %nodes) {
	my @sorted = sort @{$nodes{$node}};
	$nodes{$node} = \@sorted;
}

print "7a: Instruction order ", topo_sort(dclone(\%nodes)), "\n";
print "7a: $WORKERS workers take ", worker_time(%nodes), " seconds\n";

sub topo_sort {
	my %graph = %{shift(@_)};
	my @result = ();
	my @queue = roots(\%graph);
	while (my $node = shift(@queue)) {
		push @result, $node;
		my %next = ();
		while (my $to = shift(@{$graph{$node}})) {
			#anyone else comes here, then we can't take it yet
			$next{$to} = 0 unless has_incoming_edge(\%graph, $to);
		}
		#add sort the available nodes
		push @queue, keys %next;
		@queue = sort @queue;
	}
	my $s = join("", @result);
	return $s;
}

sub worker_time {
	my %graph = @_;
	my @queue = roots(\%graph);

	# create the incoming edge list
	my %incoming = map { $_ => [] } keys %graph;
	foreach my $from (keys %graph) {
		foreach my $to (@{$graph{$from}}) {
			push @{$incoming{$to}}, $from, 
		}
	}

	my @workers = map { [] } 1 .. $WORKERS;

	while (my $node = shift(@queue)) {
		# assign to worker
		my $w = first_avail_worker(\@workers);
		# get the time the prereqs are finished
		my $f = time_finished(\@workers, $incoming{$node});
		# pad with waiting time and add time required for node
		my $pad = $f - scalar(@$w);
		push @$w, ('.')x$pad if $pad > 0;
		push @$w, ($node) x ($BASECOST + ord($node) - 64) ;

		my %next = ();
		while (my $to = shift(@{$graph{$node}})) {
			#anyone else comes here, then we can't take it yet
			$next{$to} = 0 unless has_incoming_edge(\%graph, $to);
		}
		#add the sorted the available nodes
		push @queue, sort keys %next;
	}

	my @sums = map { scalar(@{$_}) } @workers;
	return max(@sums);
}

sub roots {
	my $nodes = shift;
	my %roots = map { $_ => 0 } keys %$nodes;
	foreach (keys %nodes) {
		delete $roots{$_} foreach @{$nodes{$_}};
	}
	return (sort keys %roots);
}

sub has_incoming_edge {
	my $tree = shift;
	my $n = shift;
	for my $a (values %$tree) {
		for my $x (@$a) {
			return 1 if $x eq $n;
		}
	}
	return 0;
}

sub first_avail_worker {
	my $w = shift;
	my @order = sort { scalar(@{$w->[$a]}) <=> scalar(@{$w->[$b]}) } 0 .. $#$w;
	return $w->[$order[0]];
}

sub time_finished {
	my $w = shift;
	my $done = shift;

	my $max = -1;
	foreach my $d (@$done) {
		foreach my $worker (@$w) {
			my $t = last_index { $_ eq $d } @$worker;
			$max = $t if $max < $t;
		}
	}
	return $max + 1;
}

