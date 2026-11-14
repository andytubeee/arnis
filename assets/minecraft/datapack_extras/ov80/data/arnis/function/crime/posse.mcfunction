# #hunt officers at one star, two more for each star after it.
scoreboard players operation #cap arnis.cr = @s arnis.wanted
scoreboard players remove #cap arnis.cr 1
scoreboard players operation #cap arnis.cr *= #2 arnis.cr
scoreboard players operation #cap arnis.cr += #hunt arnis.cr
