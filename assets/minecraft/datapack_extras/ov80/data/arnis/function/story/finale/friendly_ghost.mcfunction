summon minecraft:allay ~ ~1 ~ {Tags:["arnis.story_ghost"],CustomName:{text:"Edith",color:"aqua"},CustomNameVisible:1b,NoAI:1b,Invulnerable:1b,Silent:1b}
particle minecraft:soul ~ ~1.5 ~ 0.4 0.8 0.4 0.02 40
playsound minecraft:block.amethyst_block.chime master @a[distance=..32] ~ ~ ~ 1 0.8
# She fades a moment later.
schedule function arnis:story/finale/ghost_fade 60t
give @a[distance=..32] minecraft:lantern
function arnis:story/finale/end
