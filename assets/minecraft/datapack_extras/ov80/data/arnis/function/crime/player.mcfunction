execute if score @s arnis.cr.die matches 1.. run function arnis:crime/caught
execute unless score @s arnis.wanted matches 1.. unless entity @s[team=arnis.law] run team join arnis.law @s
# The rest runs only for a player dealing damage, so most ticks cost four lines.
execute if score @s arnis.cr.hit matches 1.. at @s run function arnis:crime/hit
execute if score @s arnis.cr.kill matches 1.. run function arnis:crime/cop_killed
