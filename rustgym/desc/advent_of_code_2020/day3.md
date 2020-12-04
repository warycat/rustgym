<article class="day-desc"><h2>--- Day 3: Toboggan Trajectory ---</h2><p>With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's <span title="It looks like the toboggan steering system even runs on Intcode! Good thing you don't have to modify it.">very minimal steering</span> and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.</p>
<p>Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (<code>.</code>) and trees (<code>#</code>) you can see. For example:</p>
<pre><code>..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
</code></pre>
<p>These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:</p>
<pre><code><em>..##.......</em>..##.........##.........##.........##.........##.......  ---&gt;
<em>#...#...#..</em>#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
<em>.#....#..#.</em>.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
<em>..#.#...#.#</em>..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
<em>.#...##..#.</em>.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
<em>..#.##.....</em>..#.##.......#.##.......#.##.......#.##.......#.##.....  ---&gt;
<em>.#.#.#....#</em>.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
<em>.#........#</em>.#........#.#........#.#........#.#........#.#........#
<em>#.##...#...</em>#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
<em>#...##....#</em>#...##....##...##....##...##....##...##....##...##....#
<em>.#..#...#.#</em>.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  ---&gt;
</code></pre>
<p>You start on the open square (<code>.</code>) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).</p>
<p>The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by <em>counting all the trees</em> you would encounter for the slope <em>right 3, down 1</em>:</p>
<p>From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.</p>
<p>The locations you'd check in the above example are marked here with <code><em>O</em></code> where there was an open square and <code><em>X</em></code> where there was a tree:</p>
<pre><code>..##.........##.........##.........##.........##.........##.......  ---&gt;
#..<em>O</em>#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....<em>X</em>..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#<em>O</em>#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..<em>X</em>...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.<em>X</em>#.......#.##.......#.##.......#.##.......#.##.....  ---&gt;
.#.#.#....#.#.#.#.<em>O</em>..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........<em>X</em>.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.<em>X</em>#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#<em>X</em>....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...<em>X</em>.#.#..#...#.#.#..#...#.#.#..#...#.#  ---&gt;
</code></pre>
<p>In this example, traversing the map using this slope would cause you to encounter <code><em>7</em></code> trees.</p>
<p>Starting at the top-left corner of your map and following a slope of right 3 and down 1, <em>how many trees would you encounter?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.</p>
<p>Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:</p>
<ul>
<li>Right 1, down 1.</li>
<li>Right 3, down 1. (This is the slope you already checked.)</li>
<li>Right 5, down 1.</li>
<li>Right 7, down 1.</li>
<li>Right 1, down 2.</li>
</ul>
<p>In the above example, these slopes would find <code>2</code>, <code>7</code>, <code>3</code>, <code>4</code>, and <code>2</code> tree(s) respectively; multiplied together, these produce the answer <code><em>336</em></code>.</p>
<p><em>What do you get if you multiply together the number of trees encountered on each of the listed slopes?</em></p>
</article>