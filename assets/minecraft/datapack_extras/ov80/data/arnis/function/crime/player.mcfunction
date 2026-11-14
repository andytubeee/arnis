execute if score @s arnis.cr.die matches 1.. run function arnis:crime/caught
execute unless score @s arnis.wanted matches 1 unless entity @s[team=arnis.law] run team join arnis.law @s
# The rest runs only for a player crouching or dealing damage, so most ticks cost four lines.
execute if score @s arnis.sneak matches 1.. run function arnis:crime/crouch
execute if score @s arnis.cr.hit matches 1.. at @s run function arnis:crime/hit
