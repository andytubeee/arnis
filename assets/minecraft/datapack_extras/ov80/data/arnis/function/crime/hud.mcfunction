execute store result score @s arnis.cash run clear @s minecraft:emerald 0
scoreboard players operation @s arnis.cash *= #10 arnis.cr
execute unless score @s arnis.wanted matches 1.. run return run title @s actionbar [{"text":"$","color":"green"},{"score":{"name":"@s","objective":"arnis.cash"},"color":"green"}]
execute if score @s arnis.wanted matches 1 run data modify storage arnis:crime stars set value "★☆☆☆☆"
execute if score @s arnis.wanted matches 2 run data modify storage arnis:crime stars set value "★★☆☆☆"
execute if score @s arnis.wanted matches 3 run data modify storage arnis:crime stars set value "★★★☆☆"
execute if score @s arnis.wanted matches 4 run data modify storage arnis:crime stars set value "★★★★☆"
execute if score @s arnis.wanted matches 5 run data modify storage arnis:crime stars set value "★★★★★"
execute if score @s arnis.unseen matches 0 run return run title @s actionbar [{"text":"$","color":"green"},{"score":{"name":"@s","objective":"arnis.cash"},"color":"green"},{"text":"   ","color":"gold"},{"storage":"arnis:crime","nbt":"stars","color":"gold"},{"text":"  SEEN BY POLICE","color":"red"}]
execute if score @s arnis.unseen <= #delay arnis.cr run return run title @s actionbar [{"text":"$","color":"green"},{"score":{"name":"@s","objective":"arnis.cash"},"color":"green"},{"text":"   ","color":"gold"},{"storage":"arnis:crime","nbt":"stars","color":"gold"},{"text":"  hidden","color":"yellow"}]
title @s actionbar [{"text":"$","color":"green"},{"score":{"name":"@s","objective":"arnis.cash"},"color":"green"},{"text":"   ","color":"gold"},{"storage":"arnis:crime","nbt":"stars","color":"gold"},{"text":"  hidden, cleared in ","color":"yellow"},{"score":{"name":"@s","objective":"arnis.hide"},"color":"white"},{"text":"s","color":"yellow"}]
