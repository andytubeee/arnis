# The countdown is #countdown at one star and half as long again for each star after it.
scoreboard players set @s arnis.unseen 0
scoreboard players operation @s arnis.hide = @s arnis.wanted
scoreboard players add @s arnis.hide 1
scoreboard players operation @s arnis.hide *= #countdown arnis.cr
scoreboard players operation @s arnis.hide /= #2 arnis.cr
