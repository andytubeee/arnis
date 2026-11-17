package dev.arnis.map;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;
import java.util.PriorityQueue;

/** The world's streets, as Arnis wrote them into resources.zip, and the shortest way along them. */
public final class Roads {
	public record Place(String name, int x, int z) {}

	/** Points from the street nearest the start to the target, and each leg's street name. */
	public record Route(List<int[]> points, List<String> legs, double length) {
		/** The streets in order, each once per stretch along it. */
		public List<String> streets() {
			List<String> out = new ArrayList<>();
			for (String s : legs) if (!s.isEmpty() && (out.isEmpty() || !out.get(out.size() - 1).equals(s))) out.add(s);
			return out;
		}
	}

	private final int[] xz;
	/** Per node: neighbour, then the way it is reached by, flattened. */
	private final int[][] links;
	private final String[] wayNames;
	private final List<Place> places;

	private Roads(int[] xz, int[][] links, String[] wayNames, List<Place> places) {
		this.xz = xz;
		this.links = links;
		this.wayNames = wayNames;
		this.places = places;
	}

	public static Roads parse(String json) {
		JsonObject o = JsonParser.parseString(json).getAsJsonObject();
		JsonArray nodes = o.getAsJsonArray("nodes");
		int[] xz = new int[nodes.size()];
		for (int i = 0; i < xz.length; i++) xz[i] = nodes.get(i).getAsInt();
		int n = xz.length / 2;
		int[] degree = new int[n];
		JsonArray ways = o.getAsJsonArray("ways");
		String[] wayNames = new String[ways.size()];
		List<int[]> edges = new ArrayList<>();
		for (int w = 0; w < wayNames.length; w++) {
			JsonObject way = ways.get(w).getAsJsonObject();
			wayNames[w] = way.get("n").getAsString();
			JsonArray p = way.getAsJsonArray("p");
			for (int i = 1; i < p.size(); i++) {
				int a = p.get(i - 1).getAsInt(), b = p.get(i).getAsInt();
				edges.add(new int[] {a, b, w});
				degree[a]++;
				degree[b]++;
			}
		}
		int[][] links = new int[n][];
		for (int i = 0; i < n; i++) links[i] = new int[degree[i] * 2];
		int[] fill = new int[n];
		for (int[] e : edges) {
			links[e[0]][fill[e[0]]++] = e[1];
			links[e[0]][fill[e[0]]++] = e[2];
			links[e[1]][fill[e[1]]++] = e[0];
			links[e[1]][fill[e[1]]++] = e[2];
		}
		List<Place> places = new ArrayList<>();
		for (JsonElement e : o.getAsJsonArray("places")) {
			JsonObject p = e.getAsJsonObject();
			places.add(new Place(p.get("n").getAsString(), p.get("x").getAsInt(), p.get("z").getAsInt()));
		}
		return new Roads(xz, links, wayNames, places);
	}

	public List<Place> places() {
		return places;
	}

	public Place place(String name) {
		String want = name.trim().toLowerCase(Locale.ROOT);
		return places.stream().filter(p -> p.name().toLowerCase(Locale.ROOT).equals(want)).findFirst().orElse(null);
	}

	/** The way from the street nearest (sx, sz) to (tx, tz), or null when no street joins them. */
	public Route route(int sx, int sz, int tx, int tz) {
		int start = nearest(sx, sz), goal = nearest(tx, tz);
		if (start < 0 || goal < 0) return null;
		int n = xz.length / 2;
		double[] g = new double[n];
		Arrays.fill(g, Double.POSITIVE_INFINITY);
		int[] from = new int[n], by = new int[n];
		Arrays.fill(from, -1);
		g[start] = 0;
		// A*, the straight line to the goal never longer than the way there.
		PriorityQueue<double[]> open = new PriorityQueue<>((a, b) -> Double.compare(a[0], b[0]));
		open.add(new double[] {dist(start, goal), start});
		while (!open.isEmpty()) {
			double[] top = open.poll();
			int at = (int) top[1];
			if (at == goal) break;
			if (top[0] > g[at] + dist(at, goal) + 1e-9) continue;
			int[] l = links[at];
			for (int i = 0; i < l.length; i += 2) {
				int to = l[i];
				double d = g[at] + dist(at, to);
				if (d < g[to]) {
					g[to] = d;
					from[to] = at;
					by[to] = l[i + 1];
					open.add(new double[] {d + dist(to, goal), to});
				}
			}
		}
		if (g[goal] == Double.POSITIVE_INFINITY) return null;
		List<int[]> points = new ArrayList<>();
		List<String> names = new ArrayList<>();
		for (int at = goal; at != -1; at = from[at]) {
			points.add(0, new int[] {xz[at * 2], xz[at * 2 + 1]});
			if (from[at] != -1) names.add(0, wayNames[by[at]]);
		}
		// The last stretch, off the street to the place itself.
		points.add(new int[] {tx, tz});
		names.add(names.isEmpty() ? "" : names.get(names.size() - 1));
		double length = g[goal] + Math.hypot(tx - xz[goal * 2], tz - xz[goal * 2 + 1]);
		return new Route(points, names, length);
	}

	private int nearest(int x, int z) {
		int best = -1;
		long bestD = Long.MAX_VALUE;
		for (int i = 0; i < xz.length / 2; i++) {
			if (links[i].length == 0) continue;
			long dx = xz[i * 2] - x, dz = xz[i * 2 + 1] - z, d = dx * dx + dz * dz;
			if (d < bestD) {
				bestD = d;
				best = i;
			}
		}
		return best;
	}

	private double dist(int a, int b) {
		return Math.hypot(xz[a * 2] - xz[b * 2], xz[a * 2 + 1] - xz[b * 2 + 1]);
	}

	/** "left", "right" or "" for going a → b → c; +z is south, so screen-wise y points down. */
	public static String turn(int[] a, int[] b, int[] c) {
		double cross = (double) (b[0] - a[0]) * (c[1] - b[1]) - (double) (b[1] - a[1]) * (c[0] - b[0]);
		double dot = (double) (b[0] - a[0]) * (c[0] - b[0]) + (double) (b[1] - a[1]) * (c[1] - b[1]);
		if (Math.abs(cross) < Math.abs(dot) * 0.27) return "";
		return cross > 0 ? "right" : "left";
	}
}
