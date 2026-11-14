# City Stories needs 1.21.6; the real one lives in the ov80 overlay.
execute as @a unless score @s arnis.story.seen matches 0.. run function arnis:story/unsupported
