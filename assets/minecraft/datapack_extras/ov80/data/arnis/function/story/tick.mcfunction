scoreboard players enable @a arnis.story.ctl
execute as @a[scores={arnis.story.ctl=1..}] at @s run function arnis:story/control
scoreboard players add #story_t arnis.story 1
execute if score #story_t arnis.story matches 20.. run function arnis:story/second
