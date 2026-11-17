# A player keeps their job until its thing is found, then takes the one fewest are after,
# so friends split up and someone alone finds them all in turn.
execute if score @s arnis.story.job matches 0 if score #f0 arnis.story matches 0 run return 0
execute if score @s arnis.story.job matches 1 if score #f1 arnis.story matches 0 run return 0
execute if score @s arnis.story.job matches 2 if score #f2 arnis.story matches 0 run return 0
execute store result score #c0 arnis.story if entity @a[scores={arnis.story.job=0}]
execute store result score #c1 arnis.story if entity @a[scores={arnis.story.job=1}]
execute store result score #c2 arnis.story if entity @a[scores={arnis.story.job=2}]
execute if score #f0 arnis.story matches 1 run scoreboard players set #c0 arnis.story 1000
execute if score #f1 arnis.story matches 1 run scoreboard players set #c1 arnis.story 1000
execute if score #f2 arnis.story matches 1 run scoreboard players set #c2 arnis.story 1000
execute if score #c0 arnis.story matches 1000 if score #c1 arnis.story matches 1000 if score #c2 arnis.story matches 1000 run return 0
scoreboard players set @s arnis.story.job 0
scoreboard players operation #min arnis.story = #c0 arnis.story
execute if score #c1 arnis.story < #min arnis.story run scoreboard players set @s arnis.story.job 1
execute if score #c1 arnis.story < #min arnis.story run scoreboard players operation #min arnis.story = #c1 arnis.story
execute if score #c2 arnis.story < #min arnis.story run scoreboard players set @s arnis.story.job 2
execute store result storage arnis:story j.k int 1 run scoreboard players get @s arnis.story.job
function arnis:story/hunt/tell with storage arnis:story j
