<article class="day-desc"><h2>--- Day 19: Monster Messages ---</h2><p>You land in an airport surrounded by dense forest. As you walk to your high-speed train, the Elves at the <span title="This is a purely fictional organization. Any resemblance to actual organizations, past or present, is purely coincidental.">Mythical Information Bureau</span> contact you again. They think their satellite has collected an image of a <em>sea monster</em>! Unfortunately, the connection to the satellite is having problems, and many of the messages sent back from the satellite have been corrupted.</p>
<p>They sent you a list of <em>the rules valid messages should obey</em> and a list of <em>received messages</em> they've collected so far (your puzzle input).</p>
<p>The <em>rules for valid messages</em> (the top part of your puzzle input) are numbered and build upon each other. For example:</p>
<pre><code>0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"
</code></pre>
<p>Some rules, like <code>3: "b"</code>, simply match a single character (in this case, <code>b</code>).</p>
<p>The remaining rules list the sub-rules that must be followed; for example, the rule <code>0: 1 2</code> means that to match rule <code>0</code>, the text being checked must match rule <code>1</code>, and the text after the part that matched rule <code>1</code> must then match rule <code>2</code>.</p>
<p>Some of the rules have multiple lists of sub-rules separated by a pipe (<code>|</code>). This means that <em>at least one</em> list of sub-rules must match. (The ones that match might be different each time the rule is encountered.) For example, the rule <code>2: 1 3 | 3 1</code> means that to match rule <code>2</code>, the text being checked must match rule <code>1</code> followed by rule <code>3</code> <em>or</em> it must match rule <code>3</code> followed by rule <code>1</code>.</p>
<p>Fortunately, there are no loops in the rules, so the list of possible matches will be finite. Since rule <code>1</code> matches <code>a</code> and rule <code>3</code> matches <code>b</code>, rule <code>2</code> matches either <code>ab</code> or <code>ba</code>. Therefore, rule <code>0</code> matches <code>aab</code> or <code>aba</code>.</p>
<p>Here's a more interesting example:</p>
<pre><code>0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
</code></pre>
<p>Here, because rule <code>4</code> matches <code>a</code> and rule <code>5</code> matches <code>b</code>, rule <code>2</code> matches two letters that are the same (<code>aa</code> or <code>bb</code>), and rule <code>3</code> matches two letters that are different (<code>ab</code> or <code>ba</code>).</p>
<p>Since rule <code>1</code> matches rules <code>2</code> and <code>3</code> once each in either order, it must match two pairs of letters, one pair with matching letters and one pair with different letters. This leaves eight possibilities: <code>aaab</code>, <code>aaba</code>, <code>bbab</code>, <code>bbba</code>, <code>abaa</code>, <code>abbb</code>, <code>baaa</code>, or <code>babb</code>.</p>
<p>Rule <code>0</code>, therefore, matches <code>a</code> (rule <code>4</code>), then any of the eight options from rule <code>1</code>, then <code>b</code> (rule <code>5</code>): <code>aaaabb</code>, <code>aaabab</code>, <code>abbabb</code>, <code>abbbab</code>, <code>aabaab</code>, <code>aabbbb</code>, <code>abaaab</code>, or <code>ababbb</code>.</p>
<p>The <em>received messages</em> (the bottom part of your puzzle input) need to be checked against the rules so you can determine which are valid and which are corrupted. Including the rules and the messages together, this might look like:</p>
<pre><code>0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
</code></pre>
<p>Your goal is to determine <em>the number of messages that completely match rule <code>0</code></em>. In the above example, <code>ababbb</code> and <code>abbbab</code> match, but <code>bababa</code>, <code>aaabbb</code>, and <code>aaaabbb</code> do not, producing the answer <em><code>2</code></em>. The whole message must match all of rule <code>0</code>; there can't be extra unmatched characters in the message. (For example, <code>aaaabbb</code> might appear to match rule <code>0</code> above, but it has an extra unmatched <code>b</code> on the end.)</p>
<p><em>How many messages completely match rule <code>0</code>?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>As you look over the list of messages, you realize your matching rules aren't quite right. To fix them, completely replace rules <code>8: 42</code> and <code>11: 42 31</code> with the following:</p>
<pre><code>8: 42 | 42 8
11: 42 31 | 42 11 31
</code></pre>
<p>This small change has a big impact: now, the rules <em>do</em> contain loops, and the list of messages they could hypothetically match is infinite. You'll need to determine how these changes affect which messages are valid.</p>
<p>Fortunately, many of the rules are unaffected by this change; it might help to start by looking at which rules always match the same set of values and how <em>those</em> rules (especially rules <code>42</code> and <code>31</code>) are used by the new versions of rules <code>8</code> and <code>11</code>.</p>
<p>(Remember, <em>you only need to handle the rules you have</em>; building a solution that could handle any hypothetical combination of rules would be <a href="https://en.wikipedia.org/wiki/Formal_grammar" target="_blank">significantly more difficult</a>.)</p>
<p>For example:</p>
<pre><code>42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
</code></pre>
<p>Without updating rules <code>8</code> and <code>11</code>, these rules only match three messages: <code>bbabbbbaabaabba</code>, <code>ababaaaaaabaaab</code>, and <code>ababaaaaabbbaba</code>.</p>
<p>However, after updating rules <code>8</code> and <code>11</code>, a total of <em><code>12</code></em> messages match:</p>
<ul>
<li><code>bbabbbbaabaabba</code></li>
<li><code>babbbbaabbbbbabbbbbbaabaaabaaa</code></li>
<li><code>aaabbbbbbaaaabaababaabababbabaaabbababababaaa</code></li>
<li><code>bbbbbbbaaaabbbbaaabbabaaa</code></li>
<li><code>bbbababbbbaaaaaaaabbababaaababaabab</code></li>
<li><code>ababaaaaaabaaab</code></li>
<li><code>ababaaaaabbbaba</code></li>
<li><code>baabbaaaabbaaaababbaababb</code></li>
<li><code>abbbbabbbbaaaababbbbbbaaaababb</code></li>
<li><code>aaaaabbaabaaaaababaa</code></li>
<li><code>aaaabbaabbaaaaaaabbbabbbaaabbaabaaa</code></li>
<li><code>aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba</code></li>
</ul>
<p><em>After updating rules <code>8</code> and <code>11</code>, how many messages completely match rule <code>0</code>?</em></p>
</article>