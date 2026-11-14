# Half a second crouched without a break beside a pedestrian robs them.
execute if score @s arnis.sneak = @s arnis.sneak.last run scoreboard players set @s arnis.sneak 0
execute if score @s arnis.sneak matches 1.. at @s unless entity @e[tag=arnis.walker,tag=!arnis.robbed,distance=..2.5] run scoreboard players set @s arnis.sneak 0
scoreboard players operation @s arnis.sneak.last = @s arnis.sneak
execute if score @s arnis.sneak matches 10.. at @s unless entity @s[gamemode=spectator] run function arnis:crime/rob
