# A step's button: the hunt step sends everyone off first, any other moves the story on.
execute if score #step arnis.story = #hunt_step arnis.story if score #hunt arnis.story matches 0 run return run function arnis:story/hunt/start
execute if score #step arnis.story = #hunt_step arnis.story run return run function arnis:story/hunt/status
function arnis:story/advance
