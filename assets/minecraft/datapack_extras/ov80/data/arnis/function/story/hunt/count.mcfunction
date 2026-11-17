# The story's bar while hunting: what's found, then who's back.
execute if score #hunt arnis.story matches 2 if score #gather arnis.story matches 0.. run return run bossbar set arnis:story name [{"text":"Regroup at ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"},{"text":": ","color":"gold"},{"score":{"name":"#here","objective":"arnis.story"},"color":"white"},{"text":" of ","color":"gold"},{"score":{"name":"#all","objective":"arnis.story"},"color":"white"},{"text":" here, ","color":"gold"},{"score":{"name":"#gather","objective":"arnis.story"},"color":"white"},{"text":"s","color":"gold"}]
execute if score #hunt arnis.story matches 2 run return run bossbar set arnis:story name [{"text":"Regroup at ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
scoreboard players operation #got arnis.story = #f0 arnis.story
scoreboard players operation #got arnis.story += #f1 arnis.story
scoreboard players operation #got arnis.story += #f2 arnis.story
scoreboard players operation #got arnis.story += #arts arnis.story
scoreboard players remove #got arnis.story 3
bossbar set arnis:story name [{"text":"Scavenger hunt: ","color":"gold"},{"score":{"name":"#got","objective":"arnis.story"},"color":"yellow"},{"text":" of ","color":"gold"},{"score":{"name":"#arts","objective":"arnis.story"},"color":"yellow"},{"text":" found, then regroup at ","color":"gold"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"}]
