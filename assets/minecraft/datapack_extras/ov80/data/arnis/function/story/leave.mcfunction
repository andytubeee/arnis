# Their part told, the person at a stop heads for the nearest footway and walks on like any
# pedestrian; with no footway in this grid cell or those around it they go, as pedestrians
# do out of sight.
tag @s remove arnis.story_npc
tag @s remove arnis.story_here
tag @s remove arnis.npc_on
data merge entity @s {Glowing:0b,CustomNameVisible:0b}
scoreboard players reset @s arnis.story
execute store result score #cx arnis.walk run data get entity @s Pos[0]
execute store result score #cz arnis.walk run data get entity @s Pos[2]
scoreboard players operation #cx arnis.walk /= #spot_cell arnis.walk
scoreboard players operation #cz arnis.walk /= #spot_cell arnis.walk
data modify storage arnis:story spots set value []
scoreboard players remove #cx arnis.walk 1
function arnis:story/leave_row
scoreboard players add #cx arnis.walk 1
function arnis:story/leave_row
scoreboard players add #cx arnis.walk 1
function arnis:story/leave_row
execute unless data storage arnis:story spots[0] run return run function arnis:walk/gone
scoreboard players set #best arnis.story 2147483647
execute store result score #px arnis.story run data get entity @s Pos[0]
execute store result score #pz arnis.story run data get entity @s Pos[2]
function arnis:story/leave_spot
# Walking from here: settle aims them at that point, and the lane takes them on.
execute store result storage arnis:walk spawn.seed int 1 run random value 0..2147483646
data modify storage arnis:walk spawn.p set value 3
tag @s add arnis.walker
function arnis:walk/settle
