# The way to where the story goes next, or else to the nearest named place.
execute if score #stories arnis.npc matches 1 if function arnis:npc/next_place run return run function arnis:npc/point
data modify storage arnis:npc todo set from storage arnis:npc places
scoreboard players set #best arnis.npc 2147483647
execute store result score #px arnis.npc run data get entity @s Pos[0]
execute store result score #pz arnis.npc run data get entity @s Pos[2]
execute if data storage arnis:npc todo[0] run function arnis:npc/nearest
execute if score #best arnis.npc matches 2147483647 run return run tellraw @s {"text":"They shrug. They're not from around here.","color":"gray","italic":true}
function arnis:npc/point
