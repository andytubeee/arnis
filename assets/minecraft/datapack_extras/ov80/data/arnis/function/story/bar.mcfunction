execute store result bossbar arnis:story max run scoreboard players get #steps arnis.story
execute store result bossbar arnis:story value run scoreboard players get #step arnis.story
execute if score #step arnis.story >= #steps arnis.story run return run bossbar set arnis:story name [{"text":"City Stories: ","color":"gold"},{"text":"the end. Thanks for playing!","color":"yellow"}]
function arnis:story/current
execute if score #step arnis.story = #hunt_step arnis.story if score #hunt arnis.story matches 1.. run return run function arnis:story/hunt/count
bossbar set arnis:story name [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
