# Keeps up with its walker; one gone (despawned, or robbed and run off) takes its box along.
scoreboard players operation #k arnis.npc = @s arnis.wk
execute as @e[tag=arnis.person,distance=..3] if score @s arnis.wk = #k arnis.npc run tag @s add arnis.npc_follow
execute unless entity @e[tag=arnis.npc_follow,distance=..3] run return run kill @s
tp @s @e[tag=arnis.npc_follow,distance=..3,limit=1]
tag @e[tag=arnis.npc_follow,distance=..3] remove arnis.npc_follow
