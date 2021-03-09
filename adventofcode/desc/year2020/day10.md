<article class="day-desc"><h2>--- Day 10: Adapter Array ---</h2><p>Patched into the aircraft's data port, you discover weather forecasts of a massive tropical storm. Before you can figure out whether it will impact your vacation plans, however, your device suddenly turns off!</p>
<p>Its battery is dead.</p>
<p>You'll need to plug it in. There's only one problem: the charging outlet near your seat produces the wrong number of <em>jolts</em>. Always prepared, you make a list of all of the joltage adapters in your bag.</p>
<p>Each of your joltage adapters is rated for a specific <em>output joltage</em> (your puzzle input). Any given adapter can take an input <code>1</code>, <code>2</code>, or <code>3</code> jolts <em>lower</em> than its rating and still produce its rated output joltage.</p>
<p>In addition, your device has a built-in joltage adapter rated for <em><code>3</code> jolts higher</em> than the highest-rated adapter in your bag. (If your adapter list were <code>3</code>, <code>9</code>, and <code>6</code>, your device's built-in adapter would be rated for <code>12</code> jolts.)</p>
<p>Treat the charging outlet near your seat as having an effective joltage rating of <code>0</code>.</p>
<p>Since you have some time to kill, you might as well test all of your adapters. Wouldn't want to get to your resort and realize you can't even charge your device!</p>
<p>If you <em>use every adapter in your bag</em> at once, what is the distribution of joltage differences between the charging outlet, the adapters, and your device?</p>
<p>For example, suppose that in your bag, you have adapters with the following joltage ratings:</p>
<pre><code>16
10
15
5
1
11
7
19
6
12
4
</code></pre>
<p>With these adapters, your device's built-in joltage adapter would be rated for <code>19 + 3 = <em>22</em></code> jolts, 3 higher than the highest-rated adapter.</p>
<p>Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use every adapter, you'd need to choose them like this:</p>
<ul>
<li>The charging outlet has an effective rating of <code>0</code> jolts, so the only adapters that could connect to it directly would need to have a joltage rating of <code>1</code>, <code>2</code>, or <code>3</code> jolts. Of these, only one you have is an adapter rated <code>1</code> jolt (difference of <em><code>1</code></em>).</li>
<li>From your <code>1</code>-jolt rated adapter, the only choice is your <code>4</code>-jolt rated adapter (difference of <em><code>3</code></em>).</li>
<li>From the <code>4</code>-jolt rated adapter, the adapters rated <code>5</code>, <code>6</code>, or <code>7</code> are valid choices. However, in order to not skip any adapters, you have to pick the adapter rated <code>5</code> jolts (difference of <em><code>1</code></em>).</li>
<li>Similarly, the next choices would need to be the adapter rated <code>6</code> and then the adapter rated <code>7</code> (with difference of <em><code>1</code></em> and <em><code>1</code></em>).</li>
<li>The only adapter that works with the <code>7</code>-jolt rated adapter is the one rated <code>10</code> jolts (difference of <em><code>3</code></em>).</li>
<li>From <code>10</code>, the choices are <code>11</code> or <code>12</code>; choose <code>11</code> (difference of <em><code>1</code></em>) and then <code>12</code> (difference of <em><code>1</code></em>).</li>
<li>After <code>12</code>, only valid adapter has a rating of <code>15</code> (difference of <em><code>3</code></em>), then <code>16</code> (difference of <em><code>1</code></em>), then <code>19</code> (difference of <em><code>3</code></em>).</li>
<li>Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is <code>22</code> jolts (always a difference of <em><code>3</code></em>).</li>
</ul>
<p>In this example, when using every adapter, there are <em><code>7</code></em> differences of 1 jolt and <em><code>5</code></em> differences of 3 jolts.</p>
<p>Here is a larger example:</p>
<pre><code>28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
</code></pre>
<p>In this larger example, in a chain that uses all of the adapters, there are <em><code>22</code></em> differences of 1 jolt and <em><code>10</code></em> differences of 3 jolts.</p>
<p>Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter and count the joltage differences between the charging outlet, the adapters, and your device. <em>What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the charging outlet to your device. The previous rules about when adapters can successfully connect still apply.</p>
<p>The first example above (the one that starts with <code>16</code>, <code>10</code>, <code>15</code>) supports the following arrangements:</p>
<pre><code>(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
(0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
(0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
</code></pre>
<p>(The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect the charging outlet to your device is <em><code>8</code></em>.</p>
<p>The second example above (the one that starts with <code>28</code>, <code>33</code>, <code>18</code>) has many arrangements. Here are a few:</p>
<pre><code>(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
46, 48, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
46, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
47, 48, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
47, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
48, 49, (52)
</code></pre>
<p>In total, this set of adapters can connect the charging outlet to your device in <em><code>19208</code></em> distinct arrangements.</p>
<p>You glance back down at your bag and try to remember why you brought so many adapters; there must be <em>more than a trillion</em> valid ways to arrange them! Surely, there must be <span title="Definitely itertools.">an efficient way</span> to count the arrangements.</p>
<p><em>What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?</em></p>
</article>