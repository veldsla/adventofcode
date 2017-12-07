use strict;
use warnings;

my $root = shift or die "Please tell me the root node :-)\n";

my %tree = ();
open (my $in,"<", "input.txt") or die "$!\n";
while(my $line = <$in>) {
	chomp $line;
	my @e = split / -> /, $line;
	
	#program
	$e[0] =~ m/(\w+)\s\((\d+)\)/ or die "Name re error: $e[0]";
	my $name = $1;
	my $weight = $2 / 1;

	#disc?
	my @disc = split /, /, ($e[1] // "");
	$tree{$name} = {weight=>$weight, disc=>\@disc};
}

#recursively calculate the total weight
weight($root);
#and find the unbalanced subtower
printsubtowers($root);

sub weight {
	my $node = shift;
	return $tree{$node}->{total} if exists $tree{$node}->{total};
	my $w = $tree{$node}->{weight};
	$w += weight($_) foreach @{$tree{$node}->{disc}};
	$tree{$node}->{total} = $w;
	return $w;
}

sub printsubtowers {
	my $node = shift;
	my $wdiff = shift;

	my %discsums;
	foreach my $name (@{$tree{$node}->{disc}}) {
		my $w = $tree{$name}->{total};
		push @{$discsums{$w}}, $name;
	}
	print "$node ", $tree{$node}->{weight}, " + (",
		join(", ", map { "($_) x " . scalar(@{$discsums{$_}}) } keys %discsums), ")\n";

	#all subtowers equal?
	if (scalar(keys(%discsums)) == 1) {
		print "I'm off, required weight = ", 
			$tree{$node}->{weight} - $wdiff, "\n";
		exit;
	}
	
	#there are 2 keys in discsums
	my ($wrong, $ok) = sort {scalar(@{$discsums{$a}}) <=> scalar(@{$discsums{$b}}) } keys %discsums;
	printsubtowers($discsums{$wrong}->[0], $wrong - $ok)
}

# vim: ai ts=4 sts=4 et sw=4 ft=perl
