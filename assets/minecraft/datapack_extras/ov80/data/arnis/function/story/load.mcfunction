# The step everyone is on, how many each player has been granted, and the pause page's choice.
scoreboard objectives add arnis.story dummy
scoreboard objectives add arnis.story.seen dummy
scoreboard objectives add arnis.story.ctl trigger
scoreboard players set #story_t arnis.story 0
function arnis:story/plan
execute unless score #step arnis.story matches 0.. run scoreboard players set #step arnis.story 0
execute unless score #errand arnis.story matches 0.. run function arnis:story/errand/reset
bossbar add arnis:story {"text":"City Stories"}
bossbar set arnis:story color yellow
function arnis:story/bar
