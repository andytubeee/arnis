# Half-block steps from an officer's eyes (@s) towards the player, through anything a person
# can be seen through, until it reaches the player or runs past the 16 blocks sight reaches.
execute positioned ~-0.5 ~-0.5 ~-0.5 if entity @a[tag=arnis.cr.me,dx=0,dy=0,dz=0] run return run scoreboard players set #sighted arnis.cr 1
execute unless block ~ ~ ~ #arnis:see_through run return 0
execute positioned ^ ^ ^0.5 if entity @s[distance=..17] run function arnis:crime/ray
