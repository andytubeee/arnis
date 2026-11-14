# One place off the list: kept in arnis:npc where if closer than the best so far.
execute store result score #dx arnis.npc run data get storage arnis:npc todo[0].x
execute store result score #dz arnis.npc run data get storage arnis:npc todo[0].z
scoreboard players operation #dx arnis.npc -= #px arnis.npc
scoreboard players operation #dz arnis.npc -= #pz arnis.npc
scoreboard players operation #dx arnis.npc *= #dx arnis.npc
scoreboard players operation #dz arnis.npc *= #dz arnis.npc
scoreboard players operation #dx arnis.npc += #dz arnis.npc
execute if score #dx arnis.npc < #best arnis.npc run data modify storage arnis:npc where set from storage arnis:npc todo[0]
execute if score #dx arnis.npc < #best arnis.npc run scoreboard players operation #best arnis.npc = #dx arnis.npc
data remove storage arnis:npc todo[0]
execute if data storage arnis:npc todo[0] run function arnis:npc/nearest
