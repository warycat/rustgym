<article class="day-desc"><h2>--- Day 15: Rambunctious Recitation ---</h2><p>You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.</p>
<p>While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a <em>memory game</em> and are <span title="Of course they are.">ever so excited</span> to explain the rules!</p>
<p>In this game, the players take turns saying <em>numbers</em>. They begin by taking turns reading from a list of <em>starting numbers</em> (your puzzle input). Then, each turn consists of considering the <em>most recently spoken number</em>:</p>
<ul>
<li>If that was the <em>first</em> time the number has been spoken, the current player says <em><code>0</code></em>.</li>
<li>Otherwise, the number had been spoken before; the current player announces <em>how many turns apart</em> the number is from when it was previously spoken.</li>
</ul>
<p>So, after the starting numbers, each turn results in that player speaking aloud either <em><code>0</code></em> (if the last number is new) or an <em>age</em> (if the last number is a repeat).</p>
<p>For example, suppose the starting numbers are <code>0,3,6</code>:</p>
<ul>
<li><em>Turn 1</em>: The <code>1</code>st number spoken is a starting number, <em><code>0</code></em>.</li>
<li><em>Turn 2</em>: The <code>2</code>nd number spoken is a starting number, <em><code>3</code></em>.</li>
<li><em>Turn 3</em>: The <code>3</code>rd number spoken is a starting number, <em><code>6</code></em>.</li>
<li><em>Turn 4</em>: Now, consider the last number spoken, <code>6</code>. Since that was the first time the number had been spoken, the <code>4</code>th number spoken is <em><code>0</code></em>.</li>
<li><em>Turn 5</em>: Next, again consider the last number spoken, <code>0</code>. Since it <em>had</em> been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, <code>4</code>) and the turn number of the time it was most recently spoken before then (turn <code>1</code>). Thus, the <code>5</code>th number spoken is <code>4 - 1</code>, <em><code>3</code></em>.</li>
<li><em>Turn 6</em>: The last number spoken, <code>3</code> had also been spoken before, most recently on turns <code>5</code> and <code>2</code>. So, the <code>6</code>th number spoken is <code>5 - 2</code>, <em><code>3</code></em>.</li>
<li><em>Turn 7</em>: Since <code>3</code> was just spoken twice in a row, and the last two turns are <code>1</code> turn apart, the <code>7</code>th number spoken is <em><code>1</code></em>.</li>
<li><em>Turn 8</em>: Since <code>1</code> is new, the <code>8</code>th number spoken is <em><code>0</code></em>.</li>
<li><em>Turn 9</em>: <code>0</code> was last spoken on turns <code>8</code> and <code>4</code>, so the <code>9</code>th number spoken is the difference between them, <em><code>4</code></em>.</li>
<li><em>Turn 10</em>: <code>4</code> is new, so the <code>10</code>th number spoken is <em><code>0</code></em>.</li>
</ul>
<p>(The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)</p>
<p>Their question for you is: what will be the <em><code>2020</code>th</em> number spoken? In the example above, the <code>2020</code>th number spoken will be <code>436</code>.</p>
<p>Here are a few more examples:</p>
<ul>
<li>Given the starting numbers <code>1,3,2</code>, the <code>2020</code>th number spoken is <code>1</code>.</li>
<li>Given the starting numbers <code>2,1,3</code>, the <code>2020</code>th number spoken is <code>10</code>.</li>
<li>Given the starting numbers <code>1,2,3</code>, the <code>2020</code>th number spoken is <code>27</code>.</li>
<li>Given the starting numbers <code>2,3,1</code>, the <code>2020</code>th number spoken is <code>78</code>.</li>
<li>Given the starting numbers <code>3,2,1</code>, the <code>2020</code>th number spoken is <code>438</code>.</li>
<li>Given the starting numbers <code>3,1,2</code>, the <code>2020</code>th number spoken is <code>1836</code>.</li>
</ul>
<p>Given your starting numbers, <em>what will be the <code>2020</code>th number spoken?</em></p>
</article>