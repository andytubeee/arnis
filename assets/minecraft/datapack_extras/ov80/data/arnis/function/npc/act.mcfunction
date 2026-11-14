# A menu button was pressed: it acts on the walker the menu was opened on, if still close.
scoreboard players operation #k arnis.npc = @s arnis.npc.t
execute as @e[tag=arnis.person,distance=..4] if score @s arnis.wk = #k arnis.npc run tag @s add arnis.npc_it
execute unless score @s arnis.npc matches 7 unless entity @e[tag=arnis.npc_it,distance=..4] run function arnis:npc/left
execute if entity @e[tag=arnis.npc_it,distance=..4] run function arnis:npc/do
tag @e[tag=arnis.npc_it] remove arnis.npc_it
scoreboard players reset @s arnis.npc
