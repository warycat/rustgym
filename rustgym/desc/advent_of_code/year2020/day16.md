<article class="day-desc"><h2>--- Day 16: Ticket Translation ---</h2><p>As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.</p>
<p>Unfortunately, you <span title="This actually happened to me once, but I solved it by just asking someone.">can't actually <em>read</em> the words on the ticket</span>. You can, however, read the numbers, and so you figure out <em>the fields these tickets must have</em> and <em>the valid ranges</em> for values in those fields.</p>
<p>You collect the <em>rules for ticket fields</em>, the <em>numbers on your ticket</em>, and the <em>numbers on other nearby tickets</em> for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).</p>
<p>The <em>rules for ticket fields</em> specify a list of fields that exist <em>somewhere</em> on the ticket and the <em>valid ranges of values</em> for each field. For example, a rule like <code>class: 1-3 or 5-7</code> means that one of the fields in every ticket is named <code>class</code> and can be any value in the ranges <code>1-3</code> or <code>5-7</code> (inclusive, such that <code>3</code> and <code>5</code> are both valid in this field, but <code>4</code> is not).</p>
<p>Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:</p>
<pre><code>.--------------------------------------------------------.
| ????: 101    ?????: 102   ??????????: 103     ???: 104 |
|                                                        |
| ??: 301  ??: 302             ???????: 303      ??????? |
| ??: 401  ??: 402           ???? ????: 403    ????????? |
'--------------------------------------------------------'
</code></pre>
<p>Here, <code>?</code> represents text in a language you don't understand. This ticket might be represented as <code>101,102,103,104,301,302,303,401,402,403</code>; of course, the actual train tickets you're looking at are <em>much</em> more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!</p>
<p>Start by determining which tickets are <em>completely invalid</em>; these are tickets that contain values which <em>aren't valid for any field</em>. Ignore <em>your ticket</em> for now.</p>
<p>For example, suppose you have the following notes:</p>
<pre><code>class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,<em>4</em>,50
<em>55</em>,2,20
38,6,<em>12</em>
</code></pre>
<p>It doesn't matter which position corresponds to which field; you can identify invalid <em>nearby tickets</em> by considering only whether tickets contain <em>values that are not valid for any field</em>. In this example, the values on the first <em>nearby ticket</em> are all valid for at least one field. This is not true of the other three <em>nearby tickets</em>: the values <code>4</code>, <code>55</code>, and <code>12</code> are are not valid for any field. Adding together all of the invalid values produces your <em>ticket scanning error rate</em>: <code>4 + 55 + 12</code> = <em><code>71</code></em>.</p>
<p>Consider the validity of the <em>nearby tickets</em> you scanned. <em>What is your ticket scanning error rate?</em></p>
</article>