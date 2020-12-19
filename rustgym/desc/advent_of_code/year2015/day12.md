<article class="day-desc"><h2>--- Day 12: JSAbacusFramework.io ---</h2><p>Santa's Accounting-Elves need help balancing the books after a recent order.  Unfortunately, their accounting software uses a peculiar storage format.  That's where you come in.</p>
<p>They have a <a href="http://json.org/">JSON</a> document which contains a variety of things: arrays (<code>[1,2,3]</code>), objects (<code>{"a":1, "b":2}</code>), numbers, and strings.  Your first job is to simply find all of the <em>numbers</em> throughout the document and add them together.</p>
<p>For example:</p>
<ul>
<li><code>[1,2,3]</code> and <code>{"a":2,"b":4}</code> both have a sum of <code>6</code>.</li>
<li><code>[[[3]]]</code> and <code>{"a":{"b":4},"c":-1}</code> both have a sum of <code>3</code>.</li>
<li><code>{"a":[-1,1]}</code> and <code>[-1,{"a":1}]</code> both have a sum of <code>0</code>.</li>
<li><code>[]</code> and <code>{}</code> both have a sum of <code>0</code>.</li>
</ul>
<p>You will not <span title="Nor are you likely to be eaten by a grue... during *this* puzzle, anyway.">encounter</span> any strings containing numbers.</p>
<p>What is the <em>sum of all numbers</em> in the document?</p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>Uh oh - the Accounting-Elves have realized that they double-counted everything <em>red</em>.</p>
<p>Ignore any object (and all of its children) which has any property with the value <code>"red"</code>.  Do this only for objects (<code>{...}</code>), not arrays (<code>[...]</code>).</p>
<ul>
<li><code>[1,2,3]</code> still has a sum of <code>6</code>.</li>
<li><code>[1,{"c":"red","b":2},3]</code> now has a sum of <code>4</code>, because the middle object is ignored.</li>
<li><code>{"d":"red","e":[1,2,3,4],"f":5}</code> now has a sum of <code>0</code>, because the entire structure is ignored.</li>
<li><code>[1,"red",5]</code> has a sum of <code>6</code>, because <code>"red"</code> in an array has no effect.</li>
</ul>
</article>