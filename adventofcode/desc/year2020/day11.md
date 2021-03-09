<article class="day-desc"><h2>--- Day 11: Seating System ---</h2><p>Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!</p>
<p>By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).</p>
<p>The seat layout fits neatly on a grid. Each position is either floor (<code>.</code>), an empty seat (<code>L</code>), or an occupied seat (<code>#</code>). For example, the initial seat layout might look like this:</p>
<pre><code>L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
</code></pre>
<p>Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the <em>number of occupied seats</em> adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:</p>
<ul>
<li>If a seat is <em>empty</em> (<code>L</code>) and there are <em>no</em> occupied seats adjacent to it, the seat becomes <em>occupied</em>.</li>
<li>If a seat is <em>occupied</em> (<code>#</code>) and <em>four or more</em> seats adjacent to it are also occupied, the seat becomes <em>empty</em>.</li>
<li>Otherwise, the seat's state does not change.</li>
</ul>
<p><span title="Floor... floor never changes.">Floor (<code>.</code>) never changes</span>; seats don't move, and nobody sits on the floor.</p>
<p>After one round of these rules, every seat in the example layout becomes occupied:</p>
<pre><code>#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
</code></pre>
<p>After a second round, the seats with four or more occupied adjacent seats become empty again:</p>
<pre><code>#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
</code></pre>
<p>This process continues for three more rounds:</p>
<pre><code>#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
</code></pre>
<pre><code>#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
</code></pre>
<pre><code>#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
</code></pre>
<p>At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count <em><code>37</code></em> occupied seats.</p>
<p>Simulate your seating area by applying the seating rules repeatedly until no seats change state. <em>How many seats end up occupied?</em></p>
</article>