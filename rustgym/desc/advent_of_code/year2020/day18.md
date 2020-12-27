<article class="day-desc"><h2>--- Day 18: Operation Order ---</h2><p>As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their <span title="Or &quot;maths&quot;, if you have more than one.">math</span> homework.</p>
<p>Unfortunately, it seems like this "math" <a href="https://www.youtube.com/watch?v=3QtRK7Y2pPU&amp;t=15" target="_blank">follows different rules</a> than you remember.</p>
<p>The homework (your puzzle input) consists of a series of expressions that consist of addition (<code>+</code>), multiplication (<code>*</code>), and parentheses (<code>(...)</code>). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.</p>
<p>However, the rules of <em>operator precedence</em> have changed. Rather than evaluating multiplication before addition, the operators have the <em>same precedence</em>, and are evaluated left-to-right regardless of the order in which they appear.</p>
<p>For example, the steps to evaluate the expression <code>1 + 2 * 3 + 4 * 5 + 6</code> are as follows:</p>
<pre><code><em>1 + 2</em> * 3 + 4 * 5 + 6
  <em>3   * 3</em> + 4 * 5 + 6
      <em>9   + 4</em> * 5 + 6
         <em>13   * 5</em> + 6
             <em>65   + 6</em>
                 <em>71</em>
</code></pre>
<p>Parentheses can override this order; for example, here is what happens if parentheses are added to form <code>1 + (2 * 3) + (4 * (5 + 6))</code>:</p>
<pre><code>1 + <em>(2 * 3)</em> + (4 * (5 + 6))
<em>1 +    6</em>    + (4 * (5 + 6))
     7      + (4 * <em>(5 + 6)</em>)
     7      + <em>(4 *   11   )</em>
     <em>7      +     44</em>
            <em>51</em>
</code></pre>
<p>Here are a few more examples:</p>
<ul>
<li><code>2 * 3 + (4 * 5)</code> becomes <em><code>26</code></em>.</li>
<li><code>5 + (8 * 3 + 9 + 3 * 4 * 3)</code> becomes <em><code>437</code></em>.</li>
<li><code>5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))</code> becomes <em><code>12240</code></em>.</li>
<li><code>((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2</code> becomes <em><code>13632</code></em>.</li>
</ul>
<p>Before you can help with the homework, you need to understand it yourself. <em>Evaluate the expression on each line of the homework; what is the sum of the resulting values?</em></p>
</article>