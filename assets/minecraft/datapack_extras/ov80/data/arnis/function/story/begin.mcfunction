# A new story: its title for everyone, then where it starts.
function arnis:story/current
title @a times 10 60 20
title @a subtitle {"storage":"arnis:story","nbt":"now.title","color":"yellow"}
title @a title {"text":"A new story","color":"gold","bold":true}
tellraw @a [{"text":"New story: ","color":"gold","bold":true},{"storage":"arnis:story","nbt":"now.title","color":"yellow","bold":false}]
execute unless data storage arnis:story now{ep:0} run tellraw @a [{"text":"Episode ","color":"gray"},{"storage":"arnis:story","nbt":"now.ep","color":"gray"},{"text":" of a series. Familiar faces ahead.","color":"gray"}]
function arnis:story/next_stop
function arnis:story/bar
execute as @a at @s run playsound minecraft:block.note_block.bell master @s ~ ~ ~ 0.8 1.2
