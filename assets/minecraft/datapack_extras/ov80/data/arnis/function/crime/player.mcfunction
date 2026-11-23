execute if score @s arnis.cr.die matches 1.. run function arnis:crime/caught
execute unless score @s arnis.wanted matches 1.. unless entity @s[team=arnis.law] run team join arnis.law @s
