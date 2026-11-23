# This player just hurt an officer, by hand or by arrow: two stars at least.
advancement revoke @s only arnis:crime/hit
execute unless score @s arnis.wanted matches 1.. run function arnis:crime/spotted
execute unless score @s arnis.wanted matches 2.. run function arnis:crime/spotted
function arnis:crime/sighted
