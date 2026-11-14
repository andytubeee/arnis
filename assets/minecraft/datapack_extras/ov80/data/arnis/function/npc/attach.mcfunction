# A click box the walker's size, which follows it each tick, and a name the first time.
tag @s add arnis.npc_on
summon minecraft:interaction ~ ~ ~ {Tags:["arnis.npc","arnis.npc_new"],width:0.8f,height:1.9f}
execute store result entity @e[type=minecraft:interaction,tag=arnis.npc_new,limit=1,sort=nearest] height float 0.019 run attribute @s minecraft:scale get 100
scoreboard players operation @e[type=minecraft:interaction,tag=arnis.npc_new,limit=1,sort=nearest] arnis.wk = @s arnis.wk
tag @e[type=minecraft:interaction,tag=arnis.npc_new] remove arnis.npc_new
execute unless data entity @s CustomName run function arnis:npc/name
