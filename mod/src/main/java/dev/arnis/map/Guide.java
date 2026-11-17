package dev.arnis.map;

import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.List;
import net.minecraft.client.Minecraft;
import net.minecraft.core.particles.DustParticleOptions;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.level.levelgen.Heightmap;

/** The route being followed: worked out on the streets, kept up as the player moves. */
final class Guide {
	private static final Identifier ROADS = Identifier.fromNamespaceAndPath("arnis", "roads.json");
	/** Further than this from the route, it is worked out again from where the player is. */
	private static final double OFF_ROUTE = 24;
	private static final double ARRIVED = 10;

	private static Roads roads;
	static Roads.Route route;
	private static int tx, tz, at, tick;
	private static String label;
	/** JourneyMap redraws the route on its maps when it's there. */
	static Runnable onChange = () -> {};

	/** The world's streets, read once from its resource pack; null in a world without them. */
	static Roads roads() {
		if (roads != null) return roads;
		Minecraft mc = Minecraft.getInstance();
		var res = mc.getResourceManager().getResource(ROADS);
		if (res.isEmpty()) return null;
		try (InputStream in = res.get().open()) {
			roads = Roads.parse(new String(in.readAllBytes(), StandardCharsets.UTF_8));
		} catch (Exception e) {
			say("Arnis Map couldn't read this world's streets: " + e.getMessage());
		}
		return roads;
	}

	static void forget() {
		roads = null;
		stop();
	}

	static void go(int x, int z, String name) {
		if (roads() == null) {
			say("This world has no Arnis street map. Generate it with City Stories or Crime Spree on.");
			return;
		}
		tx = x;
		tz = z;
		label = name;
		if (reroute()) say("Route to " + name + ": " + Math.round(route.length()) + " m. Follow the purple trail.");
		else {
			say("No streets lead to " + name + " from here.");
			label = null;
		}
	}

	static void stop() {
		label = null;
		route = null;
		onChange.run();
	}

	private static boolean reroute() {
		var p = Minecraft.getInstance().player;
		if (p == null || roads == null) return false;
		route = roads.route(p.getBlockX(), p.getBlockZ(), tx, tz);
		at = 0;
		onChange.run();
		return route != null;
	}

	static void tick(Minecraft mc) {
		if (label == null || route == null || mc.player == null || mc.level == null) return;
		double px = mc.player.getX(), pz = mc.player.getZ();
		tick++;
		if (tick % 10 == 0) {
			if (Math.hypot(px - tx, pz - tz) < ARRIVED) {
				say("You've arrived at " + label + ".");
				stop();
				return;
			}
			if (follow(px, pz) > OFF_ROUTE && tick % 40 == 0) reroute();
		}
		if (tick % 4 == 0) trail(mc, px, pz);
	}

	/** Moves `at` to the leg nearest the player; returns how far off it they are. */
	private static double follow(double px, double pz) {
		List<int[]> pts = route.points();
		double best = Double.MAX_VALUE;
		for (int i = 0; i + 1 < pts.size(); i++) {
			double d = toSegment(px, pz, pts.get(i), pts.get(i + 1));
			if (d < best) {
				best = d;
				at = i;
			}
		}
		return best;
	}

	/** Purple dust along the next 40 blocks, on the ground. */
	private static void trail(Minecraft mc, double px, double pz) {
		List<int[]> pts = route.points();
		var dust = new DustParticleOptions(0xB040FF, 1.4f);
		double left = 40, x = px, z = pz;
		for (int i = at + 1; i < pts.size() && left > 0; i++) {
			int[] b = pts.get(i);
			double len = Math.hypot(b[0] + 0.5 - x, b[1] + 0.5 - z);
			for (double s = 0; s < len && left > 0; s += 2.5, left -= 2.5) {
				double qx = x + (b[0] + 0.5 - x) * s / len, qz = z + (b[1] + 0.5 - z) * s / len;
				int y = mc.level.getHeight(Heightmap.Types.MOTION_BLOCKING, (int) Math.floor(qx), (int) Math.floor(qz));
				mc.level.addParticle(dust, qx, y + 0.2, qz, 0, 0, 0);
			}
			x = b[0] + 0.5;
			z = b[1] + 0.5;
		}
	}

	/** The line under the hotbar: the next turn and how far there is to go. */
	static String hud() {
		var p = Minecraft.getInstance().player;
		if (label == null || route == null || p == null) return null;
		List<int[]> pts = route.points();
		List<String> legs = route.legs();
		double ahead = Math.hypot(pts.get(Math.min(at + 1, pts.size() - 1))[0] - p.getX(), pts.get(Math.min(at + 1, pts.size() - 1))[1] - p.getZ());
		double total = ahead;
		String next = null;
		for (int k = at + 1; k + 1 < pts.size(); k++) {
			if (next == null && !legs.get(k).equals(legs.get(k - 1)) && !legs.get(k).isEmpty()) {
				String way = Roads.turn(pts.get(k - 1), pts.get(k), pts.get(k + 1));
				next = (way.isEmpty() ? "Continue onto " : "Turn " + way + " onto ") + legs.get(k) + " in " + Math.round(total) + " m";
			}
			total += Math.hypot(pts.get(k + 1)[0] - pts.get(k)[0], pts.get(k + 1)[1] - pts.get(k)[1]);
		}
		String on = legs.get(Math.min(at, legs.size() - 1));
		if (next == null) next = on.isEmpty() ? label : on + " to " + label;
		return next + "  ·  " + Math.round(total) + " m to " + label;
	}

	private static double toSegment(double px, double pz, int[] a, int[] b) {
		double dx = b[0] - a[0], dz = b[1] - a[1], len2 = dx * dx + dz * dz;
		double t = len2 == 0 ? 0 : Math.max(0, Math.min(1, ((px - a[0]) * dx + (pz - a[1]) * dz) / len2));
		return Math.hypot(px - (a[0] + t * dx), pz - (a[1] + t * dz));
	}

	static void say(String text) {
		Minecraft.getInstance().gui.getChat().addMessage(Component.literal("[Map] ").withColor(0xB040FF).append(Component.literal(text).withColor(0xFFFFFF)));
	}
}
