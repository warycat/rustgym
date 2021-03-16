<div class="problem-analysis"><div class="problem-analysis-header"><h4 class="header-4">Analysis</h4></div> <div class="problem-description"><p><p>
  We want to buy as many as possible houses. Intuitively, we can keep buying the cheapest house. The
  rationale is to save money at each step so we could buy more in the end. One way to implement this
  strategy is to sort all the houses by prices from low to high and then buy houses one by one until
  we run out of money.
</p>

<p>
  The sorting part has O(<b>N</b> log <b>N</b>) time complexity and the processing part has
  O(<b>N</b>) time complexity. Using counting sort could reduce the sorting complexity to
  O(<b>N</b>) since the range of the prices is [1, 1000]. The overall time complexity is
  O(<b>N</b>).
</p>

<p>
  Let's prove the correctness of this greedy algorithm. Let the solution produced by the greedy
  algorithm be <b>A</b> = {a<sub>1</sub>, a<sub>2</sub>, ..., a<sub>k</sub>} and an optimal solution
  <b>O</b> = {o<sub>1</sub>, o<sub>2</sub>, ..., o<sub>m</sub>}.
</p>

<p>
  If <b>O</b> and <b>A</b> are the same, we are done with the proof. Let's assume that there is at
  least one element o<sub>j</sub> in <b>O</b> that is not present in <b>A</b>. Because we always
  take the smallest element from the original set, we know that any element that is not in <b>A</b>
  is greater than or equal to any a<sub>i</sub> in <b>A</b>. We could replace o<sub>j</sub> with the
  absent element in <b>A</b> without worsening the solution, because there will always be element in
  <b>A</b> that is not in <b>O</b>. We then increased number of elements in common between <b>A</b>
  and <b>O</b>, hence we can repeat this operation only finite number of times. We could repeat this
  process until all the elements in <b>O</b> are elements in <b>A</b>. Therefore, <b>A</b> is as
  good as any optimal solution.
</p>
</p></div></div>
