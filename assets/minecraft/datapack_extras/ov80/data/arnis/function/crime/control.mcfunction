# A choice from the pause menu: difficulty (1-3), police on patrol (4-6), pedestrians (7-9).
execute if score @s arnis.cr.ctl matches 1 run function arnis:crime/difficulty {delay:2,countdown:20,hunt:4}
execute if score @s arnis.cr.ctl matches 2 run function arnis:crime/difficulty {delay:3,countdown:30,hunt:6}
execute if score @s arnis.cr.ctl matches 3 run function arnis:crime/difficulty {delay:5,countdown:45,hunt:8}
execute if score @s arnis.cr.ctl matches 4 run scoreboard players set #calm arnis.cr 2
execute if score @s arnis.cr.ctl matches 5 run scoreboard players set #calm arnis.cr 4
execute if score @s arnis.cr.ctl matches 6 run scoreboard players set #calm arnis.cr 6
execute if score @s arnis.cr.ctl matches 7 run function arnis:crime/crowd {day:20,night:5}
execute if score @s arnis.cr.ctl matches 8 run function arnis:crime/crowd {day:40,night:10}
execute if score @s arnis.cr.ctl matches 9 run function arnis:crime/crowd {day:60,night:15}
tellraw @a [{"selector":"@s","color":"gold"},{"text":" changed the Crime Spree settings.","color":"gray"}]
scoreboard players reset @s arnis.cr.ctl
