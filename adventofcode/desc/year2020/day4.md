<article class="day-desc"><h2>--- Day 4: Passport Processing ---</h2><p>You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.</p>
<p>It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.</p>
<p>Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.</p>
<p>The automatic passport scanners are slow because they're having trouble <em>detecting which passports have all required fields</em>. The expected fields are as follows:</p>
<ul>
<li><code>byr</code> (Birth Year)</li>
<li><code>iyr</code> (Issue Year)</li>
<li><code>eyr</code> (Expiration Year)</li>
<li><code>hgt</code> (Height)</li>
<li><code>hcl</code> (Hair Color)</li>
<li><code>ecl</code> (Eye Color)</li>
<li><code>pid</code> (Passport ID)</li>
<li><code>cid</code> (Country ID)</li>
</ul>
<p>Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of <code>key:value</code> pairs separated by spaces or newlines. Passports are separated by blank lines.</p>
<p>Here is an example batch file containing four passports:</p>
<pre><code>ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
</code></pre>
<p>The first passport is <em>valid</em> - all eight fields are present. The second passport is <em>invalid</em> - it is missing <code>hgt</code> (the Height field).</p>
<p>The third passport is interesting; the <em>only missing field</em> is <code>cid</code>, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing <code>cid</code> fields.  Treat this "passport" as <em>valid</em>.</p>
<p>The fourth passport is missing two fields, <code>cid</code> and <code>byr</code>. Missing <code>cid</code> is fine, but missing any other field is not, so this passport is <em>invalid</em>.</p>
<p>According to the above rules, your improved system would report <code><em>2</em></code> valid passports.</p>
<p>Count the number of <em>valid</em> passports - those that have all required fields. Treat <code>cid</code> as optional. <em>In your batch file, how many passports are valid?</em></p>
</article>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2><p>The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!</p>
<p>You can continue to ignore the <code>cid</code> field, but each other field has <span title="GLORY TO ARSTOTZKA">strict rules</span> about what values are valid for automatic validation:</p>
<ul>
<li><code>byr</code> (Birth Year) - four digits; at least <code>1920</code> and at most <code>2002</code>.</li>
<li><code>iyr</code> (Issue Year) - four digits; at least <code>2010</code> and at most <code>2020</code>.</li>
<li><code>eyr</code> (Expiration Year) - four digits; at least <code>2020</code> and at most <code>2030</code>.</li>
<li><code>hgt</code> (Height) - a number followed by either <code>cm</code> or <code>in</code>:
  <ul>
  <li>If <code>cm</code>, the number must be at least <code>150</code> and at most <code>193</code>.</li>
  <li>If <code>in</code>, the number must be at least <code>59</code> and at most <code>76</code>.</li>
  </ul>
</li>
<li><code>hcl</code> (Hair Color) - a <code>#</code> followed by exactly six characters <code>0</code>-<code>9</code> or <code>a</code>-<code>f</code>.</li>
<li><code>ecl</code> (Eye Color) - exactly one of: <code>amb</code> <code>blu</code> <code>brn</code> <code>gry</code> <code>grn</code> <code>hzl</code> <code>oth</code>.</li>
<li><code>pid</code> (Passport ID) - a nine-digit number, including leading zeroes.</li>
<li><code>cid</code> (Country ID) - ignored, missing or not.</li>
</ul>
<p>Your job is to count the passports where all required fields are both <em>present</em> and <em>valid</em> according to the above rules. Here are some example values:</p>
<pre><code>byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789
</code></pre>
<p>Here are some invalid passports:</p>
<pre><code>eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
</code></pre>
<p>Here are some valid passports:</p>
<pre><code>pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
</code></pre>
<p>Count the number of <em>valid</em> passports - those that have all required fields <em>and valid values</em>. Continue to treat <code>cid</code> as optional. <em>In your batch file, how many passports are valid?</em></p>
</article>