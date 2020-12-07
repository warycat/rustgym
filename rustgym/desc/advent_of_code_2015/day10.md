<article class="day-desc"><h2>--- Day 10: Elves Look, Elves Say ---</h2><p>Today, the Elves are playing a game called <a href="https://en.wikipedia.org/wiki/Look-and-say_sequence">look-and-say</a>.  They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence.  For example, <code>211</code> is read as "one two, two ones", which becomes <code>1221</code> (<code>1</code> <code>2</code>, <code>2</code> <code>1</code>s).</p>
<p>Look-and-say sequences are generated iteratively, using the previous value as input for the next step.  For each step, take the previous value, and replace each run of digits (like <code>111</code>) with the number of digits (<code>3</code>) followed by the digit itself (<code>1</code>).</p>
<p>For example:</p>
<ul>
<li><code>1</code> becomes <code>11</code> (<code>1</code> copy of digit <code>1</code>).</li>
<li><code>11</code> becomes <code>21</code> (<code>2</code> copies of digit <code>1</code>).</li>
<li><code>21</code> becomes <code>1211</code> (one <code>2</code> followed by one <code>1</code>).</li>
<li><code>1211</code> becomes <code>111221</code> (one <code>1</code>, one <code>2</code>, and two <code>1</code>s).</li>
<li><code>111221</code> becomes <code>312211</code> (three <code>1</code>s, two <code>2</code>s, and one <code>1</code>).</li>
</ul>
<p>Starting with the digits in your puzzle input, apply this process 40 times.  What is <em>the length of the result</em>?</p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>Neat, right? You might also enjoy hearing <a href="https://www.youtube.com/watch?v=ea7lJkEhytA">John Conway talking about this sequence</a> (that's Conway of <em>Conway's Game of Life</em> fame).</p>
<p>Now, starting again with the digits in your puzzle input, apply this process <em title="Only because any longer started taking alarmingly long on my test hardware, and I wanted to avoid excluding people.">50</em> times.  What is <em>the length of the new result</em>?</p>
</article>