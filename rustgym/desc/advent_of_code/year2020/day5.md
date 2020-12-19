<article class="day-desc"><h2>--- Day 5: Binary Boarding ---</h2><p>You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.</p>
<p>You write a <span title="No problem!">quick program</span> to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.</p>
<p>Instead of <a target="_blank" href="https://www.youtube.com/watch?v=oAHbLRjF0vo">zones or groups</a>, this airline uses <em>binary space partitioning</em> to seat people. A seat might be specified like <code>FBFBBFFRLR</code>, where <code>F</code> means "front", <code>B</code> means "back", <code>L</code> means "left", and <code>R</code> means "right".</p>
<p>The first 7 characters will either be <code>F</code> or <code>B</code>; these specify exactly one of the <em>128 rows</em> on the plane (numbered <code>0</code> through <code>127</code>). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the <em>front</em> (<code>0</code> through <code>63</code>) or the <em>back</em> (<code>64</code> through <code>127</code>). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.</p>
<p>For example, consider just the first seven characters of <code>FBFBBFFRLR</code>:</p>
<ul>
<li>Start by considering the whole range, rows <code>0</code> through <code>127</code>.</li>
<li><code>F</code> means to take the <em>lower half</em>, keeping rows <code>0</code> through <code>63</code>.</li>
<li><code>B</code> means to take the <em>upper half</em>, keeping rows <code>32</code> through <code>63</code>.</li>
<li><code>F</code> means to take the <em>lower half</em>, keeping rows <code>32</code> through <code>47</code>.</li>
<li><code>B</code> means to take the <em>upper half</em>, keeping rows <code>40</code> through <code>47</code>.</li>
<li><code>B</code> keeps rows <code>44</code> through <code>47</code>.</li>
<li><code>F</code> keeps rows <code>44</code> through <code>45</code>.</li>
<li>The final <code>F</code> keeps the lower of the two, <em>row <code>44</code></em>.</li>
</ul>
<p>The last three characters will be either <code>L</code> or <code>R</code>; these specify exactly one of the <em>8 columns</em> of seats on the plane (numbered <code>0</code> through <code>7</code>). The same process as above proceeds again, this time with only three steps.  <code>L</code> means to keep the <em>lower half</em>, while <code>R</code> means to keep the <em>upper half</em>.</p>
<p>For example, consider just the last 3 characters of <code>FBFBBFFRLR</code>:</p>
<ul>
<li>Start by considering the whole range, columns <code>0</code> through <code>7</code>.</li>
<li><code>R</code> means to take the <em>upper half</em>, keeping columns <code>4</code> through <code>7</code>.</li>
<li><code>L</code> means to take the <em>lower half</em>, keeping columns <code>4</code> through <code>5</code>.</li>
<li>The final <code>R</code> keeps the upper of the two, <em>column <code>5</code></em>.</li>
</ul>
<p>So, decoding <code>FBFBBFFRLR</code> reveals that it is the seat at <em>row <code>44</code>, column <code>5</code></em>.</p>
<p>Every seat also has a unique <em>seat ID</em>: multiply the row by 8, then add the column. In this example, the seat has ID <code>44 * 8 + 5 = <em>357</em></code>.</p>
<p>Here are some other boarding passes:</p>
<ul>
<li><code>BFFFBBFRRR</code>: row <code>70</code>, column <code>7</code>, seat ID <code>567</code>.</li>
<li><code>FFFBBBFRRR</code>: row <code>14</code>, column <code>7</code>, seat ID <code>119</code>.</li>
<li><code>BBFFBBFRLL</code>: row <code>102</code>, column <code>4</code>, seat ID <code>820</code>.</li>
</ul>
<p>As a sanity check, look through your list of boarding passes. <em>What is the highest seat ID on a boarding pass?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p><em>Ding!</em> The "fasten seat belt" signs have turned on. Time to find your seat.</p>
<p>It's a completely full flight, so your seat should be the only missing boarding pass in your list.  However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.</p>
<p>Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.</p>
<p><em>What is the ID of your seat?</em></p>
</article>