execute if score @s arnis.cr.die matches 1.. run function arnis:crime/caught
execute unless score @s arnis.wanted matches 1 unless entity @s[team=arnis.law] run team join arnis.law @s
# Half a second crouched without a break beside a pedestrian robs them.
execute if score @s arnis.sneak = @s arnis.sneak.last run scoreboard players set @s arnis.sneak 0
execute if score @s arnis.sneak matches 1.. at @s unless entity @e[tag=arnis.walker,tag=!arnis.robbed,distance=..2.5] run scoreboard players set @s arnis.sneak 0
scoreboard players operation @s arnis.sneak.last = @s arnis.sneak
execute if score @s arnis.sneak matches 10.. at @s unless entity @s[gamemode=spectator] run function arnis:crime/rob
execute if score @s arnis.cr.hit matches 1.. at @s run function arnis:crime/hit
scoreboard players set @s arnis.cr.hit 0
