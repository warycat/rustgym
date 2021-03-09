<article class="day-desc"><h2>--- Day 9: Encoding Error ---</h2><p>With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.</p>
<p>Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).</p>
<p>The data appears to be encrypted with the eXchange-Masking Addition System (<span title="No relation.">XMAS</span>) which, conveniently for you, is an old cypher with an important weakness.</p>
<p>XMAS starts by transmitting a <em>preamble</em> of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.</p>
<p>For example, suppose your preamble consists of the numbers <code>1</code> through <code>25</code> in a random order. To be valid, the next number must be the sum of two of those numbers:</p>
<ul>
<li><code>26</code> would be a <em>valid</em> next number, as it could be <code>1</code> plus <code>25</code> (or many other pairs, like <code>2</code> and <code>24</code>).</li>
<li><code>49</code> would be a <em>valid</em> next number, as it is the sum of <code>24</code> and <code>25</code>.</li>
<li><code>100</code> would <em>not</em> be valid; no two of the previous 25 numbers sum to <code>100</code>.</li>
<li><code>50</code> would also <em>not</em> be valid; although <code>25</code> appears in the previous 25 numbers, the two numbers in the pair must be different.</li>
</ul>
<p>Suppose the 26th number is <code>45</code>, and the first number (no longer an option, as it is more than 25 numbers ago) was <code>20</code>. Now, for the next number to be valid, there needs to be some pair of numbers among <code>1</code>-<code>19</code>, <code>21</code>-<code>25</code>, or <code>45</code> that add up to it:</p>
<ul>
<li><code>26</code> would still be a <em>valid</em> next number, as <code>1</code> and <code>25</code> are still within the previous 25 numbers.</li>
<li><code>65</code> would <em>not</em> be valid, as no two of the available numbers sum to it.</li>
<li><code>64</code> and <code>66</code> would both be <em>valid</em>, as they are the result of <code>19+45</code> and <code>21+45</code> respectively.</li>
</ul>
<p>Here is a larger example which only considers the previous <em>5</em> numbers (and has a preamble of length 5):</p>
<pre><code>35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
</code></pre>
<p>In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is <em><code>127</code></em>.</p>
<p>The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is <em>not</em> the sum of two of the 25 numbers before it. <em>What is the first number that does not have this property?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>The final step in breaking the XMAS encryption relies on the invalid number you just found: you must <em>find a contiguous set of at least two numbers</em> in your list which sum to the invalid number from step 1.</p>
<p>Again consider the above example:</p>
<pre><code>35
20
<em>15</em>
<em>25</em>
<em>47</em>
<em>40</em>
62
55
65
95
102
117
150
182
127
219
299
277
309
576
</code></pre>
<p>In this list, adding up all of the numbers from <code>15</code> through <code>40</code> produces the invalid number from step 1, <code>127</code>. (Of course, the contiguous set of numbers in your actual list might be much longer.)</p>
<p>To find the <em>encryption weakness</em>, add together the <em>smallest</em> and <em>largest</em> number in this contiguous range; in this example, these are <code>15</code> and <code>47</code>, producing <em><code>62</code></em>.</p>
<p><em>What is the encryption weakness in your XMAS-encrypted list of numbers?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>The final step in breaking the XMAS encryption relies on the invalid number you just found: you must <em>find a contiguous set of at least two numbers</em> in your list which sum to the invalid number from step 1.</p>
<p>Again consider the above example:</p>
<pre><code>35
20
<em>15</em>
<em>25</em>
<em>47</em>
<em>40</em>
62
55
65
95
102
117
150
182
127
219
299
277
309
576
</code></pre>
<p>In this list, adding up all of the numbers from <code>15</code> through <code>40</code> produces the invalid number from step 1, <code>127</code>. (Of course, the contiguous set of numbers in your actual list might be much longer.)</p>
<p>To find the <em>encryption weakness</em>, add together the <em>smallest</em> and <em>largest</em> number in this contiguous range; in this example, these are <code>15</code> and <code>47</code>, producing <em><code>62</code></em>.</p>
<p><em>What is the encryption weakness in your XMAS-encrypted list of numbers?</em></p>
</article>