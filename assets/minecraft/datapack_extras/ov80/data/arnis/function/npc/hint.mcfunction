# Distance (#m, roughly, in blocks) and compass way (arnis:npc way) from this player to
# arnis:npc where. North is -z.
execute store result score #dx arnis.npc run data get storage arnis:npc where.x
execute store result score #dz arnis.npc run data get storage arnis:npc where.z
execute store result score #px arnis.npc run data get entity @s Pos[0]
execute store result score #pz arnis.npc run data get entity @s Pos[2]
scoreboard players operation #dx arnis.npc -= #px arnis.npc
scoreboard players operation #dz arnis.npc -= #pz arnis.npc
scoreboard players operation #ax arnis.npc = #dx arnis.npc
execute if score #ax arnis.npc matches ..-1 run scoreboard players operation #ax arnis.npc *= #-1 arnis.npc
scoreboard players operation #az arnis.npc = #dz arnis.npc
execute if score #az arnis.npc matches ..-1 run scoreboard players operation #az arnis.npc *= #-1 arnis.npc
# Longer leg plus half the shorter: within 12% of the true distance.
scoreboard players operation #m arnis.npc = #ax arnis.npc
scoreboard players operation #m arnis.npc > #az arnis.npc
scoreboard players operation #h arnis.npc = #ax arnis.npc
scoreboard players operation #h arnis.npc < #az arnis.npc
scoreboard players operation #h arnis.npc /= #2 arnis.npc
scoreboard players operation #m arnis.npc += #h arnis.npc
# A leg more than twice the other is straight that way; otherwise it is a diagonal.
scoreboard players operation #2ax arnis.npc = #ax arnis.npc
scoreboard players operation #2ax arnis.npc *= #2 arnis.npc
scoreboard players operation #2az arnis.npc = #az arnis.npc
scoreboard players operation #2az arnis.npc *= #2 arnis.npc
data modify storage arnis:npc way set value ""
execute if score #az arnis.npc > #2ax arnis.npc if score #dz arnis.npc matches ..-1 run data modify storage arnis:npc way set value "north"
execute if score #az arnis.npc > #2ax arnis.npc if score #dz arnis.npc matches 0.. run data modify storage arnis:npc way set value "south"
execute if score #ax arnis.npc > #2az arnis.npc if score #dx arnis.npc matches 0.. run data modify storage arnis:npc way set value "east"
execute if score #ax arnis.npc > #2az arnis.npc if score #dx arnis.npc matches ..-1 run data modify storage arnis:npc way set value "west"
execute if data storage arnis:npc {way:""} if score #dz arnis.npc matches ..-1 if score #dx arnis.npc matches 0.. run data modify storage arnis:npc way set value "north-east"
execute if data storage arnis:npc {way:""} if score #dz arnis.npc matches ..-1 if score #dx arnis.npc matches ..-1 run data modify storage arnis:npc way set value "north-west"
execute if data storage arnis:npc {way:""} if score #dz arnis.npc matches 0.. if score #dx arnis.npc matches 0.. run data modify storage arnis:npc way set value "south-east"
execute if data storage arnis:npc {way:""} if score #dz arnis.npc matches 0.. if score #dx arnis.npc matches ..-1 run data modify storage arnis:npc way set value "south-west"
