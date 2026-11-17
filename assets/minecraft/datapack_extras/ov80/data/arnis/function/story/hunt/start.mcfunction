# #hunt: 0 not started, 1 everyone is looking, 2 regrouping. #f0..#f2: each thing found.
function arnis:story/current
scoreboard players set #hunt arnis.story 1
scoreboard players reset * arnis.story.job
scoreboard players set #f0 arnis.story 0
scoreboard players set #f1 arnis.story 0
scoreboard players set #f2 arnis.story 0
# Things this map had no place for count as found.
execute if score #arts arnis.story matches ..1 run scoreboard players set #f1 arnis.story 1
execute if score #arts arnis.story matches ..2 run scoreboard players set #f2 arnis.story 1
tellraw @a [{"text":"Scavenger hunt! ","color":"gold","bold":true},{"text":"Everyone gets their own thing to find, shown in the bar at the top. Once it's all found, regroup at ","color":"yellow","bold":false},{"storage":"arnis:story","nbt":"cur.name","color":"white","bold":false},{"text":".","color":"yellow","bold":false}]
function arnis:story/hunt/seek
