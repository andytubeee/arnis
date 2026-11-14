# A player right-clicked someone's click box: the box that remembers this player opens the
# menu. Car and train boxes answer their own advancement.
advancement revoke @s only arnis:npc/click
data modify storage arnis:npc me set from entity @s UUID
tag @s add arnis.npc_me
execute as @e[type=minecraft:interaction,tag=arnis.npc,distance=..8] if data entity @s interaction run function arnis:npc/clicked
tag @s remove arnis.npc_me
