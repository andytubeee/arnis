execute as @e[type=minecraft:interaction,tag=arnis.npc] at @s run function arnis:npc/follow
execute as @a[scores={arnis.npc=1..}] at @s run function arnis:npc/act
scoreboard players add #npc_t arnis.npc 1
execute if score #npc_t arnis.npc matches 20.. run function arnis:npc/second
