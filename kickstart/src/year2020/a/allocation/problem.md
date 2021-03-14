<p><h3>Problem</h3>
<p>
  There are <b>N</b> houses for sale.
  The i-th house costs <b>A<sub>i</sub></b> dollars to buy.
  You have a budget of <b>B</b> dollars to spend.
</p><p>
  What is the maximum number of houses you can buy?
</p>

<h3>Input</h3>
<p>
  The first line of the input gives the number of test cases, <b>T</b>.
  <b>T</b> test cases follow. Each test case begins with a single line containing the two integers <b>N</b> and <b>B</b>.
  The second line contains <b>N</b> integers. The i-th integer is <b>A<sub>i</sub></b>, the cost of the i-th house.
</p>

<h3>Output</h3>
<p>
  For each test case, output one line containing <code>Case #x: y</code>, where <code>x</code> is the test case number (starting from 1) and <code>y</code> is
  the maximum number of houses you can buy.
</p>

<h3>Limits</h3>
<p>
Time limit: 15 seconds per test set.<br>
Memory limit: 1GB.<br>
1 ≤ <b>T</b> ≤ 100.<br>
1 ≤ <b>B</b> ≤ 10<sup>5</sup>.<br>
1 ≤ <b>A<sub>i</sub></b> ≤ 1000, for all i.<br>
</p>

<h4>Test set 1</h4>
<p>
  1 ≤ <b>N</b> ≤ 100.<br>
</p>

<h4>Test set 2</h4>
<p>
  1 ≤ <b>N</b> ≤ 10<sup>5</sup>.<br>
</p>

  <h3>Sample</h3>
  <div class="problem-io-wrapper">
  <table>
  <tbody><tr>
  <td>
  <br>
  <span class="io-table-header">Input</span>
  <br>&nbsp;
  </td>
  <td>
  <br>
  <span class="io-table-header">Output</span>
  <br>&nbsp;
  </td>
  </tr>
  <tr>
  <td>
  <pre class="io-content">3
4 100
20 90 40 90
4 50
30 30 10 10
3 300
999 999 999

  </pre>
  </td>
  <td>
  <pre class="io-content">Case #1: 2
Case #2: 3
Case #3: 0

  </pre>
  </td></tr></tbody></table>
  </div>


In Sample Case #1, you have a budget of 100 dollars. You can buy the 1st and 3rd houses for 20 + 40 = 60 dollars.<br>
In Sample Case #2, you have a budget of 50 dollars. You can buy the 1st, 3rd and 4th houses for 30 + 10 + 10 = 50 dollars.<br>
In Sample Case #3, you have a budget of 300 dollars. You cannot buy any houses (so the answer is 0).<br>

<p>
  <b>Note:</b> Unlike previous editions, in Kick Start 2020, all test sets are visible verdict test sets, meaning you receive instant feedback upon submission.
</p>

</p>
