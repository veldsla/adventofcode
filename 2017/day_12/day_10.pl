use strict;
use warnings;

my %mesh =();
while (my $line = <>) {
	die unless $line =~ m/(\d+) <-> (.*)/;
	my $id = $1;
	my @pipes = split /, /, $2;

	$mesh{$id} = \@pipes;
}

my $seen = connected_from('0');
print "12a: ", scalar(keys %$seen), " programs can be reached from 0\n";

my $groups = 1;
foreach (keys %mesh) {
		++$groups && connected_from($_, $seen) unless exists $seen->{$_}
}
print "12b: There are $groups program groups \n";

sub connected_from {
	my $start = shift;
	my $skip = shift || {};
	my @queue = ($start);
	while ($#queue >= 0) {
		my $id = shift(@queue);
		$skip->{$id} = 1;
		for my $target (@{$mesh{$id}}) {
			push (@queue, $target) unless exists $skip->{$target};

		}
	}
	return $skip;
}


