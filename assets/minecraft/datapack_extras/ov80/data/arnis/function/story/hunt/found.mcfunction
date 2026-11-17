# All found: everyone heads back to the person who sent them.
scoreboard players set #hunt arnis.story 2
scoreboard players set #gather arnis.story -1
function arnis:story/hunt/jobs_off
title @a times 10 50 15
title @a subtitle [{"text":"Meet back at ","color":"yellow"},{"storage":"arnis:story","nbt":"cur.name","color":"white"}]
title @a title {"text":"Regroup!","color":"gold","bold":true}
tellraw @a [{"text":"Everything's found! ","color":"green"},{"text":"Head back to ","color":"yellow"},{"storage":"arnis:story","nbt":"cur.name","color":"white"},{"text":". The story goes on once the crew is together.","color":"yellow"}]
execute as @a at @s run playsound minecraft:ui.toast.challenge_complete master @s ~ ~ ~ 0.6 1.3
function arnis:story/bar
