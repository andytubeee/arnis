scoreboard players operation #k arnis.npc = @s arnis.wk
execute as @e[tag=arnis.person,tag=arnis.npc_on,distance=..3] if score @s arnis.wk = #k arnis.npc run tag @s remove arnis.npc_on
kill @s
