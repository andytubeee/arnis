package dev.arnis.map;

import java.util.List;
import journeymap.api.v2.client.IClientAPI;
import journeymap.api.v2.client.IClientPlugin;
import journeymap.api.v2.client.display.DisplayType;
import journeymap.api.v2.client.display.PolygonOverlay;
import journeymap.api.v2.client.model.MapPolygon;
import journeymap.api.v2.client.model.ShapeProperties;
import journeymap.api.v2.common.event.FullscreenEventRegistry;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;

/** Loaded only by JourneyMap: "Route here" on its map and waypoints, and the route drawn on both maps. */
public final class JourneyMapRoutes implements IClientPlugin {
	private static final String MOD = "arnis_map";
	private static IClientAPI api;

	@Override
	public String getModId() {
		return MOD;
	}

	@Override
	public void initialize(IClientAPI jm) {
		api = jm;
		FullscreenEventRegistry.FULLSCREEN_POPUP_MENU_EVENT.subscribe(MOD, e ->
			e.getPopupMenu().addMenuItem("Route here", pos -> Guide.go(pos.getX(), pos.getZ(), "the marked spot")));
		FullscreenEventRegistry.WAYPOINT_POPUP_MENU_EVENT.subscribe(MOD, e -> {
			var w = e.getWaypoint();
			e.getPopupMenu().addMenuItem("Route here", pos -> Guide.go(w.getX(), w.getZ(), w.getName()));
		});
		Guide.onChange = JourneyMapRoutes::draw;
	}

	/** The route as a 3-block-wide purple band, one quad per leg. */
	private static void draw() {
		api.removeAll(MOD, DisplayType.Polygon);
		if (Guide.route == null) return;
		var look = new ShapeProperties().setFillColor(0xB040FF).setFillOpacity(0.85f).setStrokeOpacity(0);
		List<int[]> pts = Guide.route.points();
		for (int i = 0; i + 1 < pts.size(); i++) {
			int[] a = pts.get(i), b = pts.get(i + 1);
			double dx = b[0] - a[0], dz = b[1] - a[1], len = Math.hypot(dx, dz);
			if (len < 1) continue;
			int nx = (int) Math.round(-dz / len * 1.5), nz = (int) Math.round(dx / len * 1.5);
			var quad = new MapPolygon(new BlockPos(a[0] + nx, 64, a[1] + nz), new BlockPos(b[0] + nx, 64, b[1] + nz),
				new BlockPos(b[0] - nx, 64, b[1] - nz), new BlockPos(a[0] - nx, 64, a[1] - nz));
			try {
				api.show(new PolygonOverlay(MOD, Level.OVERWORLD, look, quad));
			} catch (Exception ignored) {
				// One leg JourneyMap won't draw leaves the rest of the route.
			}
		}
	}
}
