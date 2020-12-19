<article class="day-desc"><h2>--- Day 1: Not Quite Lisp ---</h2><p>Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out!  To save Christmas, he needs you to collect <em class="star">fifty stars</em> by December 25th.</p>
<p>Collect stars by helping Santa solve puzzles.  Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.  Each puzzle grants <em class="star">one star</em>. <span title="Also, some puzzles contain Easter eggs like this one. Yes, I know it's not traditional to do Advent calendars for Easter.">Good luck!</span></p>
<p>Here's an easy puzzle to warm you up.</p>
<p>Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor <code>0</code>) and then follows the instructions one character at a time.</p>
<p>An opening parenthesis, <code>(</code>, means he should go up one floor, and a closing parenthesis, <code>)</code>, means he should go down one floor.</p>
<p>The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.</p>
<p>For example:</p>
<ul>
<li><code>(())</code> and <code>()()</code> both result in floor <code>0</code>.</li>
<li><code>(((</code> and <code>(()(()(</code> both result in floor <code>3</code>.</li>
<li><code>))(((((</code> also results in floor <code>3</code>.</li>
<li><code>())</code> and <code>))(</code> both result in floor <code>-1</code> (the first basement level).</li>
<li><code>)))</code> and <code>)())())</code> both result in floor <code>-3</code>.</li>
</ul>
<p>To <em>what floor</em> do the instructions take Santa?</p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>Now, given the same instructions, find the <em>position</em> of the first character that causes him to enter the basement (floor <code>-1</code>).  The first character in the instructions has position <code>1</code>, the second character has position <code>2</code>, and so on.</p>
<p>For example:</p>
<ul>
<li><code>)</code> causes him to enter the basement at character position <code>1</code>.</li>
<li><code>()())</code> causes him to enter the basement at character position <code>5</code>.</li>
</ul>
<p>What is the <em>position</em> of the character that causes Santa to first enter the basement?</p>
</article>