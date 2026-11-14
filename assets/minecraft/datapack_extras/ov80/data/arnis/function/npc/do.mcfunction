scoreboard players set @e[tag=arnis.npc_it,distance=..4] arnis.wpause 30
execute if score @s arnis.npc matches 1 run function arnis:npc/hi
execute if score @s arnis.npc matches 2 run function arnis:npc/chat
execute if score @s arnis.npc matches 3 run function arnis:npc/directions
execute if score @s arnis.npc matches 4 unless entity @e[tag=arnis.npc_it,tag=arnis.story_npc,distance=..4] run function arnis:npc/rob
execute if score @s arnis.npc matches 5 if entity @e[tag=arnis.npc_it,tag=arnis.story_npc,distance=..4] run function arnis:npc/talk
execute if score @s arnis.npc matches 6 if entity @e[tag=arnis.npc_it,tag=arnis.errand_npc,distance=..4] run function arnis:npc/errand
