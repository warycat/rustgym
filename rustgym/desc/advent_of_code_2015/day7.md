<article class="day-desc"><h2>--- Day 7: Some Assembly Required ---</h2><p>This year, Santa brought little Bobby Tables a set of wires and <a href="https://en.wikipedia.org/wiki/Bitwise_operation">bitwise logic gates</a>!  Unfortunately, little Bobby is a little under the recommended age range, and he needs help <span title="You had one of these as a kid, right?">assembling the circuit</span>.</p>
<p>Each wire has an identifier (some lowercase letters) and can carry a <a href="https://en.wikipedia.org/wiki/16-bit">16-bit</a> signal (a number from <code>0</code> to <code>65535</code>).  A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations.  A gate provides no signal until all of its inputs have a signal.</p>
<p>The included instructions booklet describes how to connect the parts together: <code>x AND y -&gt; z</code> means to connect wires <code>x</code> and <code>y</code> to an AND gate, and then connect its output to wire <code>z</code>.</p>
<p>For example:</p>
<ul>
<li><code>123 -&gt; x</code> means that the signal <code>123</code> is provided to wire <code>x</code>.</li>
<li><code>x AND y -&gt; z</code> means that the <a href="https://en.wikipedia.org/wiki/Bitwise_operation#AND">bitwise AND</a> of wire <code>x</code> and wire <code>y</code> is provided to wire <code>z</code>.</li>
<li><code>p LSHIFT 2 -&gt; q</code> means that the value from wire <code>p</code> is <a href="https://en.wikipedia.org/wiki/Logical_shift">left-shifted</a> by <code>2</code> and then provided to wire <code>q</code>.</li>
<li><code>NOT e -&gt; f</code> means that the <a href="https://en.wikipedia.org/wiki/Bitwise_operation#NOT">bitwise complement</a> of the value from wire <code>e</code> is provided to wire <code>f</code>.</li>
</ul>
<p>Other possible gates include <code>OR</code> (<a href="https://en.wikipedia.org/wiki/Bitwise_operation#OR">bitwise OR</a>) and <code>RSHIFT</code> (<a href="https://en.wikipedia.org/wiki/Logical_shift">right-shift</a>).  If, for some reason, you'd like to <em>emulate</em> the circuit instead, almost all programming languages (for example, <a href="https://en.wikipedia.org/wiki/Bitwise_operations_in_C">C</a>, <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_Operators">JavaScript</a>, or <a href="https://wiki.python.org/moin/BitwiseOperators">Python</a>) provide operators for these gates.</p>
<p>For example, here is a simple circuit:</p>
<pre><code>123 -&gt; x
456 -&gt; y
x AND y -&gt; d
x OR y -&gt; e
x LSHIFT 2 -&gt; f
y RSHIFT 2 -&gt; g
NOT x -&gt; h
NOT y -&gt; i
</code></pre>
<p>After it is run, these are the signals on the wires:</p>
<pre><code>d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456
</code></pre>
<p>In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to <em>wire <code>a</code></em>?</p>
</article>