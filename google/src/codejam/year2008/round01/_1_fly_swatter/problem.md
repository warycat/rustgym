<p><h3>Problem</h3>
<p>
  What are your chances of hitting a fly with a tennis racquet?
</p><p>
  To start with, ignore the racquet's handle. Assume the racquet is a perfect
  ring, of outer radius <b>R</b> and thickness <b>t</b> (so the inner radius of
  the ring is <b>R</b>−<b>t</b>).
</p><p>
  The ring is covered with horizontal and vertical strings. Each string is a
  cylinder of radius <b>r</b>. Each string is a chord of the ring (a straight
  line connecting two points of the circle). There is a gap of length <b>g</b>
  between neighbouring strings. The strings are symmetric with respect to the
  center of the racquet i.e. there is a pair of strings whose centers meet at
  the center of the ring.
</p><p>
  The fly is a sphere of radius <b>f</b>. Assume that the racquet is moving in
  a straight line perpendicular to the plane of the ring. Assume also that the
  fly's center is inside the outer radius of the racquet and is equally likely
  to be anywhere within that radius. Any overlap between the fly and the
  racquet (the ring or a string) counts as a hit.
</p><p style="text-align:center">
  <img src="https://codejam.googleapis.com/dashboard/get_file/AQj_6U0mWZ1I4-zFNJrEsKwit3HPdqMkHwEg2d9mTCEtOjcsJJfZ35TG9unV6w/test2.png">
</p>

<h3>Input</h3>
<p>
  One line containing an integer <b>N</b>, the number of test cases in the
  input file.
</p><p>
  The next <b>N</b> lines will each contain the numbers <b>f</b>, <b>R</b>,
  <b>t</b>, <b>r</b> and <b>g</b> separated by exactly one space. Also the
  numbers will have exactly 6 digits after the decimal point.
</p>

<h3>Output</h3>
<p>
  <b>N</b> lines, each of the form "Case #<b>k</b>: <b>P</b>", where <b>k</b>
  is the number of the test case and <b>P</b> is the probability of hitting the
  fly with a piece of the racquet.
</p><p>
  Answers with a relative or absolute error of at most 10<sup>-6</sup> will be
  considered correct.
</p>

<h3>Limits</h3>
<p>
  Time limit: 60 seconds per test set.<br>
  Memory limit: 1GB.<br>
  <b>f</b>, <b>R</b>, <b>t</b>, <b>r</b> and <b>g</b> will be positive and smaller or equal to 10000.<br>
  <b>t</b> &lt; <b>R</b><br>
  <b>f</b> &lt; <b>R</b><br>
  <b>r</b> &lt; <b>R</b><br>
</p>

<h4>Small dataset (Test set 1 - Visible)</h4>
<p>
  1 ≤ <b>N</b> ≤ 30<br>
  The total number of strings will be at most 60 (so at most 30 in each direction).
</p>

<h4>Large dataset (Test set 2 - Hidden)</h4>
<p>
  1 ≤ <b>N</b> ≤ 100<br>
  The total number of strings will be at most 2000 (so at most 1000 in each direction).
</p>
