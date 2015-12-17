sub a{my ($v, $m) = @_;sub{$m?($m<0?$_[0]<$v:$_[0]>$v):$_[0]==$v}}
my %m=(chi=>a(3),cat=>a(7,1),sam=>a(2),pom=>a(3,-1),aki=>a(0),viz=>a(0),gol=>a(5,-1),tre=>a(3,1),car=>a(2),per=>a(1));
print grep {
	my @s=(/ (([a-z]{3}).*?: (\d+)(,|$)?)/g); 
	my $c=0;
	map{$m{$s[$_]}($s[$_+1])?$c++:0}(1,5,9);
	$c==3
} <>;
