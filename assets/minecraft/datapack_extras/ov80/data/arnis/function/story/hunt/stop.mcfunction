# However the hunt ends, nothing of it stays: no jobs, bars or things.
function arnis:story/hunt/jobs_off
kill @e[type=minecraft:item,tag=arnis.hunt]
clear @a *[minecraft:custom_data~{arnis_hunt:1b}]
scoreboard players set #hunt arnis.story 0
tag @a remove arnis.told
