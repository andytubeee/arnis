# One more wanted star, up to five, and the officers who saw it start the chase.
execute unless score @s arnis.wanted matches 1.. run tellraw @s {"text":"Wanted! Lose the police.","color":"red","bold":true}
execute unless score @s arnis.wanted matches 5.. run scoreboard players add @s arnis.wanted 1
playsound minecraft:block.bell.use player @s
function arnis:crime/sighted
team leave @s
