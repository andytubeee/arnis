# #sighted: 1 when an officer within 16 blocks has a clear line from its eyes to this player's.
scoreboard players set #sighted arnis.cr 0
tag @s add arnis.cr.me
execute as @e[type=minecraft:skeleton,tag=arnis.cop,distance=..16] at @s anchored eyes facing entity @a[tag=arnis.cr.me,limit=1] eyes positioned ^ ^ ^ anchored feet run function arnis:crime/ray
tag @s remove arnis.cr.me
