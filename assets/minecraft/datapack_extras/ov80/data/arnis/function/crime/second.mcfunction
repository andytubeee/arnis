scoreboard players set #cr_t arnis.cr 0
execute as @a unless score @s arnis.cr.seen matches 1.. run function arnis:crime/intro
execute as @a[scores={arnis.wanted=1}] at @s run function arnis:crime/heat
execute as @a[gamemode=!spectator] at @s run function arnis:crime/patrol
execute as @e[type=minecraft:skeleton,tag=arnis.cop] at @s unless entity @a[distance=..80] run function arnis:walk/gone
execute as @a run function arnis:crime/hud
