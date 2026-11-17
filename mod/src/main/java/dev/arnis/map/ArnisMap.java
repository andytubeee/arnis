package dev.arnis.map;

import static net.fabricmc.fabric.api.client.command.v2.ClientCommandManager.argument;
import static net.fabricmc.fabric.api.client.command.v2.ClientCommandManager.literal;

import com.mojang.brigadier.arguments.IntegerArgumentType;
import com.mojang.brigadier.arguments.StringArgumentType;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.minecraft.client.Minecraft;
import net.minecraft.resources.Identifier;

/** /route to a named place or a spot, a trail on the ground, and the next turn under the hotbar. */
public final class ArnisMap implements ClientModInitializer {
	@Override
	public void onInitializeClient() {
		ClientTickEvents.END_CLIENT_TICK.register(Guide::tick);
		// Each world has its own streets.
		ClientPlayConnectionEvents.JOIN.register((h, s, mc) -> Guide.forget());
		ClientPlayConnectionEvents.DISCONNECT.register((h, mc) -> Guide.forget());
		HudElementRegistry.addLast(Identifier.fromNamespaceAndPath("arnis_map", "route"), (g, t) -> {
			String line = Guide.hud();
			if (line == null) return;
			Minecraft mc = Minecraft.getInstance();
			int w = mc.font.width(line);
			int x = (g.guiWidth() - w) / 2, y = g.guiHeight() - 90;
			g.fill(x - 4, y - 3, x + w + 4, y + 11, 0xA0000000);
			g.drawString(mc.font, line, x, y, 0xFFE0B0FF);
		});
		ClientCommandRegistrationCallback.EVENT.register((d, reg) -> d.register(literal("route")
			.then(literal("off").executes(c -> {
				Guide.stop();
				Guide.say("Route cleared.");
				return 1;
			}))
			.then(literal("to")
				.then(argument("x", IntegerArgumentType.integer()).then(argument("z", IntegerArgumentType.integer()).executes(c -> {
					int x = IntegerArgumentType.getInteger(c, "x"), z = IntegerArgumentType.getInteger(c, "z");
					Guide.go(x, z, x + ", " + z);
					return 1;
				}))))
			.then(argument("place", StringArgumentType.greedyString())
				.suggests((c, b) -> {
					Roads roads = Guide.roads();
					if (roads != null) {
						String typed = b.getRemaining().toLowerCase();
						roads.places().stream().map(Roads.Place::name).filter(n -> n.toLowerCase().startsWith(typed)).forEach(b::suggest);
					}
					return b.buildFuture();
				})
				.executes(c -> {
					String name = StringArgumentType.getString(c, "place");
					Roads roads = Guide.roads();
					Roads.Place p = roads == null ? null : roads.place(name);
					if (p == null) Guide.say("No place called " + name + " here. Press Tab for names.");
					else Guide.go(p.x(), p.z(), p.name());
					return 1;
				}))));
	}
}
