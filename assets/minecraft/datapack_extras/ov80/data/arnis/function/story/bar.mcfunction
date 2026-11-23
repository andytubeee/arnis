execute store result bossbar arnis:story max run scoreboard players get #steps arnis.story
execute store result bossbar arnis:story value run scoreboard players get #step arnis.story
execute if score #step arnis.story >= #steps arnis.story run return run bossbar set arnis:story name [{"text":"Story complete! ","color":"gold"},{"text":"The next one starts in a moment.","color":"yellow"}]
function arnis:story/current
execute if score #step arnis.story = #hunt_step arnis.story if score #hunt arnis.story matches 1.. run return run function arnis:story/hunt/count
# A riddle names its place only after two minutes, for anyone stuck.
execute if data storage arnis:story cur{kind:"riddle"} if score #beat_t arnis.story matches ..119 run return run bossbar set arnis:story name [{"text":"Riddle: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.hint","color":"yellow"}]
execute if data storage arnis:story cur{kind:"riddle"} run return run bossbar set arnis:story name [{"text":"Riddle: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.hint","color":"yellow"},{"text":" (stuck? It's ","color":"gray"},{"storage":"arnis:story","nbt":"cur.name","color":"white"},{"text":")","color":"gray"}]
execute if data storage arnis:story cur{kind:"search"} run return run bossbar set arnis:story name [{"text":"Search near ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"},{"text":" for the ","color":"gold"},{"storage":"arnis:story","nbt":"cur.thing","color":"yellow"},{"text":": ","color":"gold"},{"storage":"arnis:story","nbt":"heat","color":"white"}]
bossbar set arnis:story name [{"text":"Next stop: ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
