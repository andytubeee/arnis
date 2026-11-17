package dev.arnis.map;

import static org.junit.jupiter.api.Assertions.*;

import java.util.List;
import org.junit.jupiter.api.Test;

class RoadsTest {
	// A square of streets: King St along the top, Queen St along the bottom, and two short
	// sides; a long way round on the left, a short one on the right.
	static final String JSON = """
		{"v":1,"nodes":[0,0, 100,0, 0,100, 100,100, 0,500],
		 "ways":[{"n":"King St","p":[0,1]},{"n":"Bay St","p":[1,3]},
		         {"n":"Queen St","p":[3,2]},{"n":"","p":[0,4,2]}],
		 "places":[{"n":"Grange Park","x":5,"z":98}]}
		""";

	@Test
	void takes_the_short_way_along_the_streets() {
		Roads roads = Roads.parse(JSON);
		Roads.Route r = roads.route(1, 1, 2, 99);
		assertNotNull(r);
		assertEquals(List.of("King St", "Bay St", "Queen St"), r.streets());
		assertArrayEquals(new int[] {0, 0}, r.points().get(0));
		assertArrayEquals(new int[] {2, 99}, r.points().get(r.points().size() - 1));
		assertEquals(r.points().size() - 1, r.legs().size());
		assertEquals(300, r.length(), 5);
	}

	@Test
	void finds_places_by_name_ignoring_case() {
		Roads roads = Roads.parse(JSON);
		assertEquals(5, roads.place("grange park").x());
		assertNull(roads.place("Nowhere"));
	}

	@Test
	void unconnected_streets_have_no_route() {
		Roads roads = Roads.parse("""
			{"v":1,"nodes":[0,0, 10,0, 500,500, 510,500],
			 "ways":[{"n":"A","p":[0,1]},{"n":"B","p":[2,3]}],"places":[]}
			""");
		assertNull(roads.route(0, 0, 510, 500));
	}

	@Test
	void says_which_way_to_turn() {
		// Heading east, then south (+z) is a right turn in Minecraft.
		assertEquals("right", Roads.turn(new int[] {0, 0}, new int[] {100, 0}, new int[] {100, 100}));
		assertEquals("left", Roads.turn(new int[] {0, 0}, new int[] {100, 0}, new int[] {100, -100}));
	}
}
