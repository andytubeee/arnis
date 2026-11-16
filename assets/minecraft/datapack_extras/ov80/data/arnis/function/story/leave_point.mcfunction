$execute store result score #dx arnis.story run data get storage arnis:walk seg[$(s)].p[$(a)].x
$execute store result score #dz arnis.story run data get storage arnis:walk seg[$(s)].p[$(a)].z
scoreboard players operation #dx arnis.story -= #px arnis.story
scoreboard players operation #dz arnis.story -= #pz arnis.story
scoreboard players operation #dx arnis.story *= #dx arnis.story
scoreboard players operation #dz arnis.story *= #dz arnis.story
scoreboard players operation #dx arnis.story += #dz arnis.story
execute if score #dx arnis.story >= #best arnis.story run return 0
scoreboard players operation #best arnis.story = #dx arnis.story
$data modify storage arnis:walk spawn.s set value $(s)
$data modify storage arnis:walk spawn.a set value $(a)
