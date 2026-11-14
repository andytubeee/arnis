scoreboard players set #npc_t arnis.npc 0
# Only the people near a player can be clicked, so only they carry a click box.
# Pedestrians and story characters alike are people to talk to.
execute as @a[gamemode=!spectator] at @s run tag @e[tag=arnis.walker,tag=!arnis.person,distance=..6] add arnis.person
execute as @e[tag=arnis.person,tag=!arnis.npc_on] at @s if entity @a[gamemode=!spectator,distance=..6] run function arnis:npc/attach
execute as @e[type=minecraft:interaction,tag=arnis.npc] at @s unless entity @a[distance=..10] run function arnis:npc/detach
