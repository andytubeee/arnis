$scoreboard players set #f$(k) arnis.story 1
$kill @e[type=minecraft:item,tag=arnis.hunt$(k)]
$tellraw @a [{"selector":"@a[tag=arnis.finder]","color":"gold"},{"text":" found the ","color":"yellow"},{"storage":"arnis:story","nbt":"hunt[$(k)].label","color":"gold"},{"text":"!","color":"yellow"}]
execute as @a[tag=arnis.finder] at @s run playsound minecraft:entity.player.levelup master @s ~ ~ ~ 0.7 1.6
execute as @a[tag=!arnis.finder] at @s run playsound minecraft:block.amethyst_block.chime master @s ~ ~ ~ 1 1.2
tag @a remove arnis.finder
