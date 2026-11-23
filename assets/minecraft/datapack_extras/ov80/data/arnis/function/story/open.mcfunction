# Loads story #story of the run into what the step and hunt functions read.
execute store result storage arnis:story i.i int 1 run scoreboard players get #story arnis.story
function arnis:story/open_at with storage arnis:story i
data modify storage arnis:story steps set from storage arnis:story now.steps
data modify storage arnis:story hunt set from storage arnis:story now.hunt
data modify storage arnis:story finale set from storage arnis:story now.finale
execute store result score #steps arnis.story run data get storage arnis:story now.steps
execute store result score #hunt_step arnis.story run data get storage arnis:story now.hunt_step
execute store result score #arts arnis.story run data get storage arnis:story now.arts
# The hunt's things are told apart from any other hunt's by its story and step.
scoreboard players operation #hsid arnis.story = #story arnis.story
scoreboard players operation #hsid arnis.story *= #16 arnis.story
scoreboard players operation #hsid arnis.story += #hunt_step arnis.story
execute store result storage arnis:story hunt[0].sid int 1 run scoreboard players get #hsid arnis.story
execute store result storage arnis:story hunt[1].sid int 1 run scoreboard players get #hsid arnis.story
execute store result storage arnis:story hunt[2].sid int 1 run scoreboard players get #hsid arnis.story
