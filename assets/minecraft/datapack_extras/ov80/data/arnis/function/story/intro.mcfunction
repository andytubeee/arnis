scoreboard players set @s arnis.story.seen 0
function arnis:story/current
tellraw @s [{"text":"City Stories: ","color":"gold","bold":true},{"text":"one story after another, all over town. The bar at the top says what to do next: talk to the glowing person, solve a riddle by walking to the place, or search for something hidden. The pause menu points the way, and anyone can do a step for everyone.","color":"yellow","bold":false}]
tellraw @s [{"text":"Now playing: ","color":"gold"},{"storage":"arnis:story","nbt":"now.title","color":"white"}]
