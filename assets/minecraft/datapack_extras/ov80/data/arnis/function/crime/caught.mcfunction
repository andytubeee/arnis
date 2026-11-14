scoreboard players set @s arnis.cr.die 0
execute if score @s arnis.wanted matches 1.. run tellraw @s {"text":"Busted. Wanted level cleared.","color":"gray"}
scoreboard players set @s arnis.wanted 0
scoreboard players set @s arnis.unseen 0
