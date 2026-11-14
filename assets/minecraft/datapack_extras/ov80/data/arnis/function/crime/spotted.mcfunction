execute unless score @s arnis.wanted matches 1 run tellraw @s {"text":"The police saw that. Lose them!","color":"red","bold":true}
execute unless score @s arnis.wanted matches 1 run playsound minecraft:block.bell.use player @s
scoreboard players set @s arnis.wanted 1
function arnis:crime/sighted
team leave @s
