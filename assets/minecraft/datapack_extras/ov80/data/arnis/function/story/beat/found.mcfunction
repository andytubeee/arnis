# A riddle's place reached or a search's thing found: what it reveals, and on.
execute if data storage arnis:story cur{kind:"search"} run tellraw @a [{"text":"Found the ","color":"green"},{"storage":"arnis:story","nbt":"cur.thing","color":"gold"},{"text":"!","color":"green"}]
execute if data storage arnis:story cur{kind:"riddle"} run tellraw @a [{"text":"Riddle solved: ","color":"green"},{"storage":"arnis:story","nbt":"cur.name","color":"yellow"},{"text":"!","color":"green"}]
tellraw @a {"storage":"arnis:story","nbt":"cur.after","color":"white"}
particle minecraft:happy_villager ~ ~1 ~ 1 1 1 0 30
playsound minecraft:entity.player.levelup master @a[distance=..32] ~ ~ ~ 0.8 1.2
function arnis:story/advance
