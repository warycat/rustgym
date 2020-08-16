# Leetcode Solutions in Rust


This project demostrates how to create **Data Structures** and to implement **Algorithms** using programming language **Rust**
All the solutions here are crafted with love and their performance beats 99% of other solutions on the leetcode website. Tutorial videos will be added later.

### Please subscribe to our [Rust Gym Youtube Channel](https://www.youtube.com/channel/UCV9HzRLPKjI8SttaIYOygsw) for future videos.

<details><summary>Data Structures</summary>

- Stack & Queue ( Vec, VecDeque )
- Linked List ( Option<Box<ListNode>> )
- Hash Tables ( HashMap, HashSet )
- Tree Tables ( BTreeMap, BTreeSet )
- Binary Search Tree ( Option<Rc<RefCell<TreeNode>>> )
- Binary Heaps & Priority Queue ( BinaryHeap )
- Graphs ( Vec<Vec<usize>> )
- Union Find ( UnionFind )
- Trie ( Trie )
</details>

<details><summary>Algorithms</summary>

- Bit Manipulation & Numbers
- Stability in Sorting
- Heapsort
- Binary Search
- Kth Smallest Elements
- Permutations
- Subsets
- BFS Graph
- DFS Graph
- Dijkstra’s Algorithm
- Tree Traversals
    - BFS
    - DFS
        - in-order
        - pre-order
        - post-order
- Topological Sort
- Detect cycle in an undirected graph
- Detect a cycle in a directed graph
- Count connected components in a graph
- Find strongly connected components in a graph
</details>


### Docs [![Docs Status](https://docs.rs/rustgym/badge.svg)](https://docs.rs/rustgym)

### Build [![Build Status](https://travis-ci.org/warycat/leetcode_rs.svg?branch=master)](https://travis-ci.org/warycat/leetcode_rs)

# All Solutions
<details><summary>Easy 11/372 2.96%</summary>


|id|372 Easy Questions|Tags|361 Solutions|
|---|---|---|---|
|190|[Reverse Bits](https://leetcode.com/problems/reverse-bits)|bit-manipulation|   |
|690|[Employee Importance](https://leetcode.com/problems/employee-importance)|hash-table depth-first-search breadth-first-search|   |
|160|[Intersection of Two Linked Lists](https://leetcode.com/problems/intersection-of-two-linked-lists)|linked-list|   |
|237|[Delete Node in a Linked List](https://leetcode.com/problems/delete-node-in-a-linked-list)|linked-list|   |
|141|[Linked List Cycle](https://leetcode.com/problems/linked-list-cycle)|linked-list two-pointers|   |
|1237|[Find Positive Integer Solution for a Given Equation](https://leetcode.com/problems/find-positive-integer-solution-for-a-given-equation)|math binary-search|   |
|157|[Read N Characters Given Read4](https://leetcode.com/problems/read-n-characters-given-read4)|string|   |
|235|[Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree)|tree|   |
|589|[N-ary Tree Preorder Traversal](https://leetcode.com/problems/n-ary-tree-preorder-traversal)|tree|   |
|590|[N-ary Tree Postorder Traversal](https://leetcode.com/problems/n-ary-tree-postorder-traversal)|tree|   |
|559|[Maximum Depth of N-ary Tree](https://leetcode.com/problems/maximum-depth-of-n-ary-tree)|tree depth-first-search breadth-first-search|   |
|1002|[Find Common Characters](https://leetcode.com/problems/find-common-characters)|array hash-table|[solution](rustgym/src/leetcode/_1002_find_common_characters.rs)|
|1005|[Maximize Sum Of Array After K Negations](https://leetcode.com/problems/maximize-sum-of-array-after-k-negations)|greedy|[solution](rustgym/src/leetcode/_1005_maximize_sum_of_array_after_k_negations.rs)|
|1009|[Complement of Base 10 Integer](https://leetcode.com/problems/complement-of-base-10-integer)|math|[solution](rustgym/src/leetcode/_1009_complement_of_base_10_integer.rs)|
|100|[Same Tree](https://leetcode.com/problems/same-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_100_same_tree.rs)|
|1010|[Pairs of Songs With Total Durations Divisible by 60](https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60)|array|[solution](rustgym/src/leetcode/_1010_pairs_of_songs_with_total_durations_divisible_by_60.rs)|
|1013|[Partition Array Into Three Parts With Equal Sum](https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum)|array|[solution](rustgym/src/leetcode/_1013_partition_array_into_three_parts_with_equal_sum.rs)|
|1018|[Binary Prefix Divisible By 5](https://leetcode.com/problems/binary-prefix-divisible-by-5)|array|[solution](rustgym/src/leetcode/_1018_binary_prefix_divisible_by_5.rs)|
|101|[Symmetric Tree](https://leetcode.com/problems/symmetric-tree)|tree depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_101_symmetric_tree.rs)|
|1021|[Remove Outermost Parentheses](https://leetcode.com/problems/remove-outermost-parentheses)|stack|[solution](rustgym/src/leetcode/_1021_remove_outermost_parentheses.rs)|
|1022|[Sum of Root To Leaf Binary Numbers](https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers)|tree|[solution](rustgym/src/leetcode/_1022_sum_root_to_leaf_binary_number.rs)|
|1025|[Divisor Game](https://leetcode.com/problems/divisor-game)|math dynamic-programming|[solution](rustgym/src/leetcode/_1025_divisor_game.rs)|
|1029|[Two City Scheduling](https://leetcode.com/problems/two-city-scheduling)|greedy|[solution](rustgym/src/leetcode/_1029_two_city_scheduling.rs)|
|1030|[Matrix Cells in Distance Order](https://leetcode.com/problems/matrix-cells-in-distance-order)|sort|[solution](rustgym/src/leetcode/_1030_matrix_cells_in_distance_order.rs)|
|1033|[Moving Stones Until Consecutive](https://leetcode.com/problems/moving-stones-until-consecutive)|brainteaser|[solution](rustgym/src/leetcode/_1033_moving_stones_until_consecutive.rs)|
|1037|[Valid Boomerang](https://leetcode.com/problems/valid-boomerang)|math|[solution](rustgym/src/leetcode/_1037_valid_boomerang.rs)|
|1042|[Flower Planting With No Adjacent](https://leetcode.com/problems/flower-planting-with-no-adjacent)|graph|[solution](rustgym/src/leetcode/_1042_flower_planting_with_no_adjacent.rs)|
|1046|[Last Stone Weight](https://leetcode.com/problems/last-stone-weight)|heap greedy|[solution](rustgym/src/leetcode/_1046_last_stone_weight.rs)|
|1047|[Remove All Adjacent Duplicates In String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string)|stack|[solution](rustgym/src/leetcode/_1047_remove_all_adjacent_duplicates_in_string.rs)|
|104|[Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_104_maximum_depth_of_binary_tree.rs)|
|1051|[Height Checker](https://leetcode.com/problems/height-checker)|array|[solution](rustgym/src/leetcode/_1051_height_checker.rs)|
|1056|[Confusing Number](https://leetcode.com/problems/confusing-number)|math|[solution](rustgym/src/leetcode/_1056_confusing_number.rs)|
|1064|[Fixed Point](https://leetcode.com/problems/fixed-point)|array binary-search|[solution](rustgym/src/leetcode/_1064_fixed_point.rs)|
|1065|[Index Pairs of a String](https://leetcode.com/problems/index-pairs-of-a-string)|string trie|[solution](rustgym/src/leetcode/_1065_index_pairs_of_a_string.rs)|
|1071|[Greatest Common Divisor of Strings](https://leetcode.com/problems/greatest-common-divisor-of-strings)|string|[solution](rustgym/src/leetcode/_1071_greatest_common_divisor_of_strings.rs)|
|1078|[Occurrences After Bigram](https://leetcode.com/problems/occurrences-after-bigram)|hash-table|[solution](rustgym/src/leetcode/_1078_occurrences_after_bigram.rs)|
|107|[Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii)|tree breadth-first-search|[solution](rustgym/src/leetcode/_107_binary_tree_level_order_traversal_2.rs)|
|1085|[Sum of Digits in the Minimum Number](https://leetcode.com/problems/sum-of-digits-in-the-minimum-number)|array|[solution](rustgym/src/leetcode/_1085_sum_of_digits_in_the_minmum_number.rs)|
|1086|[High Five](https://leetcode.com/problems/high-five)|array hash-table sort|[solution](rustgym/src/leetcode/_1086_high_five.rs)|
|1089|[Duplicate Zeros](https://leetcode.com/problems/duplicate-zeros)|array|[solution](rustgym/src/leetcode/_1089_duplicate_zeros.rs)|
|108|[Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_108_convert_sorted_array_binary_search_tree.rs)|
|1099|[Two Sum Less Than K](https://leetcode.com/problems/two-sum-less-than-k)|array|[solution](rustgym/src/leetcode/_1099_two_sum_less_than_k.rs)|
|1103|[Distribute Candies to People](https://leetcode.com/problems/distribute-candies-to-people)|math|[solution](rustgym/src/leetcode/_1103_distribute_candies_to_people.rs)|
|1108|[Defanging an IP Address](https://leetcode.com/problems/defanging-an-ip-address)|string|[solution](rustgym/src/leetcode/_1108_defanging_an_ip_address.rs)|
|110|[Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_110_balanced_binary_tree.rs)|
|1118|[Number of Days in a Month](https://leetcode.com/problems/number-of-days-in-a-month)||[solution](rustgym/src/leetcode/_1118_number_of_days_in_a_month.rs)|
|1119|[Remove Vowels from a String](https://leetcode.com/problems/remove-vowels-from-a-string)|string|[solution](rustgym/src/leetcode/_1119_remove_vowels_from_a_string.rs)|
|111|[Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree)|tree depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_111_minimum_depth_of_binary_tree.rs)|
|1122|[Relative Sort Array](https://leetcode.com/problems/relative-sort-array)|array sort|[solution](rustgym/src/leetcode/_1122_relative_sort_array.rs)|
|1128|[Number of Equivalent Domino Pairs](https://leetcode.com/problems/number-of-equivalent-domino-pairs)|array|[solution](rustgym/src/leetcode/_1128_number_of_equivalent_domino_pairs.rs)|
|112|[Path Sum](https://leetcode.com/problems/path-sum)|tree depth-first-search|[solution](rustgym/src/leetcode/_112_path_sum.rs)|
|1133|[Largest Unique Number](https://leetcode.com/problems/largest-unique-number)|array hash-table|[solution](rustgym/src/leetcode/_1133_largest_unique_number.rs)|
|1134|[Armstrong Number](https://leetcode.com/problems/armstrong-number)|math|[solution](rustgym/src/leetcode/_1134_armstrong_number.rs)|
|1137|[N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number)|recursion|[solution](rustgym/src/leetcode/_1137_n_th_tribonacci_number.rs)|
|1150|[Check If a Number Is Majority Element in a Sorted Array](https://leetcode.com/problems/check-if-a-number-is-majority-element-in-a-sorted-array)|array binary-search|[solution](rustgym/src/leetcode/_1150_check_if_a_number_is_majority_element_in_a_sorted_array.rs)|
|1154|[Day of the Year](https://leetcode.com/problems/day-of-the-year)|math|[solution](rustgym/src/leetcode/_1154_day_of_the_year.rs)|
|1160|[Find Words That Can Be Formed by Characters](https://leetcode.com/problems/find-words-that-can-be-formed-by-characters)|array hash-table|[solution](rustgym/src/leetcode/_1160_find_words_that_can_be_formed_by_characters.rs)|
|1165|[Single-Row Keyboard](https://leetcode.com/problems/single-row-keyboard)|string|[solution](rustgym/src/leetcode/_1165_single_row_keyboard.rs)|
|1170|[Compare Strings by Frequency of the Smallest Character](https://leetcode.com/problems/compare-strings-by-frequency-of-the-smallest-character)|array string|[solution](rustgym/src/leetcode/_1170_compare_strings_by_frequency_of_the_smallest_character.rs)|
|1175|[Prime Arrangements](https://leetcode.com/problems/prime-arrangements)|math|[solution](rustgym/src/leetcode/_1175_prime_arrangements.rs)|
|1176|[Diet Plan Performance](https://leetcode.com/problems/diet-plan-performance)|array sliding-window|[solution](rustgym/src/leetcode/_1176_diet_plan_performance.rs)|
|1180|[Count Substrings with Only One Distinct Letter](https://leetcode.com/problems/count-substrings-with-only-one-distinct-letter)|math string|[solution](rustgym/src/leetcode/_1180_count_substring_with_only_one_distinct_letter.rs)|
|1184|[Distance Between Bus Stops](https://leetcode.com/problems/distance-between-bus-stops)|array|[solution](rustgym/src/leetcode/_1184_distance_between_bus_stops.rs)|
|1185|[Day of the Week](https://leetcode.com/problems/day-of-the-week)|array|[solution](rustgym/src/leetcode/_1185_day_of_the_week.rs)|
|1189|[Maximum Number of Balloons](https://leetcode.com/problems/maximum-number-of-balloons)|hash-table string|[solution](rustgym/src/leetcode/_1189_maximum_number_of_balloons.rs)|
|118|[Pascal's Triangle](https://leetcode.com/problems/pascals-triangle)|array|[solution](rustgym/src/leetcode/_118_pascal_triangle.rs)|
|1196|[How Many Apples Can You Put into the Basket](https://leetcode.com/problems/how-many-apples-can-you-put-into-the-basket)|greedy|[solution](rustgym/src/leetcode/_1196_how_many_apples_can_you_put_into_the_basket.rs)|
|119|[Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii)|array|[solution](rustgym/src/leetcode/_119_pascal_triangle_2.rs)|
|1200|[Minimum Absolute Difference](https://leetcode.com/problems/minimum-absolute-difference)|array|[solution](rustgym/src/leetcode/_1200_minimum_absolute_difference.rs)|
|1207|[Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences)|hash-table|[solution](rustgym/src/leetcode/_1207_unique_number_of_occurrences.rs)|
|1213|[Intersection of Three Sorted Arrays](https://leetcode.com/problems/intersection-of-three-sorted-arrays)|hash-table two-pointers|[solution](rustgym/src/leetcode/_1213_intersection_of_three_sorted_arrays.rs)|
|1217|[Minimum Cost to Move Chips to The Same Position](https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position)|array math greedy|[solution](rustgym/src/leetcode/_1217_play_with_chips.rs)|
|121|[Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock)|array dynamic-programming|[solution](rustgym/src/leetcode/_121_best_time_to_buy_and_sell_stock.rs)|
|1221|[Split a String in Balanced Strings](https://leetcode.com/problems/split-a-string-in-balanced-strings)|string greedy|[solution](rustgym/src/leetcode/_1221_split_a_string_in_balanced_strings.rs)|
|1228|[Missing Number In Arithmetic Progression](https://leetcode.com/problems/missing-number-in-arithmetic-progression)|math|[solution](rustgym/src/leetcode/_1228_missing_number_in_arithmetic_progression.rs)|
|122|[Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii)|array greedy|[solution](rustgym/src/leetcode/_122_best_time_to_buy_and_sell_stock_2.rs)|
|1232|[Check If It Is a Straight Line](https://leetcode.com/problems/check-if-it-is-a-straight-line)|array math geometry|[solution](rustgym/src/leetcode/_1232_check_if_it_is_a_straight_line.rs)|
|1243|[Array Transformation](https://leetcode.com/problems/array-transformation)|array|[solution](rustgym/src/leetcode/_1243_array_transformation.rs)|
|1252|[Cells with Odd Values in a Matrix](https://leetcode.com/problems/cells-with-odd-values-in-a-matrix)|array|[solution](rustgym/src/leetcode/_1252_cells_with_odd_values_in_a_matrix.rs)|
|125|[Valid Palindrome](https://leetcode.com/problems/valid-palindrome)|two-pointers string|[solution](rustgym/src/leetcode/_125_valid_palindrome.rs)|
|1260|[Shift 2D Grid](https://leetcode.com/problems/shift-2d-grid)|array|[solution](rustgym/src/leetcode/_1260_shift_2d_grid.rs)|
|1266|[Minimum Time Visiting All Points](https://leetcode.com/problems/minimum-time-visiting-all-points)|array geometry|[solution](rustgym/src/leetcode/_1266_minimum_time_visition_all_points.rs)|
|1271|[Hexspeak](https://leetcode.com/problems/hexspeak)|math string|[solution](rustgym/src/leetcode/_1271_hexspeak.rs)|
|1275|[Find Winner on a Tic Tac Toe Game](https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game)|array|[solution](rustgym/src/leetcode/_1275_find_winner_on_a_tic_tac_toe_game.rs)|
|1281|[Subtract the Product and Sum of Digits of an Integer](https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer)|math|[solution](rustgym/src/leetcode/_1281_subtract_the_product_and_sum_of_digits_of_an_integer.rs)|
|1287|[Element Appearing More Than 25% In Sorted Array](https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array)|array|[solution](rustgym/src/leetcode/_1287_element_appearing_more_than_25_in_sorted_array.rs)|
|1290|[Convert Binary Number in a Linked List to Integer](https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer)|linked-list bit-manipulation|[solution](rustgym/src/leetcode/_1290_convert_binary_number_in_a_linked_list_to_integer.rs)|
|1295|[Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits)|array|[solution](rustgym/src/leetcode/_1295_find_numbers_with_even_number_of_digits.rs)|
|1299|[Replace Elements with Greatest Element on Right Side](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side)|array|[solution](rustgym/src/leetcode/_1299_replace_elements_with_greatest_element_on_right_side.rs)|
|1304|[Find N Unique Integers Sum up to Zero](https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero)|array|[solution](rustgym/src/leetcode/_1304_find_n_unique_integers_sum_up_to_zero.rs)|
|1309|[Decrypt String from Alphabet to Integer Mapping](https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping)|string|[solution](rustgym/src/leetcode/_1309_decrypt_string_from_alphabet_to_integer_mapping.rs)|
|1313|[Decompress Run-Length Encoded List](https://leetcode.com/problems/decompress-run-length-encoded-list)|array|[solution](rustgym/src/leetcode/_1313_decompres_run_length_encoded_list.rs)|
|1317|[Convert Integer to the Sum of Two No-Zero Integers](https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers)|math|[solution](rustgym/src/leetcode/_1317_convert_integer_to_the_sum_of_two_no_zero_integers.rs)|
|1323|[Maximum 69 Number](https://leetcode.com/problems/maximum-69-number)|math|[solution](rustgym/src/leetcode/_1323_maximum_69_number.rs)|
|1331|[Rank Transform of an Array](https://leetcode.com/problems/rank-transform-of-an-array)|array|[solution](rustgym/src/leetcode/_1331_rank_transform_of_an_array.rs)|
|1332|[Remove Palindromic Subsequences](https://leetcode.com/problems/remove-palindromic-subsequences)|string|[solution](rustgym/src/leetcode/_1332_remove_palindromic_subsequences.rs)|
|1337|[The K Weakest Rows in a Matrix](https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix)|array binary-search|[solution](rustgym/src/leetcode/_1337_the_k_weakest_rows_in_a_matrix.rs)|
|1342|[Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero)|bit-manipulation|[solution](rustgym/src/leetcode/_1342_number_of_steps_to_reduce_a_number_to_zero.rs)|
|1346|[Check If N and Its Double Exist](https://leetcode.com/problems/check-if-n-and-its-double-exist)|array|[solution](rustgym/src/leetcode/_1346_check_if_n_and_its_double_exist.rs)|
|1351|[Count Negative Numbers in a Sorted Matrix](https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix)|array binary-search|[solution](rustgym/src/leetcode/_1351_count_negative_numbers_in_a_sorted_matrix.rs)|
|1356|[Sort Integers by The Number of 1 Bits](https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits)|sort bit-manipulation|[solution](rustgym/src/leetcode/_1356_sort_integers_by_the_number_of_1_bits.rs)|
|1360|[Number of Days Between Two Dates](https://leetcode.com/problems/number-of-days-between-two-dates)||[solution](rustgym/src/leetcode/_1360_number_of_days_between_two_dates.rs)|
|1365|[How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number)|array hash-table|[solution](rustgym/src/leetcode/_1365_how_many_numbers_are_smaller_than_the_current_number.rs)|
|136|[Single Number](https://leetcode.com/problems/single-number)|hash-table bit-manipulation|[solution](rustgym/src/leetcode/_136_single_number.rs)|
|1370|[Increasing Decreasing String](https://leetcode.com/problems/increasing-decreasing-string)|string sort|[solution](rustgym/src/leetcode/_1370_increasing_decreasing_string.rs)|
|1374|[Generate a String With Characters That Have Odd Counts](https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts)|string|[solution](rustgym/src/leetcode/_1374_generate_a_string_with_characters_that_have_odd_counts.rs)|
|1380|[Lucky Numbers in a Matrix](https://leetcode.com/problems/lucky-numbers-in-a-matrix)|array|[solution](rustgym/src/leetcode/_1380_lucky_numbers_in_a_matrix.rs)|
|1385|[Find the Distance Value Between Two Arrays](https://leetcode.com/problems/find-the-distance-value-between-two-arrays)|array|[solution](rustgym/src/leetcode/_1385_find_the_distance_value_between_two_arrays.rs)|
|1389|[Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order)|array|[solution](rustgym/src/leetcode/_1389_create_target_array_in_the_given_order.rs)|
|1394|[Find Lucky Integer in an Array](https://leetcode.com/problems/find-lucky-integer-in-an-array)|array|[solution](rustgym/src/leetcode/_1394_find_lucky_integer_in_an_array.rs)|
|1399|[Count Largest Group](https://leetcode.com/problems/count-largest-group)|array|[solution](rustgym/src/leetcode/_1399_count_largest_group.rs)|
|13|[Roman to Integer](https://leetcode.com/problems/roman-to-integer)|math string|[solution](rustgym/src/leetcode/_13_roman_to_integer.rs)|
|1403|[Minimum Subsequence in Non-Increasing Order](https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order)|greedy sort|[solution](rustgym/src/leetcode/_1403_minimum_subsequence_in_non_increasing_order.rs)|
|1408|[String Matching in an Array](https://leetcode.com/problems/string-matching-in-an-array)|string|[solution](rustgym/src/leetcode/_1408_string_matching_in_an_array.rs)|
|1413|[Minimum Value to Get Positive Step by Step Sum](https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum)|array|[solution](rustgym/src/leetcode/_1413_minimum_value_to_get_positive_step_by_step_sum.rs)|
|1417|[Reformat The String](https://leetcode.com/problems/reformat-the-string)|string|[solution](rustgym/src/leetcode/_1417_reformat_the_string.rs)|
|1422|[Maximum Score After Splitting a String](https://leetcode.com/problems/maximum-score-after-splitting-a-string)|string|[solution](rustgym/src/leetcode/_1422_maximum_score_after_splitting_a_string.rs)|
|1426|[Counting Elements](https://leetcode.com/problems/counting-elements)|array|[solution](rustgym/src/leetcode/_1426_counting_elements.rs)|
|1427|[Perform String Shifts](https://leetcode.com/problems/perform-string-shifts)|array math|[solution](rustgym/src/leetcode/_1427_perform_string_shifts.rs)|
|1431|[Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies)|array|[solution](rustgym/src/leetcode/_1431_kids_with_the_greatest_number_of_candies.rs)|
|1436|[Destination City](https://leetcode.com/problems/destination-city)|string|[solution](rustgym/src/leetcode/_1436_destination_city.rs)|
|1441|[Build an Array With Stack Operations](https://leetcode.com/problems/build-an-array-with-stack-operations)|stack|[solution](rustgym/src/leetcode/_1441_build_an_array_with_stack_operations.rs)|
|1446|[Consecutive Characters](https://leetcode.com/problems/consecutive-characters)|string|[solution](rustgym/src/leetcode/_1446_consecutive_characters.rs)|
|1450|[Number of Students Doing Homework at a Given Time](https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time)|array|[solution](rustgym/src/leetcode/_1450_number_of_students_doing_homework_at_a_given_time.rs)|
|1455|[Check If a Word Occurs As a Prefix of Any Word in a Sentence](https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence)|string|[solution](rustgym/src/leetcode/_1455_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs)|
|1460|[Make Two Arrays Equal by Reversing Sub-arrays](https://leetcode.com/problems/make-two-arrays-equal-by-reversing-sub-arrays)|array|[solution](rustgym/src/leetcode/_1460_make_two_arrays_equal_by_reversing_sub_arrays.rs)|
|1464|[Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array)|array|[solution](rustgym/src/leetcode/_1464_maximum_product_of_two_elements_in_an_array.rs)|
|1469|[Find All The Lonely Nodes](https://leetcode.com/problems/find-all-the-lonely-nodes)|tree depth-first-search|[solution](rustgym/src/leetcode/_1469_find_all_the_lonely_nodes.rs)|
|1470|[Shuffle the Array](https://leetcode.com/problems/shuffle-the-array)|array|[solution](rustgym/src/leetcode/_1470_shuffle_the_array.rs)|
|1474|[Delete N Nodes After M Nodes of a Linked List](https://leetcode.com/problems/delete-n-nodes-after-m-nodes-of-a-linked-list)|linked-list|[solution](rustgym/src/leetcode/_1474_delete_n_nodes_after_m_nodes_of_a_linked_list.rs)|
|1475|[Final Prices With a Special Discount in a Shop](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop)|array|[solution](rustgym/src/leetcode/_1475_final_prices_with_a_speial_discount_in_shop.rs)|
|1480|[Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array)|array|[solution](rustgym/src/leetcode/_1480_running_sum_of_1d_array.rs)|
|1486|[XOR Operation in an Array](https://leetcode.com/problems/xor-operation-in-an-array)|array bit-manipulation|[solution](rustgym/src/leetcode/_1486_xor_operation_in_an_array.rs)|
|1491|[Average Salary Excluding the Minimum and Maximum Salary](https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary)|array sort|[solution](rustgym/src/leetcode/_1491_average_salary_excluding_minimum_and_maximum_salary.rs)|
|1496|[Path Crossing](https://leetcode.com/problems/path-crossing)|string|[solution](rustgym/src/leetcode/_1496_path_crossing.rs)|
|14|[Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix)|string|[solution](rustgym/src/leetcode/_14_longest_common_prefix.rs)|
|1502|[Can Make Arithmetic Progression From Sequence](https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence)|array sort|[solution](rustgym/src/leetcode/_1502_can_make_arithmetic_progression_from_sequence.rs)|
|1507|[Reformat Date](https://leetcode.com/problems/reformat-date)|string|[solution](rustgym/src/leetcode/_1507_reformat_date.rs)|
|1512|[Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs)|array hash-table math|[solution](rustgym/src/leetcode/_1512_number_of_good_pairs.rs)|
|1518|[Water Bottles](https://leetcode.com/problems/water-bottles)|greedy|[solution](rustgym/src/leetcode/_1518_water_bottles.rs)|
|1523|[Count Odd Numbers in an Interval Range](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range)|math|[solution](rustgym/src/leetcode/_1523_cound_odd_numbers_in_interval_range.rs)|
|1528|[Shuffle String](https://leetcode.com/problems/shuffle-string)|sort|[solution](rustgym/src/leetcode/_1528_shuffle_string.rs)|
|1534|[Count Good Triplets](https://leetcode.com/problems/count-good-triplets)|array|[solution](rustgym/src/leetcode/_1534_count_good_triplets.rs)|
|1539|[Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number)|array hash-table|[solution](rustgym/src/leetcode/_1539_kth_missing_positive_number.rs)|
|1544|[Make The String Great](https://leetcode.com/problems/make-the-string-great)|string stack|[solution](rustgym/src/leetcode/_1544_make_the_string_great.rs)|
|1550|[Three Consecutive Odds](https://leetcode.com/problems/three-consecutive-odds)|array|[solution](rustgym/src/leetcode/_1550_three_consecutive_odds.rs)|
|155|[Min Stack](https://leetcode.com/problems/min-stack)|stack design|[solution](rustgym/src/leetcode/_155_min_stack.rs)|
|167|[Two Sum II - Input array is sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted)|array two-pointers binary-search|[solution](rustgym/src/leetcode/_167_two_sum_2.rs)|
|168|[Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title)|math|[solution](rustgym/src/leetcode/_168_excel_sheet_column_title.rs)|
|169|[Majority Element](https://leetcode.com/problems/majority-element)|array divide-and-conquer bit-manipulation|[solution](rustgym/src/leetcode/_169_majority_element.rs)|
|170|[Two Sum III - Data structure design](https://leetcode.com/problems/two-sum-iii-data-structure-design)|hash-table design|[solution](rustgym/src/leetcode/_170_two_sum_3.rs)|
|171|[Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number)|math|[solution](rustgym/src/leetcode/_171_excel_sheet_column_number.rs)|
|172|[Factorial Trailing Zeroes](https://leetcode.com/problems/factorial-trailing-zeroes)|math|[solution](rustgym/src/leetcode/_172_factorial_trailing_zeroes.rs)|
|189|[Rotate Array](https://leetcode.com/problems/rotate-array)|array|[solution](rustgym/src/leetcode/_189_rotate_array.rs)|
|191|[Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits)|bit-manipulation|[solution](rustgym/src/leetcode/_191_number_of_1_bits.rs)|
|198|[House Robber](https://leetcode.com/problems/house-robber)|dynamic-programming|[solution](rustgym/src/leetcode/_198_house_robber.rs)|
|1|[Two Sum](https://leetcode.com/problems/two-sum)|array hash-table|[solution](rustgym/src/leetcode/_1_two_sum.rs)|
|202|[Happy Number](https://leetcode.com/problems/happy-number)|hash-table math|[solution](rustgym/src/leetcode/_202_happy_number.rs)|
|203|[Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements)|linked-list|[solution](rustgym/src/leetcode/_203_remove_linked_list_elements.rs)|
|204|[Count Primes](https://leetcode.com/problems/count-primes)|hash-table math|[solution](rustgym/src/leetcode/_204_count_primes.rs)|
|205|[Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings)|hash-table|[solution](rustgym/src/leetcode/_205_isomorphic_strings.rs)|
|206|[Reverse Linked List](https://leetcode.com/problems/reverse-linked-list)|linked-list|[solution](rustgym/src/leetcode/_206_reverse_linked_list.rs)|
|20|[Valid Parentheses](https://leetcode.com/problems/valid-parentheses)|string stack|[solution](rustgym/src/leetcode/_20_valid_parentheses.rs)|
|217|[Contains Duplicate](https://leetcode.com/problems/contains-duplicate)|array hash-table|[solution](rustgym/src/leetcode/_217_contains_duplicate.rs)|
|219|[Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii)|array hash-table|[solution](rustgym/src/leetcode/_219_contains_duplicate_2.rs)|
|21|[Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists)|linked-list|[solution](rustgym/src/leetcode/_21_merge_two_sorted_lists.rs)|
|225|[Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues)|stack design|[solution](rustgym/src/leetcode/_225_implement_stack_using_queues.rs)|
|226|[Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree)|tree|[solution](rustgym/src/leetcode/_226_invert_binary_tree.rs)|
|231|[Power of Two](https://leetcode.com/problems/power-of-two)|math bit-manipulation|[solution](rustgym/src/leetcode/_231_power_of_two.rs)|
|232|[Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks)|stack design|[solution](rustgym/src/leetcode/_232_implent_queue_using_stacks.rs)|
|234|[Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list)|linked-list two-pointers|[solution](rustgym/src/leetcode/_234_palindrome_linked_list.rs)|
|242|[Valid Anagram](https://leetcode.com/problems/valid-anagram)|hash-table sort|[solution](rustgym/src/leetcode/_242_valid_anagram.rs)|
|243|[Shortest Word Distance](https://leetcode.com/problems/shortest-word-distance)|array|[solution](rustgym/src/leetcode/_243_shortest_word_distance.rs)|
|246|[Strobogrammatic Number](https://leetcode.com/problems/strobogrammatic-number)|hash-table math|[solution](rustgym/src/leetcode/_246_strobogrammantic_number.rs)|
|252|[Meeting Rooms](https://leetcode.com/problems/meeting-rooms)|sort|[solution](rustgym/src/leetcode/_252_meeting_rooms.rs)|
|256|[Paint House](https://leetcode.com/problems/paint-house)|dynamic-programming|[solution](rustgym/src/leetcode/_256_paint_house.rs)|
|257|[Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths)|tree depth-first-search|[solution](rustgym/src/leetcode/_257_binary_tree_paths.rs)|
|258|[Add Digits](https://leetcode.com/problems/add-digits)|math|[solution](rustgym/src/leetcode/_258_add_digits.rs)|
|263|[Ugly Number](https://leetcode.com/problems/ugly-number)|math|[solution](rustgym/src/leetcode/_263_ugly_number.rs)|
|266|[Palindrome Permutation](https://leetcode.com/problems/palindrome-permutation)|hash-table|[solution](rustgym/src/leetcode/_266_palindrome_permutation.rs)|
|268|[Missing Number](https://leetcode.com/problems/missing-number)|array math bit-manipulation|[solution](rustgym/src/leetcode/_268_missing_number.rs)|
|26|[Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array)|array two-pointers|[solution](rustgym/src/leetcode/_26_remove_duplicates_from_sorted_array.rs)|
|270|[Closest Binary Search Tree Value](https://leetcode.com/problems/closest-binary-search-tree-value)|binary-search tree|[solution](rustgym/src/leetcode/_270_closest_binary_search_tree_value.rs)|
|276|[Paint Fence](https://leetcode.com/problems/paint-fence)|dynamic-programming|[solution](rustgym/src/leetcode/_276_paint_fence.rs)|
|278|[First Bad Version](https://leetcode.com/problems/first-bad-version)|binary-search|[solution](rustgym/src/leetcode/_278_first_bad_version.rs)|
|27|[Remove Element](https://leetcode.com/problems/remove-element)|array two-pointers|[solution](rustgym/src/leetcode/_27_remove_element.rs)|
|283|[Move Zeroes](https://leetcode.com/problems/move-zeroes)|array two-pointers|[solution](rustgym/src/leetcode/_283_move_zeros.rs)|
|28|[Implement strStr()](https://leetcode.com/problems/implement-strstr)|two-pointers string|[solution](rustgym/src/leetcode/_28_implement_str_str.rs)|
|290|[Word Pattern](https://leetcode.com/problems/word-pattern)|hash-table|[solution](rustgym/src/leetcode/_290_word_pattern.rs)|
|292|[Nim Game](https://leetcode.com/problems/nim-game)|brainteaser minimax|[solution](rustgym/src/leetcode/_292_nim_game.rs)|
|293|[Flip Game](https://leetcode.com/problems/flip-game)|string|[solution](rustgym/src/leetcode/_293_flip_game.rs)|
|299|[Bulls and Cows](https://leetcode.com/problems/bulls-and-cows)|hash-table|[solution](rustgym/src/leetcode/_299_bulls_and_cows.rs)|
|303|[Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable)|dynamic-programming|[solution](rustgym/src/leetcode/_303_range_sum_query.rs)|
|326|[Power of Three](https://leetcode.com/problems/power-of-three)|math|[solution](rustgym/src/leetcode/_326_power_of_three.rs)|
|339|[Nested List Weight Sum](https://leetcode.com/problems/nested-list-weight-sum)|depth-first-search|[solution](rustgym/src/leetcode/_339_nested_list_weight_sum.rs)|
|342|[Power of Four](https://leetcode.com/problems/power-of-four)|bit-manipulation|[solution](rustgym/src/leetcode/_342_power_of_four.rs)|
|344|[Reverse String](https://leetcode.com/problems/reverse-string)|two-pointers string|[solution](rustgym/src/leetcode/_344_reverse_string.rs)|
|345|[Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string)|two-pointers string|[solution](rustgym/src/leetcode/_345_reverse_vowels_of_a_string.rs)|
|346|[Moving Average from Data Stream](https://leetcode.com/problems/moving-average-from-data-stream)|design queue|[solution](rustgym/src/leetcode/_346_moving_average_from_data_stream.rs)|
|349|[Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays)|hash-table two-pointers binary-search sort|[solution](rustgym/src/leetcode/_349_intersection_of_two_arrays.rs)|
|350|[Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii)|hash-table two-pointers binary-search sort|[solution](rustgym/src/leetcode/_350_intersection_of_two_arrays_2.rs)|
|359|[Logger Rate Limiter](https://leetcode.com/problems/logger-rate-limiter)|hash-table design|[solution](rustgym/src/leetcode/_359_logger_rate_limiter.rs)|
|35|[Search Insert Position](https://leetcode.com/problems/search-insert-position)|array binary-search|[solution](rustgym/src/leetcode/_35_search_insert_position.rs)|
|367|[Valid Perfect Square](https://leetcode.com/problems/valid-perfect-square)|math binary-search|[solution](rustgym/src/leetcode/_367_valid_perfect_square.rs)|
|374|[Guess Number Higher or Lower](https://leetcode.com/problems/guess-number-higher-or-lower)|binary-search|[solution](rustgym/src/leetcode/_374_guess_number_higher_or_lower.rs)|
|383|[Ransom Note](https://leetcode.com/problems/ransom-note)|string|[solution](rustgym/src/leetcode/_383_ransom_note.rs)|
|387|[First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string)|hash-table string|[solution](rustgym/src/leetcode/_387_first_unique_character_in_a_string.rs)|
|389|[Find the Difference](https://leetcode.com/problems/find-the-difference)|hash-table bit-manipulation|[solution](rustgym/src/leetcode/_389_find_the_difference.rs)|
|38|[Count and Say](https://leetcode.com/problems/count-and-say)|string|[solution](rustgym/src/leetcode/_38_count_and_say.rs)|
|392|[Is Subsequence](https://leetcode.com/problems/is-subsequence)|binary-search dynamic-programming greedy|[solution](rustgym/src/leetcode/_392_is_subsequence.rs)|
|401|[Binary Watch](https://leetcode.com/problems/binary-watch)|backtracking bit-manipulation|[solution](rustgym/src/leetcode/_401_binary_watch.rs)|
|404|[Sum of Left Leaves](https://leetcode.com/problems/sum-of-left-leaves)|tree|[solution](rustgym/src/leetcode/_404_sum_of_left_leaves.rs)|
|405|[Convert a Number to Hexadecimal](https://leetcode.com/problems/convert-a-number-to-hexadecimal)|bit-manipulation|[solution](rustgym/src/leetcode/_405_convert_a_number_to_hexadecimal.rs)|
|408|[Valid Word Abbreviation](https://leetcode.com/problems/valid-word-abbreviation)|string|[solution](rustgym/src/leetcode/_408_valid_word_abbreviation.rs)|
|409|[Longest Palindrome](https://leetcode.com/problems/longest-palindrome)|hash-table|[solution](rustgym/src/leetcode/_409_longest_palindrome.rs)|
|412|[Fizz Buzz](https://leetcode.com/problems/fizz-buzz)||[solution](rustgym/src/leetcode/_412_fizz_buzz.rs)|
|414|[Third Maximum Number](https://leetcode.com/problems/third-maximum-number)|array|[solution](rustgym/src/leetcode/_414_third_maximum_number.rs)|
|415|[Add Strings](https://leetcode.com/problems/add-strings)|string|[solution](rustgym/src/leetcode/_415_add_strings.rs)|
|422|[Valid Word Square](https://leetcode.com/problems/valid-word-square)||[solution](rustgym/src/leetcode/_422_valid_word_square.rs)|
|434|[Number of Segments in a String](https://leetcode.com/problems/number-of-segments-in-a-string)|string|[solution](rustgym/src/leetcode/_434_number_of_segments_in_a_string.rs)|
|441|[Arranging Coins](https://leetcode.com/problems/arranging-coins)|math binary-search|[solution](rustgym/src/leetcode/_441_arranging_coins.rs)|
|443|[String Compression](https://leetcode.com/problems/string-compression)|string|[solution](rustgym/src/leetcode/_443_string_compression.rs)|
|447|[Number of Boomerangs](https://leetcode.com/problems/number-of-boomerangs)|hash-table|[solution](rustgym/src/leetcode/_447_number_of_boomerangs.rs)|
|448|[Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array)|array|[solution](rustgym/src/leetcode/_448_find_all_numbers_disappeared_in_an_array.rs)|
|453|[Minimum Moves to Equal Array Elements](https://leetcode.com/problems/minimum-moves-to-equal-array-elements)|math|[solution](rustgym/src/leetcode/_453_minimum_moves_to_equal_array_elements.rs)|
|455|[Assign Cookies](https://leetcode.com/problems/assign-cookies)|greedy|[solution](rustgym/src/leetcode/_455_assign_cookies.rs)|
|459|[Repeated Substring Pattern](https://leetcode.com/problems/repeated-substring-pattern)|string|[solution](rustgym/src/leetcode/_459_repeated_substring_pattern.rs)|
|461|[Hamming Distance](https://leetcode.com/problems/hamming-distance)|bit-manipulation|[solution](rustgym/src/leetcode/_461_hamming_distance.rs)|
|463|[Island Perimeter](https://leetcode.com/problems/island-perimeter)|hash-table|[solution](rustgym/src/leetcode/_463_island_perimeter.rs)|
|475|[Heaters](https://leetcode.com/problems/heaters)|binary-search|[solution](rustgym/src/leetcode/_475_heaters.rs)|
|476|[Number Complement](https://leetcode.com/problems/number-complement)|bit-manipulation|[solution](rustgym/src/leetcode/_476_number_complement.rs)|
|482|[License Key Formatting](https://leetcode.com/problems/license-key-formatting)||[solution](rustgym/src/leetcode/_482_license_key_formatting.rs)|
|485|[Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones)|array|[solution](rustgym/src/leetcode/_485_max_consecutive_ones.rs)|
|492|[Construct the Rectangle](https://leetcode.com/problems/construct-the-rectangle)||[solution](rustgym/src/leetcode/_492_construct_the_rectangle.rs)|
|496|[Next Greater Element I](https://leetcode.com/problems/next-greater-element-i)|stack|[solution](rustgym/src/leetcode/_496_next_greater_element_1.rs)|
|500|[Keyboard Row](https://leetcode.com/problems/keyboard-row)|hash-table|[solution](rustgym/src/leetcode/_500_keyboard_row.rs)|
|501|[Find Mode in Binary Search Tree](https://leetcode.com/problems/find-mode-in-binary-search-tree)|tree|[solution](rustgym/src/leetcode/_501_find_mode_in_binary_search_tree.rs)|
|504|[Base 7](https://leetcode.com/problems/base-7)||[solution](rustgym/src/leetcode/_504_base_7.rs)|
|506|[Relative Ranks](https://leetcode.com/problems/relative-ranks)||[solution](rustgym/src/leetcode/_506_relative_ranks.rs)|
|507|[Perfect Number](https://leetcode.com/problems/perfect-number)|math|[solution](rustgym/src/leetcode/_507_perfect_number.rs)|
|509|[Fibonacci Number](https://leetcode.com/problems/fibonacci-number)|array|[solution](rustgym/src/leetcode/_509_fibonacci_number.rs)|
|520|[Detect Capital](https://leetcode.com/problems/detect-capital)|string|[solution](rustgym/src/leetcode/_520_detect_captial.rs)|
|521|[Longest Uncommon Subsequence I ](https://leetcode.com/problems/longest-uncommon-subsequence-i)|string|[solution](rustgym/src/leetcode/_521_longest_uncommon_subsequence_1.rs)|
|530|[Minimum Absolute Difference in BST](https://leetcode.com/problems/minimum-absolute-difference-in-bst)|tree|[solution](rustgym/src/leetcode/_530_minimum_absolute_difference_in_bst.rs)|
|532|[K-diff Pairs in an Array](https://leetcode.com/problems/k-diff-pairs-in-an-array)|array two-pointers|[solution](rustgym/src/leetcode/_532_k_diff_pairs_in_an_array.rs)|
|538|[Convert BST to Greater Tree](https://leetcode.com/problems/convert-bst-to-greater-tree)|tree|[solution](rustgym/src/leetcode/_538_convert_bst_to_greater_tree.rs)|
|53|[Maximum Subarray](https://leetcode.com/problems/maximum-subarray)|array divide-and-conquer dynamic-programming|[solution](rustgym/src/leetcode/_53_maximum_subarray.rs)|
|541|[Reverse String II](https://leetcode.com/problems/reverse-string-ii)|string|[solution](rustgym/src/leetcode/_541_reverse_string_2.rs)|
|543|[Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree)|tree|[solution](rustgym/src/leetcode/_543_diameter_of_binary_tree.rs)|
|551|[Student Attendance Record I](https://leetcode.com/problems/student-attendance-record-i)|string|[solution](rustgym/src/leetcode/_551_student_attendance_record_1.rs)|
|557|[Reverse Words in a String III](https://leetcode.com/problems/reverse-words-in-a-string-iii)|string|[solution](rustgym/src/leetcode/_557_reverse_words_in_a_string_3.rs)|
|561|[Array Partition I](https://leetcode.com/problems/array-partition-i)|array|[solution](rustgym/src/leetcode/_561_array_partition_1.rs)|
|563|[Binary Tree Tilt](https://leetcode.com/problems/binary-tree-tilt)|tree|[solution](rustgym/src/leetcode/_563_binary_tree_tilt.rs)|
|566|[Reshape the Matrix](https://leetcode.com/problems/reshape-the-matrix)|array|[solution](rustgym/src/leetcode/_566_reshape_the_matrix.rs)|
|572|[Subtree of Another Tree](https://leetcode.com/problems/subtree-of-another-tree)|tree|[solution](rustgym/src/leetcode/_572_subtree_of_another_tree.rs)|
|575|[Distribute Candies](https://leetcode.com/problems/distribute-candies)|hash-table|[solution](rustgym/src/leetcode/_575_distribute_candies.rs)|
|581|[Shortest Unsorted Continuous Subarray](https://leetcode.com/problems/shortest-unsorted-continuous-subarray)|array|[solution](rustgym/src/leetcode/_581_shortest_unsorted_continuous_subarray.rs)|
|58|[Length of Last Word](https://leetcode.com/problems/length-of-last-word)|string|[solution](rustgym/src/leetcode/_58_length_of_last_word.rs)|
|594|[Longest Harmonious Subsequence](https://leetcode.com/problems/longest-harmonious-subsequence)|hash-table|[solution](rustgym/src/leetcode/_594_longest_harmonious_subsequence.rs)|
|598|[Range Addition II](https://leetcode.com/problems/range-addition-ii)|math|[solution](rustgym/src/leetcode/_598_range_addition_2.rs)|
|599|[Minimum Index Sum of Two Lists](https://leetcode.com/problems/minimum-index-sum-of-two-lists)|hash-table|[solution](rustgym/src/leetcode/_599_minimum_index_sum_of_two_lists.rs)|
|604|[Design Compressed String Iterator](https://leetcode.com/problems/design-compressed-string-iterator)|design|[solution](rustgym/src/leetcode/_604_design_compressed_string_iterator.rs)|
|605|[Can Place Flowers](https://leetcode.com/problems/can-place-flowers)|array|[solution](rustgym/src/leetcode/_605_can_place_flowers.rs)|
|606|[Construct String from Binary Tree](https://leetcode.com/problems/construct-string-from-binary-tree)|string tree|[solution](rustgym/src/leetcode/_606_construct_string_from_binary_tree.rs)|
|617|[Merge Two Binary Trees](https://leetcode.com/problems/merge-two-binary-trees)|tree|[solution](rustgym/src/leetcode/_617_merge_two_binary_trees.rs)|
|624|[Maximum Distance in Arrays](https://leetcode.com/problems/maximum-distance-in-arrays)|array hash-table|[solution](rustgym/src/leetcode/_624_maximum_distance_in_arrays.rs)|
|628|[Maximum Product of Three Numbers](https://leetcode.com/problems/maximum-product-of-three-numbers)|array math|[solution](rustgym/src/leetcode/_628_maximum_product_of_three_numbers.rs)|
|633|[Sum of Square Numbers](https://leetcode.com/problems/sum-of-square-numbers)|math|[solution](rustgym/src/leetcode/_633_sum_of_square_numbers.rs)|
|637|[Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree)|tree|[solution](rustgym/src/leetcode/_637_average_of_levels_in_binary_tree.rs)|
|643|[Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i)|array|[solution](rustgym/src/leetcode/_643_maximum_average_subarray_1.rs)|
|645|[Set Mismatch](https://leetcode.com/problems/set-mismatch)|hash-table math|[solution](rustgym/src/leetcode/_645_set_mismatch.rs)|
|653|[Two Sum IV - Input is a BST](https://leetcode.com/problems/two-sum-iv-input-is-a-bst)|tree|[solution](rustgym/src/leetcode/_653_two_sum_4.rs)|
|657|[Robot Return to Origin](https://leetcode.com/problems/robot-return-to-origin)|string|[solution](rustgym/src/leetcode/_657_robot_return_to_origin.rs)|
|661|[Image Smoother](https://leetcode.com/problems/image-smoother)|array|[solution](rustgym/src/leetcode/_661_image_smoother.rs)|
|665|[Non-decreasing Array](https://leetcode.com/problems/non-decreasing-array)|array|[solution](rustgym/src/leetcode/_665_non_decreasing_array.rs)|
|669|[Trim a Binary Search Tree](https://leetcode.com/problems/trim-a-binary-search-tree)|tree|[solution](rustgym/src/leetcode/_669_trim_a_binary_search_tree.rs)|
|66|[Plus One](https://leetcode.com/problems/plus-one)|array|[solution](rustgym/src/leetcode/_66_plus_one.rs)|
|671|[Second Minimum Node In a Binary Tree](https://leetcode.com/problems/second-minimum-node-in-a-binary-tree)|tree|[solution](rustgym/src/leetcode/_671_second_minimum_node_in_a_binary_tree.rs)|
|674|[Longest Continuous Increasing Subsequence](https://leetcode.com/problems/longest-continuous-increasing-subsequence)|array|[solution](rustgym/src/leetcode/_674_longest_continuous_increasing_subsequence.rs)|
|67|[Add Binary](https://leetcode.com/problems/add-binary)|math string|[solution](rustgym/src/leetcode/_67_add_binary.rs)|
|680|[Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii)|string|[solution](rustgym/src/leetcode/_680_valid_palindrome_2.rs)|
|682|[Baseball Game](https://leetcode.com/problems/baseball-game)|stack|[solution](rustgym/src/leetcode/_682_baseball_game.rs)|
|686|[Repeated String Match](https://leetcode.com/problems/repeated-string-match)|string|[solution](rustgym/src/leetcode/_686_repeated_string_match.rs)|
|687|[Longest Univalue Path](https://leetcode.com/problems/longest-univalue-path)|tree recursion|[solution](rustgym/src/leetcode/_687_longest_univalue_path.rs)|
|693|[Binary Number with Alternating Bits](https://leetcode.com/problems/binary-number-with-alternating-bits)|bit-manipulation|[solution](rustgym/src/leetcode/_693_binary_number_with_alternating_bits.rs)|
|696|[Count Binary Substrings](https://leetcode.com/problems/count-binary-substrings)|string|[solution](rustgym/src/leetcode/_696_count_binary_substrings.rs)|
|697|[Degree of an Array](https://leetcode.com/problems/degree-of-an-array)|array|[solution](rustgym/src/leetcode/_697_degree_of_an_array.rs)|
|69|[Sqrt(x)](https://leetcode.com/problems/sqrtx)|math binary-search|[solution](rustgym/src/leetcode/_69_sqrt.rs)|
|700|[Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree)|tree|[solution](rustgym/src/leetcode/_700_search_in_a_binary_search_tree.rs)|
|703|[Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream)|heap|[solution](rustgym/src/leetcode/_703_kth_largest_element_in_a_stream.rs)|
|704|[Binary Search](https://leetcode.com/problems/binary-search)|binary-search|[solution](rustgym/src/leetcode/_704_binary_search.rs)|
|705|[Design HashSet](https://leetcode.com/problems/design-hashset)|hash-table design|[solution](rustgym/src/leetcode/_705_design_hash_set.rs)|
|706|[Design HashMap](https://leetcode.com/problems/design-hashmap)|hash-table design|[solution](rustgym/src/leetcode/_706_design_hash_map.rs)|
|709|[To Lower Case](https://leetcode.com/problems/to-lower-case)|string|[solution](rustgym/src/leetcode/_709_to_lower_case.rs)|
|70|[Climbing Stairs](https://leetcode.com/problems/climbing-stairs)|dynamic-programming|[solution](rustgym/src/leetcode/_70_climbing_stairs.rs)|
|716|[Max Stack](https://leetcode.com/problems/max-stack)|design|[solution](rustgym/src/leetcode/_716_max_stack.rs)|
|717|[1-bit and 2-bit Characters](https://leetcode.com/problems/1-bit-and-2-bit-characters)|array|[solution](rustgym/src/leetcode/_717_1bit_and_2bit_characters.rs)|
|720|[Longest Word in Dictionary](https://leetcode.com/problems/longest-word-in-dictionary)|hash-table trie|[solution](rustgym/src/leetcode/_720_longest_word_in_dictionary.rs)|
|724|[Find Pivot Index](https://leetcode.com/problems/find-pivot-index)|array|[solution](rustgym/src/leetcode/_724_find_pivot_index.rs)|
|728|[Self Dividing Numbers](https://leetcode.com/problems/self-dividing-numbers)|math|[solution](rustgym/src/leetcode/_728_self_dividing_numbers.rs)|
|733|[Flood Fill](https://leetcode.com/problems/flood-fill)|depth-first-search|[solution](rustgym/src/leetcode/_733_flood_fill.rs)|
|734|[Sentence Similarity](https://leetcode.com/problems/sentence-similarity)|hash-table|[solution](rustgym/src/leetcode/_734_sentence_similarity.rs)|
|744|[Find Smallest Letter Greater Than Target](https://leetcode.com/problems/find-smallest-letter-greater-than-target)|binary-search|[solution](rustgym/src/leetcode/_744_find_smallest_letter_greater_than_target.rs)|
|746|[Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs)|array dynamic-programming|[solution](rustgym/src/leetcode/_746_min_cost_climbing_stairs.rs)|
|747|[Largest Number At Least Twice of Others](https://leetcode.com/problems/largest-number-at-least-twice-of-others)|array|[solution](rustgym/src/leetcode/_747_largest_number_at_least_twice_of_others.rs)|
|748|[Shortest Completing Word](https://leetcode.com/problems/shortest-completing-word)|hash-table|[solution](rustgym/src/leetcode/_748_shortest_completing_word.rs)|
|758|[Bold Words in String](https://leetcode.com/problems/bold-words-in-string)|string|[solution](rustgym/src/leetcode/_758_bold_words_in_string.rs)|
|760|[Find Anagram Mappings](https://leetcode.com/problems/find-anagram-mappings)|hash-table|[solution](rustgym/src/leetcode/_760_find_anagram_mappings.rs)|
|762|[Prime Number of Set Bits in Binary Representation](https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation)|bit-manipulation|[solution](rustgym/src/leetcode/_762_prime_number_of_set_bits_in_binary_representation.rs)|
|766|[Toeplitz Matrix](https://leetcode.com/problems/toeplitz-matrix)|array|[solution](rustgym/src/leetcode/_766_toeplitiz_matrix.rs)|
|771|[Jewels and Stones](https://leetcode.com/problems/jewels-and-stones)|hash-table|[solution](rustgym/src/leetcode/_771_jewels_and_stones.rs)|
|783|[Minimum Distance Between BST Nodes](https://leetcode.com/problems/minimum-distance-between-bst-nodes)|tree recursion|[solution](rustgym/src/leetcode/_783_minimum_distance_between_bst_nodes.rs)|
|788|[Rotated Digits](https://leetcode.com/problems/rotated-digits)|string|[solution](rustgym/src/leetcode/_788_rotated_digits.rs)|
|796|[Rotate String](https://leetcode.com/problems/rotate-string)||[solution](rustgym/src/leetcode/_796_rotate_string.rs)|
|7|[Reverse Integer](https://leetcode.com/problems/reverse-integer)|math|[solution](rustgym/src/leetcode/_7_reverse_integer.rs)|
|800|[Similar RGB Color](https://leetcode.com/problems/similar-rgb-color)|math string|[solution](rustgym/src/leetcode/_800_similar_rgb_color.rs)|
|804|[Unique Morse Code Words](https://leetcode.com/problems/unique-morse-code-words)|string|[solution](rustgym/src/leetcode/_804_unique_morse_code_words.rs)|
|806|[Number of Lines To Write String](https://leetcode.com/problems/number-of-lines-to-write-string)||[solution](rustgym/src/leetcode/_806_number_of_lines_to_write_string.rs)|
|811|[Subdomain Visit Count](https://leetcode.com/problems/subdomain-visit-count)|hash-table|[solution](rustgym/src/leetcode/_811_subdomain_visit_count.rs)|
|812|[Largest Triangle Area](https://leetcode.com/problems/largest-triangle-area)|math|[solution](rustgym/src/leetcode/_812_largest_triangle_area.rs)|
|819|[Most Common Word](https://leetcode.com/problems/most-common-word)|string|[solution](rustgym/src/leetcode/_819_most_common_word.rs)|
|821|[Shortest Distance to a Character](https://leetcode.com/problems/shortest-distance-to-a-character)||[solution](rustgym/src/leetcode/_821_shortest_distance_to_a_character.rs)|
|824|[Goat Latin](https://leetcode.com/problems/goat-latin)|string|[solution](rustgym/src/leetcode/_824_goat_latin.rs)|
|830|[Positions of Large Groups](https://leetcode.com/problems/positions-of-large-groups)|array|[solution](rustgym/src/leetcode/_830_positions_of_large_groups.rs)|
|832|[Flipping an Image](https://leetcode.com/problems/flipping-an-image)|array|[solution](rustgym/src/leetcode/_832_flipping_an_image.rs)|
|836|[Rectangle Overlap](https://leetcode.com/problems/rectangle-overlap)|math|[solution](rustgym/src/leetcode/_836_rectangle_overlap.rs)|
|83|[Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list)|linked-list|[solution](rustgym/src/leetcode/_83_remove_duplicates_from_sorted_list.rs)|
|840|[Magic Squares In Grid](https://leetcode.com/problems/magic-squares-in-grid)|array|[solution](rustgym/src/leetcode/_840_magic_squares_in_grid.rs)|
|844|[Backspace String Compare](https://leetcode.com/problems/backspace-string-compare)|two-pointers stack|[solution](rustgym/src/leetcode/_844_backspace_string_compare.rs)|
|849|[Maximize Distance to Closest Person](https://leetcode.com/problems/maximize-distance-to-closest-person)|array|[solution](rustgym/src/leetcode/_849_maximize_distance_to_closest_person.rs)|
|852|[Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array)|binary-search|[solution](rustgym/src/leetcode/_852_peak_index_in_a_mountain_array.rs)|
|859|[Buddy Strings](https://leetcode.com/problems/buddy-strings)|string|[solution](rustgym/src/leetcode/_859_buddy_strings.rs)|
|860|[Lemonade Change](https://leetcode.com/problems/lemonade-change)|greedy|[solution](rustgym/src/leetcode/_860_lemonade_change.rs)|
|867|[Transpose Matrix](https://leetcode.com/problems/transpose-matrix)|array|[solution](rustgym/src/leetcode/_867_transpose_matrix.rs)|
|868|[Binary Gap](https://leetcode.com/problems/binary-gap)|math|[solution](rustgym/src/leetcode/_868_binary_gap.rs)|
|872|[Leaf-Similar Trees](https://leetcode.com/problems/leaf-similar-trees)|tree depth-first-search|[solution](rustgym/src/leetcode/_872_leaf_similar_trees.rs)|
|874|[Walking Robot Simulation](https://leetcode.com/problems/walking-robot-simulation)|greedy|[solution](rustgym/src/leetcode/_874_walking_robot_simulation.rs)|
|876|[Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list)|linked-list|[solution](rustgym/src/leetcode/_876_middle_of_the_linked_list.rs)|
|883|[Projection Area of 3D Shapes](https://leetcode.com/problems/projection-area-of-3d-shapes)|math|[solution](rustgym/src/leetcode/_883_projection_area_of_3d_shapes.rs)|
|884|[Uncommon Words from Two Sentences](https://leetcode.com/problems/uncommon-words-from-two-sentences)|hash-table|[solution](rustgym/src/leetcode/_884_uncommon_words_from_two_sentences.rs)|
|888|[Fair Candy Swap](https://leetcode.com/problems/fair-candy-swap)|array|[solution](rustgym/src/leetcode/_888_fair_candy_swap.rs)|
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array)|array two-pointers|[solution](rustgym/src/leetcode/_88_merge_sorted_array.rs)|
|892|[Surface Area of 3D Shapes](https://leetcode.com/problems/surface-area-of-3d-shapes)|math geometry|[solution](rustgym/src/leetcode/_892_surface_area_of_3d_shapes.rs)|
|893|[Groups of Special-Equivalent Strings](https://leetcode.com/problems/groups-of-special-equivalent-strings)|string|[solution](rustgym/src/leetcode/_893_groups_of_special_equivalent_string.rs)|
|896|[Monotonic Array](https://leetcode.com/problems/monotonic-array)|array|[solution](rustgym/src/leetcode/_896_monotonic_array.rs)|
|897|[Increasing Order Search Tree](https://leetcode.com/problems/increasing-order-search-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_897_increasing_order_search_tree.rs)|
|905|[Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity)|array|[solution](rustgym/src/leetcode/_905_sort_array_by_parity.rs)|
|908|[Smallest Range I](https://leetcode.com/problems/smallest-range-i)|math|[solution](rustgym/src/leetcode/_908_smallest_range_1.rs)|
|914|[X of a Kind in a Deck of Cards](https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards)|array math|[solution](rustgym/src/leetcode/_914_x_of_a_kind_in_a_deck_of_cards.rs)|
|917|[Reverse Only Letters](https://leetcode.com/problems/reverse-only-letters)|string|[solution](rustgym/src/leetcode/_917_reverse_only_letters.rs)|
|922|[Sort Array By Parity II](https://leetcode.com/problems/sort-array-by-parity-ii)|array sort|[solution](rustgym/src/leetcode/_922_sort_array_by_parity_2.rs)|
|925|[Long Pressed Name](https://leetcode.com/problems/long-pressed-name)|two-pointers string|[solution](rustgym/src/leetcode/_925_long_pressed_name.rs)|
|929|[Unique Email Addresses](https://leetcode.com/problems/unique-email-addresses)|string|[solution](rustgym/src/leetcode/_929_unique_email_addresses.rs)|
|933|[Number of Recent Calls](https://leetcode.com/problems/number-of-recent-calls)|queue|[solution](rustgym/src/leetcode/_933_number_of_recent_calls.rs)|
|937|[Reorder Data in Log Files](https://leetcode.com/problems/reorder-data-in-log-files)|string|[solution](rustgym/src/leetcode/_937_reorder_log_files.rs)|
|938|[Range Sum of BST](https://leetcode.com/problems/range-sum-of-bst)|tree recursion|[solution](rustgym/src/leetcode/_938_range_sum_of_bst.rs)|
|941|[Valid Mountain Array](https://leetcode.com/problems/valid-mountain-array)|array|[solution](rustgym/src/leetcode/_941_valid_mountain_array.rs)|
|942|[DI String Match](https://leetcode.com/problems/di-string-match)|math|[solution](rustgym/src/leetcode/_942_di_string_match.rs)|
|944|[Delete Columns to Make Sorted](https://leetcode.com/problems/delete-columns-to-make-sorted)|greedy|[solution](rustgym/src/leetcode/_944_delete_columns_to_make_sorted.rs)|
|949|[Largest Time for Given Digits](https://leetcode.com/problems/largest-time-for-given-digits)|math|[solution](rustgym/src/leetcode/_949_largest_time_for_given_digits.rs)|
|953|[Verifying an Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary)|hash-table|[solution](rustgym/src/leetcode/_953_verifying_an_alien_dictionary.rs)|
|961|[N-Repeated Element in Size 2N Array](https://leetcode.com/problems/n-repeated-element-in-size-2n-array)|hash-table|[solution](rustgym/src/leetcode/_961_n_repeated_element_in_size_2n_array.rs)|
|965|[Univalued Binary Tree](https://leetcode.com/problems/univalued-binary-tree)|tree|[solution](rustgym/src/leetcode/_965_univalued_binary_tree.rs)|
|970|[Powerful Integers](https://leetcode.com/problems/powerful-integers)|hash-table math|[solution](rustgym/src/leetcode/_970_powerful_integers.rs)|
|976|[Largest Perimeter Triangle](https://leetcode.com/problems/largest-perimeter-triangle)|math sort|[solution](rustgym/src/leetcode/_976_largest_perimeter_triangle.rs)|
|977|[Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array)|array two-pointers|[solution](rustgym/src/leetcode/_977_squares_of_a_sorted_array.rs)|
|985|[Sum of Even Numbers After Queries](https://leetcode.com/problems/sum-of-even-numbers-after-queries)|array|[solution](rustgym/src/leetcode/_985_sum_of_even_numbers_after_queries.rs)|
|989|[Add to Array-Form of Integer](https://leetcode.com/problems/add-to-array-form-of-integer)|array|[solution](rustgym/src/leetcode/_989_add_to_array_form_of_integer.rs)|
|993|[Cousins in Binary Tree](https://leetcode.com/problems/cousins-in-binary-tree)|tree breadth-first-search|[solution](rustgym/src/leetcode/_993_cousins_in_binary_tree.rs)|
|997|[Find the Town Judge](https://leetcode.com/problems/find-the-town-judge)|graph|[solution](rustgym/src/leetcode/_997_find_the_town_judge.rs)|
|999|[Available Captures for Rook](https://leetcode.com/problems/available-captures-for-rook)|array|[solution](rustgym/src/leetcode/_999_available_captures_for_rook.rs)|
|9|[Palindrome Number](https://leetcode.com/problems/palindrome-number)|math|[solution](rustgym/src/leetcode/_9_palindrome_number.rs)|
</details>
<details><summary>Medium 31/735 4.22%</summary>


|id|735 Medium Questions|Tags|704 Solutions|
|---|---|---|---|
|427|[Construct Quad Tree](https://leetcode.com/problems/construct-quad-tree)||   |
|558|[Logical OR of Two Binary Grids Represented as Quad-Trees](https://leetcode.com/problems/logical-or-of-two-binary-grids-represented-as-quad-trees)||   |
|1265|[Print Immutable Linked List in Reverse](https://leetcode.com/problems/print-immutable-linked-list-in-reverse)||   |
|1506|[Find Root of N-Ary Tree](https://leetcode.com/problems/find-root-of-n-ary-tree)||   |
|1522|[Diameter of N-Ary Tree](https://leetcode.com/problems/diameter-of-n-ary-tree)||   |
|1538|[Guess the Majority in a Hidden Array](https://leetcode.com/problems/guess-the-majority-in-a-hidden-array)||   |
|277|[Find the Celebrity](https://leetcode.com/problems/find-the-celebrity)|array|   |
|702|[Search in a Sorted Array of Unknown Size](https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size)|binary-search|   |
|1533|[Find the Index of the Large Integer](https://leetcode.com/problems/find-the-index-of-the-large-integer)|binary-search|   |
|240|[Search a 2D Matrix II](https://leetcode.com/problems/search-a-2d-matrix-ii)|binary-search divide-and-conquer|   |
|1236|[Web Crawler](https://leetcode.com/problems/web-crawler)|depth-first-search breadth-first-search|   |
|133|[Clone Graph](https://leetcode.com/problems/clone-graph)|depth-first-search breadth-first-search graph|   |
|284|[Peeking Iterator](https://leetcode.com/problems/peeking-iterator)|design|   |
|673|[Number of Longest Increasing Subsequence](https://leetcode.com/problems/number-of-longest-increasing-subsequence)|dynamic-programming|   |
|138|[Copy List with Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer)|hash-table linked-list|   |
|1485|[Clone Binary Tree With Random Pointer](https://leetcode.com/problems/clone-binary-tree-with-random-pointer)|hash-table tree depth-first-search breadth-first-search|   |
|1490|[Clone N-ary Tree](https://leetcode.com/problems/clone-n-ary-tree)|hash-table tree depth-first-search breadth-first-search|   |
|708|[Insert into a Sorted Circular Linked List](https://leetcode.com/problems/insert-into-a-sorted-circular-linked-list)|linked-list|   |
|430|[Flatten a Multilevel Doubly Linked List](https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list)|linked-list depth-first-search|   |
|426|[Convert Binary Search Tree to Sorted Doubly Linked List](https://leetcode.com/problems/convert-binary-search-tree-to-sorted-doubly-linked-list)|linked-list divide-and-conquer tree|   |
|142|[Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii)|linked-list two-pointers|   |
|767|[Reorganize String](https://leetcode.com/problems/reorganize-string)|string heap greedy sort|   |
|236|[Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree)|tree|   |
|285|[Inorder Successor in BST](https://leetcode.com/problems/inorder-successor-in-bst)|tree|   |
|449|[Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst)|tree|   |
|510|[Inorder Successor in BST II](https://leetcode.com/problems/inorder-successor-in-bst-ii)|tree|   |
|1379|[Find a Corresponding Node of a Binary Tree in a Clone of That Tree](https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree)|tree|   |
|429|[N-ary Tree Level Order Traversal](https://leetcode.com/problems/n-ary-tree-level-order-traversal)|tree breadth-first-search|   |
|116|[Populating Next Right Pointers in Each Node](https://leetcode.com/problems/populating-next-right-pointers-in-each-node)|tree depth-first-search|   |
|117|[Populating Next Right Pointers in Each Node II](https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii)|tree depth-first-search|   |
|863|[All Nodes Distance K in Binary Tree](https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree)|tree depth-first-search breadth-first-search|   |
|1003|[Check If Word Is Valid After Substitutions](https://leetcode.com/problems/check-if-word-is-valid-after-substitutions)|string stack|[solution](rustgym/src/leetcode/_1003_check_if_word_is_valid_after_substitutions.rs)|
|1004|[Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii)|two-pointers sliding-window|[solution](rustgym/src/leetcode/_1004_max_consecutive_ones_3.rs)|
|1006|[Clumsy Factorial](https://leetcode.com/problems/clumsy-factorial)|math|[solution](rustgym/src/leetcode/_1006_clumsy_factorial.rs)|
|1007|[Minimum Domino Rotations For Equal Row](https://leetcode.com/problems/minimum-domino-rotations-for-equal-row)|array greedy|[solution](rustgym/src/leetcode/_1007_minimum_domino_rotations_for_equal_row.rs)|
|1008|[Construct Binary Search Tree from Preorder Traversal](https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal)|tree|[solution](rustgym/src/leetcode/_1008_construct_binary_search_tree_from_preorder_traversal.rs)|
|1011|[Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days)|array binary-search|[solution](rustgym/src/leetcode/_1011_capacity_to_ship_packages_within_d_days.rs)|
|1014|[Best Sightseeing Pair](https://leetcode.com/problems/best-sightseeing-pair)|array|[solution](rustgym/src/leetcode/_1014_best_sightseeing_pair.rs)|
|1015|[Smallest Integer Divisible by K](https://leetcode.com/problems/smallest-integer-divisible-by-k)|math|[solution](rustgym/src/leetcode/_1015_smallest_integer_divisible_by_k.rs)|
|1016|[Binary String With Substrings Representing 1 To N](https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n)|string|[solution](rustgym/src/leetcode/_1016_binary_string_with_substrings_representing_1_to_n.rs)|
|1017|[Convert to Base -2](https://leetcode.com/problems/convert-to-base-2)|math|[solution](rustgym/src/leetcode/_1017_convert_to_base_minus_2.rs)|
|1019|[Next Greater Node In Linked List](https://leetcode.com/problems/next-greater-node-in-linked-list)|linked-list stack|[solution](rustgym/src/leetcode/_1019_next_greater_node_in_linked_list.rs)|
|1020|[Number of Enclaves](https://leetcode.com/problems/number-of-enclaves)|depth-first-search|[solution](rustgym/src/leetcode/_1020_number_of_enclaves.rs)|
|1023|[Camelcase Matching](https://leetcode.com/problems/camelcase-matching)|string trie|[solution](rustgym/src/leetcode/_1023_camelcase_matching.rs)|
|1024|[Video Stitching](https://leetcode.com/problems/video-stitching)|dynamic-programming|[solution](rustgym/src/leetcode/_1024_video_stitching.rs)|
|1026|[Maximum Difference Between Node and Ancestor](https://leetcode.com/problems/maximum-difference-between-node-and-ancestor)|tree depth-first-search|[solution](rustgym/src/leetcode/_1026_maximum_difference_between_node_and_ancestor.rs)|
|1027|[Longest Arithmetic Subsequence](https://leetcode.com/problems/longest-arithmetic-subsequence)|dynamic-programming|[solution](rustgym/src/leetcode/_1027_longest_arithmetic_sequence.rs)|
|102|[Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal)|tree breadth-first-search|[solution](rustgym/src/leetcode/_102_binary_tree_level_order_traversal.rs)|
|1031|[Maximum Sum of Two Non-Overlapping Subarrays](https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays)|array|[solution](rustgym/src/leetcode/_1031_maximum_sum_of_two_non_overlapping_subarrays.rs)|
|1034|[Coloring A Border](https://leetcode.com/problems/coloring-a-border)|depth-first-search|[solution](rustgym/src/leetcode/_1034_coloring_a_border.rs)|
|1035|[Uncrossed Lines](https://leetcode.com/problems/uncrossed-lines)|array|[solution](rustgym/src/leetcode/_1035_uncrossed_lines.rs)|
|1038|[Binary Search Tree to Greater Sum Tree](https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree)|binary-search-tree|[solution](rustgym/src/leetcode/_1038_binary_search_tree_to_greater_sum_tree.rs)|
|1039|[Minimum Score Triangulation of Polygon](https://leetcode.com/problems/minimum-score-triangulation-of-polygon)|dynamic-programming|[solution](rustgym/src/leetcode/_1039_minimum_score_triangulation_of_polygon.rs)|
|103|[Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal)|stack tree breadth-first-search|[solution](rustgym/src/leetcode/_103_binary_tree_zigzag_level_order_traversal.rs)|
|1040|[Moving Stones Until Consecutive II](https://leetcode.com/problems/moving-stones-until-consecutive-ii)|array sliding-window|[solution](rustgym/src/leetcode/_1040_moving_stones_until_consecutive_2.rs)|
|1041|[Robot Bounded In Circle](https://leetcode.com/problems/robot-bounded-in-circle)|math|[solution](rustgym/src/leetcode/_1041_robot_bounded_in_circle.rs)|
|1043|[Partition Array for Maximum Sum](https://leetcode.com/problems/partition-array-for-maximum-sum)|graph|[solution](rustgym/src/leetcode/_1043_partition_array_for_maximum_sum.rs)|
|1048|[Longest String Chain](https://leetcode.com/problems/longest-string-chain)|hash-table dynamic-programming|[solution](rustgym/src/leetcode/_1048_longest_string_chain.rs)|
|1049|[Last Stone Weight II](https://leetcode.com/problems/last-stone-weight-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_1049_last_stone_weight_2.rs)|
|1052|[Grumpy Bookstore Owner](https://leetcode.com/problems/grumpy-bookstore-owner)|array sliding-window|[solution](rustgym/src/leetcode/_1052_grumpy_bookstore_owner.rs)|
|1053|[Previous Permutation With One Swap](https://leetcode.com/problems/previous-permutation-with-one-swap)|array greedy|[solution](rustgym/src/leetcode/_1053_previous_permutation_with_one_swap.rs)|
|1054|[Distant Barcodes](https://leetcode.com/problems/distant-barcodes)|heap sort|[solution](rustgym/src/leetcode/_1054_distant_barcodes.rs)|
|1055|[Shortest Way to Form String](https://leetcode.com/problems/shortest-way-to-form-string)|dynamic-programming greedy|[solution](rustgym/src/leetcode/_1055_shortest_way_to_form_string.rs)|
|1057|[Campus Bikes](https://leetcode.com/problems/campus-bikes)|greedy sort|[solution](rustgym/src/leetcode/_1057_campus_bikes.rs)|
|1058|[Minimize Rounding Error to Meet Target](https://leetcode.com/problems/minimize-rounding-error-to-meet-target)|math dynamic-programming greedy|[solution](rustgym/src/leetcode/_1058_minimize_rounding_error_to_meet_target.rs)|
|1059|[All Paths from Source Lead to Destination](https://leetcode.com/problems/all-paths-from-source-lead-to-destination)|depth-first-search graph|[solution](rustgym/src/leetcode/_1059_all_paths_from_source_lead_to_destination.rs)|
|105|[Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal)|array tree depth-first-search|[solution](rustgym/src/leetcode/_105_construct_binary_tree_from_preorder_and_inorder_traversal.rs)|
|1060|[Missing Element in Sorted Array](https://leetcode.com/problems/missing-element-in-sorted-array)|binary-search|[solution](rustgym/src/leetcode/_1060_missing_element_in_sorted_array.rs)|
|1061|[Lexicographically Smallest Equivalent String](https://leetcode.com/problems/lexicographically-smallest-equivalent-string)|depth-first-search union-find|[solution](rustgym/src/leetcode/_1061_lexicographically_smallest_equivalent_string.rs)|
|1062|[Longest Repeating Substring](https://leetcode.com/problems/longest-repeating-substring)|string|[solution](rustgym/src/leetcode/_1062_longest_repeating_substring.rs)|
|1066|[Campus Bikes II](https://leetcode.com/problems/campus-bikes-ii)|dynamic-programming backtracking|[solution](rustgym/src/leetcode/_1066_campus_bikes_2.rs)|
|106|[Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal)|array tree depth-first-search|[solution](rustgym/src/leetcode/_106_construct_binary_tree_from_inorder_and_postorder_traversal.rs)|
|1072|[Flip Columns For Maximum Number of Equal Rows](https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows)|hash-table|[solution](rustgym/src/leetcode/_1072_flip_columns_for_maximum_number_of_equal_rows.rs)|
|1073|[Adding Two Negabinary Numbers](https://leetcode.com/problems/adding-two-negabinary-numbers)|math|[solution](rustgym/src/leetcode/_1073_adding_two_negabinary_numbers.rs)|
|1079|[Letter Tile Possibilities](https://leetcode.com/problems/letter-tile-possibilities)|backtracking|[solution](rustgym/src/leetcode/_1079_letter_tile_possibilities.rs)|
|1080|[Insufficient Nodes in Root to Leaf Paths](https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths)|depth-first-search|[solution](rustgym/src/leetcode/_1080_insufficient_nodes_in_root_to_leaf_paths.rs)|
|1081|[Smallest Subsequence of Distinct Characters](https://leetcode.com/problems/smallest-subsequence-of-distinct-characters)|string|[solution](rustgym/src/leetcode/_1081_smallest_subsequence_of_distinct_characters.rs)|
|1087|[Brace Expansion](https://leetcode.com/problems/brace-expansion)|backtracking|[solution](rustgym/src/leetcode/_1087_brace_expansion.rs)|
|1090|[Largest Values From Labels](https://leetcode.com/problems/largest-values-from-labels)|hash-table greedy|[solution](rustgym/src/leetcode/_1090_largest_values_from_labels.rs)|
|1091|[Shortest Path in Binary Matrix](https://leetcode.com/problems/shortest-path-in-binary-matrix)|breadth-first-search|[solution](rustgym/src/leetcode/_1091_shortest_path_in_binary_matrix.rs)|
|1093|[Statistics from a Large Sample](https://leetcode.com/problems/statistics-from-a-large-sample)|math two-pointers|[solution](rustgym/src/leetcode/_1093_statistics_from_a_large_sample.rs)|
|1094|[Car Pooling](https://leetcode.com/problems/car-pooling)|greedy|[solution](rustgym/src/leetcode/_1094_car_pooling.rs)|
|109|[Convert Sorted List to Binary Search Tree](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree)|linked-list depth-first-search|[solution](rustgym/src/leetcode/_109_convert_sorted_list_to_binary_search_tree.rs)|
|1100|[Find K-Length Substrings With No Repeated Characters](https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters)|string sliding-window|[solution](rustgym/src/leetcode/_1100_find_k_length_substrings_with_no_repeated_characters.rs)|
|1101|[The Earliest Moment When Everyone Become Friends](https://leetcode.com/problems/the-earliest-moment-when-everyone-become-friends)|union-find|[solution](rustgym/src/leetcode/_1101_the_earliest_moment_when_everyone_become_friends.rs)|
|1102|[Path With Maximum Minimum Value](https://leetcode.com/problems/path-with-maximum-minimum-value)|depth-first-search union-find graph|[solution](rustgym/src/leetcode/_1102_path_with_maximum_minimum_value.rs)|
|1104|[Path In Zigzag Labelled Binary Tree](https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree)|math tree|[solution](rustgym/src/leetcode/_1104_path_in_zigzag_labelled_binary_tree.rs)|
|1105|[Filling Bookcase Shelves](https://leetcode.com/problems/filling-bookcase-shelves)|dynamic-programming|[solution](rustgym/src/leetcode/_1105_filling_bookcase_shelves.rs)|
|1109|[Corporate Flight Bookings](https://leetcode.com/problems/corporate-flight-bookings)|array math|[solution](rustgym/src/leetcode/_1109_corp_flight_bookings.rs)|
|1110|[Delete Nodes And Return Forest](https://leetcode.com/problems/delete-nodes-and-return-forest)|tree depth-first-search|[solution](rustgym/src/leetcode/_1110_delete_nodes_and_return_forest.rs)|
|1111|[Maximum Nesting Depth of Two Valid Parentheses Strings](https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings)|binary-search greedy|[solution](rustgym/src/leetcode/_1111_maximum_nesting_depth_of_two_valid_parentheses_strings.rs)|
|1120|[Maximum Average Subtree](https://leetcode.com/problems/maximum-average-subtree)|tree|[solution](rustgym/src/leetcode/_1120_maximum_average_subtree.rs)|
|1123|[Lowest Common Ancestor of Deepest Leaves](https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves)|tree depth-first-search|[solution](rustgym/src/leetcode/_1123_lowest_common_ancestor_or_deepest_leaves.rs)|
|1124|[Longest Well-Performing Interval](https://leetcode.com/problems/longest-well-performing-interval)|stack|[solution](rustgym/src/leetcode/_1124_longest_well_performing_interval.rs)|
|1129|[Shortest Path with Alternating Colors](https://leetcode.com/problems/shortest-path-with-alternating-colors)|breadth-first-search graph|[solution](rustgym/src/leetcode/_1129_shortest_path_with_alternating_colors.rs)|
|1130|[Minimum Cost Tree From Leaf Values](https://leetcode.com/problems/minimum-cost-tree-from-leaf-values)|dynamic-programming stack tree|[solution](rustgym/src/leetcode/_1130_minimum_cost_tree_from_leaf_values.rs)|
|1131|[Maximum of Absolute Value Expression](https://leetcode.com/problems/maximum-of-absolute-value-expression)|math bit-manipulation|[solution](rustgym/src/leetcode/_1131_maximum_of_absolute_value_expression.rs)|
|1135|[Connecting Cities With Minimum Cost](https://leetcode.com/problems/connecting-cities-with-minimum-cost)|union-find graph|[solution](rustgym/src/leetcode/_1135_connecting_cities_with_minimum_cost.rs)|
|1138|[Alphabet Board Path](https://leetcode.com/problems/alphabet-board-path)|hash-table string|[solution](rustgym/src/leetcode/_1138_alphabet_board_path.rs)|
|1139|[Largest 1-Bordered Square](https://leetcode.com/problems/largest-1-bordered-square)|dynamic-programming|[solution](rustgym/src/leetcode/_1139_largest_1_bordered_square.rs)|
|113|[Path Sum II](https://leetcode.com/problems/path-sum-ii)|tree depth-first-search|[solution](rustgym/src/leetcode/_113_path_sum_2.rs)|
|1140|[Stone Game II](https://leetcode.com/problems/stone-game-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_1140_stone_game_2.rs)|
|1143|[Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence)|dynamic-programming|[solution](rustgym/src/leetcode/_1143_longest_common_subsequence.rs)|
|1144|[Decrease Elements To Make Array Zigzag](https://leetcode.com/problems/decrease-elements-to-make-array-zigzag)|array|[solution](rustgym/src/leetcode/_1144_decrease_elements_to_make_array_zigzag.rs)|
|1145|[Binary Tree Coloring Game](https://leetcode.com/problems/binary-tree-coloring-game)|tree depth-first-search|[solution](rustgym/src/leetcode/_1145_binary_tree_coloring_game.rs)|
|1146|[Snapshot Array](https://leetcode.com/problems/snapshot-array)|array|[solution](rustgym/src/leetcode/_1146_snapshot_array.rs)|
|114|[Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list)|tree depth-first-search|[solution](rustgym/src/leetcode/_114_flatten_binary_tree_to_linked_list.rs)|
|1151|[Minimum Swaps to Group All 1's Together](https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together)|array sliding-window|[solution](rustgym/src/leetcode/_1151_minimum_swaps_to_group_all_1s_together.rs)|
|1152|[Analyze User Website Visit Pattern](https://leetcode.com/problems/analyze-user-website-visit-pattern)|array hash-table sort|[solution](rustgym/src/leetcode/_1152_analyze_user_website_visit_pattern.rs)|
|1155|[Number of Dice Rolls With Target Sum](https://leetcode.com/problems/number-of-dice-rolls-with-target-sum)|dynamic-programming|[solution](rustgym/src/leetcode/_1155_number_of_dice_rolls_with_target_sum.rs)|
|1156|[Swap For Longest Repeated Character Substring](https://leetcode.com/problems/swap-for-longest-repeated-character-substring)|string|[solution](rustgym/src/leetcode/_1156_swap_for_longest_repeated_character_substring.rs)|
|1161|[Maximum Level Sum of a Binary Tree](https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree)|graph|[solution](rustgym/src/leetcode/_1161_maximum_level_sum_of_a_binary_tree.rs)|
|1162|[As Far from Land as Possible](https://leetcode.com/problems/as-far-from-land-as-possible)|breadth-first-search graph|[solution](rustgym/src/leetcode/_1162_as_far_from_land_as_possible.rs)|
|1166|[Design File System](https://leetcode.com/problems/design-file-system)|hash-table design|[solution](rustgym/src/leetcode/_1166_design_file_system.rs)|
|1167|[Minimum Cost to Connect Sticks](https://leetcode.com/problems/minimum-cost-to-connect-sticks)|greedy|[solution](rustgym/src/leetcode/_1167_minimum_cost_to_connect_sticks.rs)|
|1169|[Invalid Transactions](https://leetcode.com/problems/invalid-transactions)|array string|[solution](rustgym/src/leetcode/_1169_invalid_transactions.rs)|
|1171|[Remove Zero Sum Consecutive Nodes from Linked List](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list)|linked-list|[solution](rustgym/src/leetcode/_1171_remove_zero_sum_consecutive_nodes_from_linked_list.rs)|
|1177|[Can Make Palindrome from Substring](https://leetcode.com/problems/can-make-palindrome-from-substring)|array string|[solution](rustgym/src/leetcode/_1177_can_make_palindrome_from_substring.rs)|
|1181|[Before and After Puzzle](https://leetcode.com/problems/before-and-after-puzzle)|string|[solution](rustgym/src/leetcode/_1181_before_and_after_puzzle.rs)|
|1182|[Shortest Distance to Target Color](https://leetcode.com/problems/shortest-distance-to-target-color)|binary-search|[solution](rustgym/src/leetcode/_1182_shortest_distance_to_target_color.rs)|
|1186|[Maximum Subarray Sum with One Deletion](https://leetcode.com/problems/maximum-subarray-sum-with-one-deletion)|dynamic-programming|[solution](rustgym/src/leetcode/_1186_maximum_subarray_sum_with_one_deletion.rs)|
|1190|[Reverse Substrings Between Each Pair of Parentheses](https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses)|stack|[solution](rustgym/src/leetcode/_1190_reverse_substrings_between_each_pair_of_parentheses.rs)|
|1191|[K-Concatenation Maximum Sum](https://leetcode.com/problems/k-concatenation-maximum-sum)|dynamic-programming|[solution](rustgym/src/leetcode/_1191_k_concatenation_maximum_sum.rs)|
|1197|[Minimum Knight Moves](https://leetcode.com/problems/minimum-knight-moves)|breadth-first-search|[solution](rustgym/src/leetcode/_1197_minimum_knight_moves_math.rs)|
|1198|[Find Smallest Common Element in All Rows](https://leetcode.com/problems/find-smallest-common-element-in-all-rows)|hash-table binary-search|[solution](rustgym/src/leetcode/_1198_find_smallest_common_element_in_all_rows.rs)|
|11|[Container With Most Water](https://leetcode.com/problems/container-with-most-water)|array two-pointers|[solution](rustgym/src/leetcode/_11_container_with_most_water.rs)|
|1201|[Ugly Number III](https://leetcode.com/problems/ugly-number-iii)|math binary-search|[solution](rustgym/src/leetcode/_1201_ugly_number_3.rs)|
|1202|[Smallest String With Swaps](https://leetcode.com/problems/smallest-string-with-swaps)|array union-find|[solution](rustgym/src/leetcode/_1202_smallest_string_with_swaps.rs)|
|1208|[Get Equal Substrings Within Budget](https://leetcode.com/problems/get-equal-substrings-within-budget)|array sliding-window|[solution](rustgym/src/leetcode/_1208_get_equal_substrings_within_budget.rs)|
|1209|[Remove All Adjacent Duplicates in String II](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii)|stack|[solution](rustgym/src/leetcode/_1209_remove_all_adjacent_duplicates_in_string_2.rs)|
|120|[Triangle](https://leetcode.com/problems/triangle)|array dynamic-programming|[solution](rustgym/src/leetcode/_120_triangle.rs)|
|1214|[Two Sum BSTs](https://leetcode.com/problems/two-sum-bsts)|binary-search-tree|[solution](rustgym/src/leetcode/_1214_two_sum_bsts.rs)|
|1215|[Stepping Numbers](https://leetcode.com/problems/stepping-numbers)|backtracking|[solution](rustgym/src/leetcode/_1215_stepping_numbers.rs)|
|1218|[Longest Arithmetic Subsequence of Given Difference](https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference)|math dynamic-programming|[solution](rustgym/src/leetcode/_1218_longest_arithmetic_subsequence_of_given_difference.rs)|
|1219|[Path with Maximum Gold](https://leetcode.com/problems/path-with-maximum-gold)|backtracking|[solution](rustgym/src/leetcode/_1219_path_with_maximum_gold.rs)|
|1222|[Queens That Can Attack the King](https://leetcode.com/problems/queens-that-can-attack-the-king)|array|[solution](rustgym/src/leetcode/_1222_queens_that_can_attack_the_king.rs)|
|1223|[Dice Roll Simulation](https://leetcode.com/problems/dice-roll-simulation)|dynamic-programming|[solution](rustgym/src/leetcode/_1223_dice_roll_simulation.rs)|
|1227|[Airplane Seat Assignment Probability](https://leetcode.com/problems/airplane-seat-assignment-probability)|math dynamic-programming brainteaser|[solution](rustgym/src/leetcode/_1227_airplane_seat_assignment_probability.rs)|
|1229|[Meeting Scheduler](https://leetcode.com/problems/meeting-scheduler)|line-sweep|[solution](rustgym/src/leetcode/_1229_meeting_scheduler.rs)|
|1230|[Toss Strange Coins](https://leetcode.com/problems/toss-strange-coins)|math dynamic-programming|[solution](rustgym/src/leetcode/_1230_toss_strange_coins.rs)|
|1233|[Remove Sub-Folders from the Filesystem](https://leetcode.com/problems/remove-sub-folders-from-the-filesystem)|array string|[solution](rustgym/src/leetcode/_1233_remove_sub_folders_from_the_filesystem.rs)|
|1234|[Replace the Substring for Balanced String](https://leetcode.com/problems/replace-the-substring-for-balanced-string)|two-pointers string|[solution](rustgym/src/leetcode/_1234_replace_the_substring_for_balanced_string.rs)|
|1238|[Circular Permutation in Binary Representation](https://leetcode.com/problems/circular-permutation-in-binary-representation)|math|[solution](rustgym/src/leetcode/_1238_circular_permutation_in_binary_representation.rs)|
|1239|[Maximum Length of a Concatenated String with Unique Characters](https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters)|backtracking bit-manipulation|[solution](rustgym/src/leetcode/_1239_maximum_length_of_concatenated_string_with_unique_characters.rs)|
|1244|[Design A Leaderboard](https://leetcode.com/problems/design-a-leaderboard)|hash-table sort design|[solution](rustgym/src/leetcode/_1244_design_a_leaderboard.rs)|
|1245|[Tree Diameter](https://leetcode.com/problems/tree-diameter)|tree depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_1245_tree_diameter.rs)|
|1247|[Minimum Swaps to Make Strings Equal](https://leetcode.com/problems/minimum-swaps-to-make-strings-equal)|string greedy|[solution](rustgym/src/leetcode/_1247_minimum_swaps_to_make_strings_equal.rs)|
|1248|[Count Number of Nice Subarrays](https://leetcode.com/problems/count-number-of-nice-subarrays)|two-pointers|[solution](rustgym/src/leetcode/_1248_count_number_of_nice_subarrays.rs)|
|1249|[Minimum Remove to Make Valid Parentheses](https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses)|string stack|[solution](rustgym/src/leetcode/_1249_minimum_remove_to_make_valid_parentheses.rs)|
|1253|[Reconstruct a 2-Row Binary Matrix](https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix)|math greedy|[solution](rustgym/src/leetcode/_1253_reconstruct_a_2_row_binary_matrix.rs)|
|1254|[Number of Closed Islands](https://leetcode.com/problems/number-of-closed-islands)|depth-first-search|[solution](rustgym/src/leetcode/_1254_number_of_closed_islands.rs)|
|1256|[Encode Number](https://leetcode.com/problems/encode-number)|math bit-manipulation|[solution](rustgym/src/leetcode/_1256_encode_number.rs)|
|1257|[Smallest Common Region](https://leetcode.com/problems/smallest-common-region)|tree|[solution](rustgym/src/leetcode/_1257_smallest_common_region.rs)|
|1258|[Synonymous Sentences](https://leetcode.com/problems/synonymous-sentences)|backtracking|[solution](rustgym/src/leetcode/_1258_synonymous_sentences.rs)|
|1261|[Find Elements in a Contaminated Binary Tree](https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree)|hash-table tree|[solution](rustgym/src/leetcode/_1261_find_elements_in_contaminated_binary_tree.rs)|
|1262|[Greatest Sum Divisible by Three](https://leetcode.com/problems/greatest-sum-divisible-by-three)|dynamic-programming|[solution](rustgym/src/leetcode/_1262_greatest_sum_divisible_by_three.rs)|
|1267|[Count Servers that Communicate](https://leetcode.com/problems/count-servers-that-communicate)|array graph|[solution](rustgym/src/leetcode/_1267_count_servers_that_communicate.rs)|
|1268|[Search Suggestions System](https://leetcode.com/problems/search-suggestions-system)|string|[solution](rustgym/src/leetcode/_1268_search_suggestions_system.rs)|
|1272|[Remove Interval](https://leetcode.com/problems/remove-interval)|math line-sweep|[solution](rustgym/src/leetcode/_1272_remove_interval.rs)|
|1273|[Delete Tree Nodes](https://leetcode.com/problems/delete-tree-nodes)|dynamic-programming depth-first-search|[solution](rustgym/src/leetcode/_1273_delete_tree_nodes.rs)|
|1276|[Number of Burgers with No Waste of Ingredients](https://leetcode.com/problems/number-of-burgers-with-no-waste-of-ingredients)|math greedy|[solution](rustgym/src/leetcode/_1276_number_of_burgers_with_no_waste_of_ingredients.rs)|
|1277|[Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones)|array dynamic-programming|[solution](rustgym/src/leetcode/_1277_count_square_submatrices_with_all_ones.rs)|
|127|[Word Ladder](https://leetcode.com/problems/word-ladder)|breadth-first-search|[solution](rustgym/src/leetcode/_127_word_ladder.rs)|
|1282|[Group the People Given the Group Size They Belong To](https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to)|greedy|[solution](rustgym/src/leetcode/_1282_group_the_people_given_the_group_size_they_belong_to.rs)|
|1283|[Find the Smallest Divisor Given a Threshold](https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold)|binary-search|[solution](rustgym/src/leetcode/_1283_find_the_smallest_divisor_given_a_threshold.rs)|
|1286|[Iterator for Combination](https://leetcode.com/problems/iterator-for-combination)|backtracking design|[solution](rustgym/src/leetcode/_1286_iterator_for_combination.rs)|
|1288|[Remove Covered Intervals](https://leetcode.com/problems/remove-covered-intervals)|line-sweep|[solution](rustgym/src/leetcode/_1288_remove_covered_intervals.rs)|
|1291|[Sequential Digits](https://leetcode.com/problems/sequential-digits)|backtracking|[solution](rustgym/src/leetcode/_1291_sequential_digits.rs)|
|1292|[Maximum Side Length of a Square with Sum Less than or Equal to Threshold](https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold)|array binary-search|[solution](rustgym/src/leetcode/_1292_maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold.rs)|
|1296|[Divide Array in Sets of K Consecutive Numbers](https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers)|array greedy|[solution](rustgym/src/leetcode/_1296_divide_array_in_sets_of_k_consecutive_numbers.rs)|
|1297|[Maximum Number of Occurrences of a Substring](https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring)|string bit-manipulation|[solution](rustgym/src/leetcode/_1297_maximum_number_of_occurrences_of_a_substring.rs)|
|129|[Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers)|tree depth-first-search|[solution](rustgym/src/leetcode/_129_sum_root_to_leaf_numbers.rs)|
|12|[Integer to Roman](https://leetcode.com/problems/integer-to-roman)|math string|[solution](rustgym/src/leetcode/_12_integer_to_roman.rs)|
|1300|[Sum of Mutated Array Closest to Target](https://leetcode.com/problems/sum-of-mutated-array-closest-to-target)|array binary-search|[solution](rustgym/src/leetcode/_1300_sum_of_mutated_array_closest_to_target.rs)|
|1302|[Deepest Leaves Sum](https://leetcode.com/problems/deepest-leaves-sum)|tree depth-first-search|[solution](rustgym/src/leetcode/_1302_deepest_leaves_sum.rs)|
|1305|[All Elements in Two Binary Search Trees](https://leetcode.com/problems/all-elements-in-two-binary-search-trees)|sort tree|[solution](rustgym/src/leetcode/_1305_all_elements_in_two_binary_search_tree.rs)|
|1306|[Jump Game III](https://leetcode.com/problems/jump-game-iii)|breadth-first-search graph|[solution](rustgym/src/leetcode/_1306_jump_game_3.rs)|
|130|[Surrounded Regions](https://leetcode.com/problems/surrounded-regions)|depth-first-search breadth-first-search union-find|[solution](rustgym/src/leetcode/_130_surrounded_regions.rs)|
|1310|[XOR Queries of a Subarray](https://leetcode.com/problems/xor-queries-of-a-subarray)|bit-manipulation|[solution](rustgym/src/leetcode/_1310_xor_queries_of_a_subarray.rs)|
|1311|[Get Watched Videos by Your Friends](https://leetcode.com/problems/get-watched-videos-by-your-friends)|hash-table string breadth-first-search|[solution](rustgym/src/leetcode/_1311_get_watched_videos_by_your_friends.rs)|
|1314|[Matrix Block Sum](https://leetcode.com/problems/matrix-block-sum)|dynamic-programming|[solution](rustgym/src/leetcode/_1314_matrix_block_sum.rs)|
|1315|[Sum of Nodes with Even-Valued Grandparent](https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent)|tree depth-first-search|[solution](rustgym/src/leetcode/_1315_sum_of_nodes_with_even_valued_grandparent.rs)|
|1318|[Minimum Flips to Make a OR b Equal to c](https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c)|bit-manipulation|[solution](rustgym/src/leetcode/_1318_minimum_flips_to_make_a_or_b_equal_to_c.rs)|
|1319|[Number of Operations to Make Network Connected](https://leetcode.com/problems/number-of-operations-to-make-network-connected)|depth-first-search breadth-first-search union-find|[solution](rustgym/src/leetcode/_1319_number_of_operations_to_make_network_connected.rs)|
|131|[Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning)|backtracking|[solution](rustgym/src/leetcode/_131_palindrome_partitioning.rs)|
|1324|[Print Words Vertically](https://leetcode.com/problems/print-words-vertically)|string|[solution](rustgym/src/leetcode/_1324_print_words_vertically.rs)|
|1325|[Delete Leaves With a Given Value](https://leetcode.com/problems/delete-leaves-with-a-given-value)|tree|[solution](rustgym/src/leetcode/_1325_delete_leaves_with_a_given_value.rs)|
|1328|[Break a Palindrome](https://leetcode.com/problems/break-a-palindrome)|string|[solution](rustgym/src/leetcode/_1328_break_a_palindrome.rs)|
|1329|[Sort the Matrix Diagonally](https://leetcode.com/problems/sort-the-matrix-diagonally)|array sort|[solution](rustgym/src/leetcode/_1329_sort_the_matrix_diagonally.rs)|
|1333|[Filter Restaurants by Vegan-Friendly, Price and Distance](https://leetcode.com/problems/filter-restaurants-by-vegan-friendly-price-and-distance)|array sort|[solution](rustgym/src/leetcode/_1333_filter_restaurants_by_vengan_friendly_price_and_distance.rs)|
|1334|[Find the City With the Smallest Number of Neighbors at a Threshold Distance](https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance)|graph|[solution](rustgym/src/leetcode/_1334_find_the_city_with_the_smallest_number_of_neighbors_at_a_threshold_distance.rs)|
|1338|[Reduce Array Size to The Half](https://leetcode.com/problems/reduce-array-size-to-the-half)|array greedy|[solution](rustgym/src/leetcode/_1338_reduce_array_size_to_the_half.rs)|
|1339|[Maximum Product of Splitted Binary Tree](https://leetcode.com/problems/maximum-product-of-splitted-binary-tree)|dynamic-programming tree depth-first-search|[solution](rustgym/src/leetcode/_1339_maximum_product_of_splitted_binary_tree.rs)|
|1343|[Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold](https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold)|array|[solution](rustgym/src/leetcode/_1343_number_of_sub_arrays_of_size_k_and_average_greater_than_or_equal_to_threshold.rs)|
|1344|[Angle Between Hands of a Clock](https://leetcode.com/problems/angle-between-hands-of-a-clock)|math|[solution](rustgym/src/leetcode/_1344_angle_between_hands_of_a_clock.rs)|
|1347|[Minimum Number of Steps to Make Two Strings Anagram](https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram)|string|[solution](rustgym/src/leetcode/_1347_minimum_number_of_steps_to_make_two_strings_anagram.rs)|
|1348|[Tweet Counts Per Frequency](https://leetcode.com/problems/tweet-counts-per-frequency)|design|[solution](rustgym/src/leetcode/_1348_tweet_counts_per_frequency.rs)|
|134|[Gas Station](https://leetcode.com/problems/gas-station)|greedy|[solution](rustgym/src/leetcode/_134_gas_station.rs)|
|1352|[Product of the Last K Numbers](https://leetcode.com/problems/product-of-the-last-k-numbers)|array design|[solution](rustgym/src/leetcode/_1352_product_of_the_last_k_numbers.rs)|
|1353|[Maximum Number of Events That Can Be Attended](https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended)|greedy sort segment-tree|[solution](rustgym/src/leetcode/_1353_maximum_number_of_events_that_can_be_attended.rs)|
|1357|[Apply Discount Every n Orders](https://leetcode.com/problems/apply-discount-every-n-orders)|design|[solution](rustgym/src/leetcode/_1357_apply_discount_every_n_orders.rs)|
|1358|[Number of Substrings Containing All Three Characters](https://leetcode.com/problems/number-of-substrings-containing-all-three-characters)|string|[solution](rustgym/src/leetcode/_1358_number_of_substrings_containing_all_three_characters.rs)|
|1361|[Validate Binary Tree Nodes](https://leetcode.com/problems/validate-binary-tree-nodes)|graph|[solution](rustgym/src/leetcode/_1361_validate_binary_tree_nodes.rs)|
|1362|[Closest Divisors](https://leetcode.com/problems/closest-divisors)|math|[solution](rustgym/src/leetcode/_1362_closest_divisors.rs)|
|1366|[Rank Teams by Votes](https://leetcode.com/problems/rank-teams-by-votes)|array sort|[solution](rustgym/src/leetcode/_1366_rank_teams_by_votes.rs)|
|1367|[Linked List in Binary Tree](https://leetcode.com/problems/linked-list-in-binary-tree)|linked-list dynamic-programming tree|[solution](rustgym/src/leetcode/_1367_linked_list_in_binary_tree.rs)|
|1371|[Find the Longest Substring Containing Vowels in Even Counts](https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts)|string|[solution](rustgym/src/leetcode/_1371_find_the_longest_substring_containing_vowels_in_even_counts.rs)|
|1372|[Longest ZigZag Path in a Binary Tree](https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree)|dynamic-programming tree|[solution](rustgym/src/leetcode/_1372_longest_zigzag_path_in_a_binary_tree.rs)|
|1375|[Bulb Switcher III](https://leetcode.com/problems/bulb-switcher-iii)|array|[solution](rustgym/src/leetcode/_1375_bulb_switcher_3.rs)|
|1376|[Time Needed to Inform All Employees](https://leetcode.com/problems/time-needed-to-inform-all-employees)|depth-first-search|[solution](rustgym/src/leetcode/_1376_time_needed_to_inform_all_employees.rs)|
|137|[Single Number II](https://leetcode.com/problems/single-number-ii)|bit-manipulation|[solution](rustgym/src/leetcode/_137_single_number_2.rs)|
|1381|[Design a Stack With Increment Operation](https://leetcode.com/problems/design-a-stack-with-increment-operation)|stack design|[solution](rustgym/src/leetcode/_1381_design_a_stack_with_increment_operation.rs)|
|1382|[Balance a Binary Search Tree](https://leetcode.com/problems/balance-a-binary-search-tree)|binary-search-tree|[solution](rustgym/src/leetcode/_1382_balance_a_binary_search_tree.rs)|
|1386|[Cinema Seat Allocation](https://leetcode.com/problems/cinema-seat-allocation)|array greedy|[solution](rustgym/src/leetcode/_1386_cinema_seat_allocation.rs)|
|1387|[Sort Integers by The Power Value](https://leetcode.com/problems/sort-integers-by-the-power-value)|sort graph|[solution](rustgym/src/leetcode/_1387_sort_integers_by_the_power_value.rs)|
|1390|[Four Divisors](https://leetcode.com/problems/four-divisors)|math|[solution](rustgym/src/leetcode/_1390_four_divisors.rs)|
|1391|[Check if There is a Valid Path in a Grid](https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_1391_check_if_there_is_a_valid_path_in_a_grid.rs)|
|1395|[Count Number of Teams](https://leetcode.com/problems/count-number-of-teams)|array|[solution](rustgym/src/leetcode/_1395_count_number_of_teams.rs)|
|1396|[Design Underground System](https://leetcode.com/problems/design-underground-system)|design|[solution](rustgym/src/leetcode/_1396_design_underground_system.rs)|
|139|[Word Break](https://leetcode.com/problems/word-break)|dynamic-programming|[solution](rustgym/src/leetcode/_139_word_break.rs)|
|1400|[Construct K Palindrome Strings](https://leetcode.com/problems/construct-k-palindrome-strings)|greedy|[solution](rustgym/src/leetcode/_1400_construct_k_palindrome_strings.rs)|
|1401|[Circle and Rectangle Overlapping](https://leetcode.com/problems/circle-and-rectangle-overlapping)|geometry|[solution](rustgym/src/leetcode/_1401_circle_and_rectangle_overlapping.rs)|
|1404|[Number of Steps to Reduce a Number in Binary Representation to One](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one)|string bit-manipulation|[solution](rustgym/src/leetcode/_1404_number_of_steps_to_reduce_a_number_in_binary_representation_to_one.rs)|
|1405|[Longest Happy String](https://leetcode.com/problems/longest-happy-string)|dynamic-programming greedy|[solution](rustgym/src/leetcode/_1405_longest_happy_string.rs)|
|1409|[Queries on a Permutation With Key](https://leetcode.com/problems/queries-on-a-permutation-with-key)|array|[solution](rustgym/src/leetcode/_1409_queries_on_a_permutation_with_key.rs)|
|1410|[HTML Entity Parser](https://leetcode.com/problems/html-entity-parser)|string stack|[solution](rustgym/src/leetcode/_1410_html_entity_parser.rs)|
|1414|[Find the Minimum Number of Fibonacci Numbers Whose Sum Is K](https://leetcode.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k)|array greedy|[solution](rustgym/src/leetcode/_1414_find_the_minimum_number_of_fibonacci_numbers_whoes_sum_is_k.rs)|
|1415|[The k-th Lexicographical String of All Happy Strings of Length n](https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n)|backtracking|[solution](rustgym/src/leetcode/_1415_the_k_th_lexicographical_string_of_all_happy_strings_of_length_n.rs)|
|1418|[Display Table of Food Orders in a Restaurant](https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant)|hash-table|[solution](rustgym/src/leetcode/_1418_display_table_of_food_orders_in_a_restaurant.rs)|
|1419|[Minimum Number of Frogs Croaking](https://leetcode.com/problems/minimum-number-of-frogs-croaking)|string|[solution](rustgym/src/leetcode/_1419_minimum_number_of_frogs_croaking.rs)|
|1423|[Maximum Points You Can Obtain from Cards](https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards)|array dynamic-programming sliding-window|[solution](rustgym/src/leetcode/_1423_maximum_points_you_can_obtain_from_cards.rs)|
|1424|[Diagonal Traverse II](https://leetcode.com/problems/diagonal-traverse-ii)|array sort|[solution](rustgym/src/leetcode/_1424_diagonal_traversel_2.rs)|
|1428|[Leftmost Column with at Least a One](https://leetcode.com/problems/leftmost-column-with-at-least-a-one)|array|[solution](rustgym/src/leetcode/_1428_leftmost_column_with_at_least_a_one.rs)|
|1429|[First Unique Number](https://leetcode.com/problems/first-unique-number)|hash-table design|[solution](rustgym/src/leetcode/_1429_first_unique_number.rs)|
|1430|[Check If a String Is a Valid Sequence from Root to Leaves Path in a Binary Tree](https://leetcode.com/problems/check-if-a-string-is-a-valid-sequence-from-root-to-leaves-path-in-a-binary-tree)|tree|[solution](rustgym/src/leetcode/_1430_chick_if_a_string_is_a_valid_sequence_from_root_to_leaves_path_in_a_binary_tree.rs)|
|1432|[Max Difference You Can Get From Changing an Integer](https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer)|string|[solution](rustgym/src/leetcode/_1432_max_difference_you_can_get_from_change_an_integer.rs)|
|1433|[Check If a String Can Break Another String](https://leetcode.com/problems/check-if-a-string-can-break-another-string)|string greedy|[solution](rustgym/src/leetcode/_1433_check_if_a_string_can_break_another_string.rs)|
|1437|[Check If All 1's Are at Least Length K Places Away](https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away)|array|[solution](rustgym/src/leetcode/_1437_check_if_all_1s_are_at_least_length_k_places_away.rs)|
|1438|[Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit](https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit)|array sliding-window|[solution](rustgym/src/leetcode/_1438_longest_continuous_subarray_with_absolute_diff_less_than_or_equal_to_limit.rs)|
|143|[Reorder List](https://leetcode.com/problems/reorder-list)|linked-list|[solution](rustgym/src/leetcode/_143_reorder_list.rs)|
|1442|[Count Triplets That Can Form Two Arrays of Equal XOR](https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor)|array math bit-manipulation|[solution](rustgym/src/leetcode/_1442_count_triplets_that_can_form_two_arrays_of_equal_xor.rs)|
|1443|[Minimum Time to Collect All Apples in a Tree](https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_1443_minimum_time_to_collect_all_apples_in_a_tree.rs)|
|1447|[Simplified Fractions](https://leetcode.com/problems/simplified-fractions)|math|[solution](rustgym/src/leetcode/_1447_simplified_fractions.rs)|
|1448|[Count Good Nodes in Binary Tree](https://leetcode.com/problems/count-good-nodes-in-binary-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_1448_count_good_nodes_in_binary_tree.rs)|
|144|[Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal)|stack tree|[solution](rustgym/src/leetcode/_144_binary_tree_preorder_traversal.rs)|
|1451|[Rearrange Words in a Sentence](https://leetcode.com/problems/rearrange-words-in-a-sentence)|string sort|[solution](rustgym/src/leetcode/_1451_rearrange_words_in_a_sentence.rs)|
|1452|[People Whose List of Favorite Companies Is Not a Subset of Another List](https://leetcode.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list)|string sort|[solution](rustgym/src/leetcode/_1452_people_whose_list_of_favorite_companies_is_not_a_subset_of_another_list.rs)|
|1456|[Maximum Number of Vowels in a Substring of Given Length](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length)|string sliding-window|[solution](rustgym/src/leetcode/_1456_maximum_number_of_vowels_in_a_substring_of_given_length.rs)|
|1457|[Pseudo-Palindromic Paths in a Binary Tree](https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree)|bit-manipulation tree depth-first-search|[solution](rustgym/src/leetcode/_1457_pseudo_palindromic_paths_in_a_binary_tree.rs)|
|1461|[Check If a String Contains All Binary Codes of Size K](https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k)|string bit-manipulation|[solution](rustgym/src/leetcode/_1461_check_if_a_string_contains_all_binary_codes_of_size_k.rs)|
|1462|[Course Schedule IV](https://leetcode.com/problems/course-schedule-iv)|graph|[solution](rustgym/src/leetcode/_1462_course_schedule_4.rs)|
|1465|[Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts](https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts)|array|[solution](rustgym/src/leetcode/_1465_maximum_area_of_a_piece_of_cake_after_horizontal_and_vertical_cuts.rs)|
|1466|[Reorder Routes to Make All Paths Lead to the City Zero](https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero)|tree depth-first-search|[solution](rustgym/src/leetcode/_1466_reorder_routes_to_make_all_paths_lead_to_the_city_zero.rs)|
|146|[LRU Cache](https://leetcode.com/problems/lru-cache)|design|[solution](rustgym/src/leetcode/_146_lru_cache.rs)|
|1471|[The k Strongest Values in an Array](https://leetcode.com/problems/the-k-strongest-values-in-an-array)|array sort|[solution](rustgym/src/leetcode/_1471_the_k_strongest_values_in_an_array.rs)|
|1472|[Design Browser History](https://leetcode.com/problems/design-browser-history)|design|[solution](rustgym/src/leetcode/_1472_design_browser_history.rs)|
|1476|[Subrectangle Queries](https://leetcode.com/problems/subrectangle-queries)|array|[solution](rustgym/src/leetcode/_1476_subrectangle_queries.rs)|
|1477|[Find Two Non-overlapping Sub-arrays Each With Target Sum](https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum)|dynamic-programming|[solution](rustgym/src/leetcode/_1477_find_two_non_overlapping_sub_arrays_each_with_target_sum.rs)|
|147|[Insertion Sort List](https://leetcode.com/problems/insertion-sort-list)|linked-list sort|[solution](rustgym/src/leetcode/_147_insertion_sort_list.rs)|
|1481|[Least Number of Unique Integers after K Removals](https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals)|array sort|[solution](rustgym/src/leetcode/_1481_least_number_of_unique_integers_after_k_removals.rs)|
|1482|[Minimum Number of Days to Make m Bouquets](https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets)|array binary-search|[solution](rustgym/src/leetcode/_1482_minimum_number_of_days_to_make_m_bouquets.rs)|
|1487|[Making File Names Unique](https://leetcode.com/problems/making-file-names-unique)|hash-table string|[solution](rustgym/src/leetcode/_1487_making_file_names_unique.rs)|
|1488|[Avoid Flood in The City](https://leetcode.com/problems/avoid-flood-in-the-city)|array hash-table|[solution](rustgym/src/leetcode/_1488_avoid_flood_in_the_city.rs)|
|148|[Sort List](https://leetcode.com/problems/sort-list)|linked-list sort|[solution](rustgym/src/leetcode/_148_sort_list.rs)|
|1492|[The kth Factor of n](https://leetcode.com/problems/the-kth-factor-of-n)|math|[solution](rustgym/src/leetcode/_1492_the_kth_factor_of_n.rs)|
|1493|[Longest Subarray of 1's After Deleting One Element](https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element)|array|[solution](rustgym/src/leetcode/_1493_longest_subarray_of_1s_after_deleting_one_element.rs)|
|1497|[Check If Array Pairs Are Divisible by k](https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k)|array math greedy|[solution](rustgym/src/leetcode/_1497_check_if_array_pairs_are_divisible_by_k.rs)|
|1498|[Number of Subsequences That Satisfy the Given Sum Condition](https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition)|sort sliding-window|[solution](rustgym/src/leetcode/_1498_number_of_subsequences_that_satisfy_the_given_sum_condition.rs)|
|1500|[Design a File Sharing System](https://leetcode.com/problems/design-a-file-sharing-system)|array|[solution](rustgym/src/leetcode/_1500_design_a_file_sharing_system.rs)|
|1503|[Last Moment Before All Ants Fall Out of a Plank](https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank)|array brainteaser|[solution](rustgym/src/leetcode/_1503_last_moment_before_all_ants_fall_out_of_a_plank.rs)|
|1504|[Count Submatrices With All Ones](https://leetcode.com/problems/count-submatrices-with-all-ones)|dynamic-programming|[solution](rustgym/src/leetcode/_1504_count_submatrices_with_all_ones.rs)|
|1508|[Range Sum of Sorted Subarray Sums](https://leetcode.com/problems/range-sum-of-sorted-subarray-sums)|array sort|[solution](rustgym/src/leetcode/_1508_range_sum_of_sorted_subarray_sums.rs)|
|1509|[Minimum Difference Between Largest and Smallest Value in Three Moves](https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves)|array sort|[solution](rustgym/src/leetcode/_1509_minimum_difference_between_largest_and_smallest_value_in_three_moves.rs)|
|150|[Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation)|stack|[solution](rustgym/src/leetcode/_150_evaluate_reverse_polish_notation.rs)|
|1513|[Number of Substrings With Only 1s](https://leetcode.com/problems/number-of-substrings-with-only-1s)|math string|[solution](rustgym/src/leetcode/_1513_number_of_substrings_with_only_1s.rs)|
|1514|[Path with Maximum Probability](https://leetcode.com/problems/path-with-maximum-probability)|graph|[solution](rustgym/src/leetcode/_1514_path_with_maximum_probability.rs)|
|1519|[Number of Nodes in the Sub-Tree With the Same Label](https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_1519_number_of_nodes_in_the_sub_tree_with_the_same_label.rs)|
|151|[Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string)|string|[solution](rustgym/src/leetcode/_151_reverse_words_in_a_string.rs)|
|1524|[Number of Sub-arrays With Odd Sum](https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum)|array math|[solution](rustgym/src/leetcode/_1524_number_of_sub_arrays_with_odd_sum.rs)|
|1525|[Number of Good Ways to Split a String](https://leetcode.com/problems/number-of-good-ways-to-split-a-string)|string bit-manipulation|[solution](rustgym/src/leetcode/_1525_number_of_good_ways_to_split_a_string.rs)|
|1529|[Bulb Switcher IV](https://leetcode.com/problems/bulb-switcher-iv)|string|[solution](rustgym/src/leetcode/_1529_bulb_switcher_5.rs)|
|152|[Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray)|array dynamic-programming|[solution](rustgym/src/leetcode/_152_maximum_product_subarray.rs)|
|1530|[Number of Good Leaf Nodes Pairs](https://leetcode.com/problems/number-of-good-leaf-nodes-pairs)|tree depth-first-search|[solution](rustgym/src/leetcode/_1530_number_of_good_leaf_nodes_pairs.rs)|
|1535|[Find the Winner of an Array Game](https://leetcode.com/problems/find-the-winner-of-an-array-game)|array|[solution](rustgym/src/leetcode/_1535_find_the_winner_of_an_array_game.rs)|
|1536|[Minimum Swaps to Arrange a Binary Grid](https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid)|greedy|[solution](rustgym/src/leetcode/_1536_minimum_swaps_to_arrange_a_binary_grid.rs)|
|153|[Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array)|array binary-search|[solution](rustgym/src/leetcode/_153_find_minimum_in_rotated_sorted_array.rs)|
|1540|[Can Convert String in K Moves](https://leetcode.com/problems/can-convert-string-in-k-moves)|string greedy|[solution](rustgym/src/leetcode/_1540_can_convert_string_in_k_moves.rs)|
|1541|[Minimum Insertions to Balance a Parentheses String](https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string)|string stack|[solution](rustgym/src/leetcode/_1541_minimum_insertions_to_balance_a_parentheses_string.rs)|
|1545|[Find Kth Bit in Nth Binary String](https://leetcode.com/problems/find-kth-bit-in-nth-binary-string)|string|[solution](rustgym/src/leetcode/_1545_find_kth_bit_in_nth_binary_stringrs.rs)|
|1546|[Maximum Number of Non-Overlapping Subarrays With Sum Equals Target](https://leetcode.com/problems/maximum-number-of-non-overlapping-subarrays-with-sum-equals-target)|dynamic-programming|[solution](rustgym/src/leetcode/_1546_maximum_number_of_non_overlapping_subarrays_with_sum_equals_target.rs)|
|1551|[Minimum Operations to Make Array Equal](https://leetcode.com/problems/minimum-operations-to-make-array-equal)|math|[solution](rustgym/src/leetcode/_1551_minimum_operations_to_make_array_equalrs.rs)|
|1552|[Magnetic Force Between Two Balls](https://leetcode.com/problems/magnetic-force-between-two-balls)|array binary-search|[solution](rustgym/src/leetcode/_1552_magnetic_force_between_two_balls.rs)|
|156|[Binary Tree Upside Down](https://leetcode.com/problems/binary-tree-upside-down)|tree|[solution](rustgym/src/leetcode/_156_binary_tree_upside_down.rs)|
|159|[Longest Substring with At Most Two Distinct Characters](https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters)|hash-table two-pointers string sliding-window|[solution](rustgym/src/leetcode/_159_longest_substring_with_at_most_two_distinc_characters.rs)|
|15|[3Sum](https://leetcode.com/problems/3sum)|array two-pointers|[solution](rustgym/src/leetcode/_15_three_sum.rs)|
|161|[One Edit Distance](https://leetcode.com/problems/one-edit-distance)|string|[solution](rustgym/src/leetcode/_161_one_edit_distance.rs)|
|162|[Find Peak Element](https://leetcode.com/problems/find-peak-element)|array binary-search|[solution](rustgym/src/leetcode/_162_find_peak_element.rs)|
|163|[Missing Ranges](https://leetcode.com/problems/missing-ranges)|array|[solution](rustgym/src/leetcode/_163_missing_ranges.rs)|
|165|[Compare Version Numbers](https://leetcode.com/problems/compare-version-numbers)|string|[solution](rustgym/src/leetcode/_165_compare_version_numbers.rs)|
|166|[Fraction to Recurring Decimal](https://leetcode.com/problems/fraction-to-recurring-decimal)|hash-table math|[solution](rustgym/src/leetcode/_166_fraction_to_recurring_decimal.rs)|
|16|[3Sum Closest](https://leetcode.com/problems/3sum-closest)|array two-pointers|[solution](rustgym/src/leetcode/_16_3sum_closest.rs)|
|173|[Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator)|stack tree design|[solution](rustgym/src/leetcode/_173_binary_search_tree_iterator.rs)|
|179|[Largest Number](https://leetcode.com/problems/largest-number)|sort|[solution](rustgym/src/leetcode/_179_largest_number.rs)|
|17|[Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number)|string backtracking|[solution](rustgym/src/leetcode/_17_letter_combinations_of_a_phone_number.rs)|
|186|[Reverse Words in a String II](https://leetcode.com/problems/reverse-words-in-a-string-ii)|string|[solution](rustgym/src/leetcode/_186_reverse_words_in_a_string_2.rs)|
|187|[Repeated DNA Sequences](https://leetcode.com/problems/repeated-dna-sequences)|hash-table bit-manipulation|[solution](rustgym/src/leetcode/_187_repeated_dna_sequences.rs)|
|18|[4Sum](https://leetcode.com/problems/4sum)|array hash-table two-pointers|[solution](rustgym/src/leetcode/_18_4sum.rs)|
|199|[Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view)|tree depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_199_binary_tree_right_side_view.rs)|
|19|[Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list)|linked-list two-pointers|[solution](rustgym/src/leetcode/_19_remove_nth_node_from_end_of_list.rs)|
|200|[Number of Islands](https://leetcode.com/problems/number-of-islands)|depth-first-search breadth-first-search union-find|[solution](rustgym/src/leetcode/_200_number_of_islands.rs)|
|201|[Bitwise AND of Numbers Range](https://leetcode.com/problems/bitwise-and-of-numbers-range)|bit-manipulation|[solution](rustgym/src/leetcode/_201_bitwise_and_of_numbers_range.rs)|
|207|[Course Schedule](https://leetcode.com/problems/course-schedule)|depth-first-search breadth-first-search graph topological-sort|[solution](rustgym/src/leetcode/_207_course_schedule.rs)|
|208|[Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree)|design trie|[solution](rustgym/src/leetcode/_208_implement_trie.rs)|
|209|[Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum)|array two-pointers binary-search|[solution](rustgym/src/leetcode/_209_minimum_size_subarray_sum.rs)|
|210|[Course Schedule II](https://leetcode.com/problems/course-schedule-ii)|depth-first-search breadth-first-search graph topological-sort|[solution](rustgym/src/leetcode/_210_course_schedule_2.rs)|
|211|[Design Add and Search Words Data Structure](https://leetcode.com/problems/design-add-and-search-words-data-structure)|backtracking design trie|[solution](rustgym/src/leetcode/_211_add_and_search_word_data_structure_design.rs)|
|213|[House Robber II](https://leetcode.com/problems/house-robber-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_213_house_robber_2.rs)|
|215|[Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array)|divide-and-conquer heap|[solution](rustgym/src/leetcode/_215_kth_largest_element_in_an_array.rs)|
|216|[Combination Sum III](https://leetcode.com/problems/combination-sum-iii)|array backtracking|[solution](rustgym/src/leetcode/_216_combination_sum_3.rs)|
|220|[Contains Duplicate III](https://leetcode.com/problems/contains-duplicate-iii)|sort ordered-map|[solution](rustgym/src/leetcode/_220_contains_duplicate_3.rs)|
|221|[Maximal Square](https://leetcode.com/problems/maximal-square)|dynamic-programming|[solution](rustgym/src/leetcode/_221_maximal_square.rs)|
|222|[Count Complete Tree Nodes](https://leetcode.com/problems/count-complete-tree-nodes)|binary-search tree|[solution](rustgym/src/leetcode/_222_count_complete_tree_nodes.rs)|
|223|[Rectangle Area](https://leetcode.com/problems/rectangle-area)|math|[solution](rustgym/src/leetcode/_223_rectangle_area.rs)|
|227|[Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii)|string|[solution](rustgym/src/leetcode/_227_basic_calculator_2.rs)|
|228|[Summary Ranges](https://leetcode.com/problems/summary-ranges)|array|[solution](rustgym/src/leetcode/_228_summary_ranges.rs)|
|229|[Majority Element II](https://leetcode.com/problems/majority-element-ii)|array|[solution](rustgym/src/leetcode/_229_majority_element_2.rs)|
|22|[Generate Parentheses](https://leetcode.com/problems/generate-parentheses)|string backtracking|[solution](rustgym/src/leetcode/_22_generate_parentheses.rs)|
|230|[Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst)|binary-search tree|[solution](rustgym/src/leetcode/_230_kth_smallest_element_in_a_bst.rs)|
|238|[Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self)|array|[solution](rustgym/src/leetcode/_238_product_of_array_except_self.rs)|
|241|[Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses)|divide-and-conquer|[solution](rustgym/src/leetcode/_241_different_ways_to_add_parentheses.rs)|
|244|[Shortest Word Distance II](https://leetcode.com/problems/shortest-word-distance-ii)|hash-table design|[solution](rustgym/src/leetcode/_244_shortest_word_distance_2.rs)|
|245|[Shortest Word Distance III](https://leetcode.com/problems/shortest-word-distance-iii)|array|[solution](rustgym/src/leetcode/_245_shortest_word_distance_3.rs)|
|247|[Strobogrammatic Number II](https://leetcode.com/problems/strobogrammatic-number-ii)|math recursion|[solution](rustgym/src/leetcode/_247_strobogrammatic_number_2.rs)|
|249|[Group Shifted Strings](https://leetcode.com/problems/group-shifted-strings)|hash-table string|[solution](rustgym/src/leetcode/_249_group_shifted_strings.rs)|
|24|[Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs)|linked-list|[solution](rustgym/src/leetcode/_24_swap_nodes_in_pairs.rs)|
|250|[Count Univalue Subtrees](https://leetcode.com/problems/count-univalue-subtrees)|tree|[solution](rustgym/src/leetcode/_250_count_univalue_subtrees.rs)|
|251|[Flatten 2D Vector](https://leetcode.com/problems/flatten-2d-vector)|design|[solution](rustgym/src/leetcode/_251_flatten_2d_vector.rs)|
|253|[Meeting Rooms II](https://leetcode.com/problems/meeting-rooms-ii)|heap greedy sort|[solution](rustgym/src/leetcode/_253_meeting_rooms_2.rs)|
|254|[Factor Combinations](https://leetcode.com/problems/factor-combinations)|backtracking|[solution](rustgym/src/leetcode/_254_factor_combinations.rs)|
|255|[Verify Preorder Sequence in Binary Search Tree](https://leetcode.com/problems/verify-preorder-sequence-in-binary-search-tree)|stack tree|[solution](rustgym/src/leetcode/_255_verify_preorder_sequence_in_binary_search_tree.rs)|
|259|[3Sum Smaller](https://leetcode.com/problems/3sum-smaller)|array two-pointers|[solution](rustgym/src/leetcode/_259_3sum_smaller.rs)|
|260|[Single Number III](https://leetcode.com/problems/single-number-iii)|bit-manipulation|[solution](rustgym/src/leetcode/_260_single_number_3.rs)|
|261|[Graph Valid Tree](https://leetcode.com/problems/graph-valid-tree)|depth-first-search breadth-first-search union-find graph|[solution](rustgym/src/leetcode/_261_graph_valid_tree.rs)|
|264|[Ugly Number II](https://leetcode.com/problems/ugly-number-ii)|math dynamic-programming heap|[solution](rustgym/src/leetcode/_264_ugly_number_2.rs)|
|267|[Palindrome Permutation II](https://leetcode.com/problems/palindrome-permutation-ii)|backtracking|[solution](rustgym/src/leetcode/_267_palindrome_permutation_2.rs)|
|271|[Encode and Decode Strings](https://leetcode.com/problems/encode-and-decode-strings)|string|[solution](rustgym/src/leetcode/_271_encode_and_decode_strings.rs)|
|274|[H-Index](https://leetcode.com/problems/h-index)|hash-table sort|[solution](rustgym/src/leetcode/_274_h_index.rs)|
|275|[H-Index II](https://leetcode.com/problems/h-index-ii)|binary-search|[solution](rustgym/src/leetcode/_275_h_index_2.rs)|
|279|[Perfect Squares](https://leetcode.com/problems/perfect-squares)|math dynamic-programming breadth-first-search|[solution](rustgym/src/leetcode/_279_perfect_squares.rs)|
|280|[Wiggle Sort](https://leetcode.com/problems/wiggle-sort)|array sort|[solution](rustgym/src/leetcode/_280_wiggle_sort.rs)|
|281|[Zigzag Iterator](https://leetcode.com/problems/zigzag-iterator)|design|[solution](rustgym/src/leetcode/_281_zigzag_iterator.rs)|
|286|[Walls and Gates](https://leetcode.com/problems/walls-and-gates)|breadth-first-search|[solution](rustgym/src/leetcode/_286_walls_and_gates.rs)|
|287|[Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number)|array two-pointers binary-search|[solution](rustgym/src/leetcode/_287_find_the_duplicate_number.rs)|
|288|[Unique Word Abbreviation](https://leetcode.com/problems/unique-word-abbreviation)|hash-table design|[solution](rustgym/src/leetcode/_288_unique_word_abbreviation.rs)|
|289|[Game of Life](https://leetcode.com/problems/game-of-life)|array|[solution](rustgym/src/leetcode/_289_game_of_life.rs)|
|294|[Flip Game II](https://leetcode.com/problems/flip-game-ii)|backtracking minimax|[solution](rustgym/src/leetcode/_294_flip_game_2.rs)|
|298|[Binary Tree Longest Consecutive Sequence](https://leetcode.com/problems/binary-tree-longest-consecutive-sequence)|tree|[solution](rustgym/src/leetcode/_298_binary_tree_longest_consecutive_sequence.rs)|
|29|[Divide Two Integers](https://leetcode.com/problems/divide-two-integers)|math binary-search|[solution](rustgym/src/leetcode/_29_divide_two_integers.rs)|
|2|[Add Two Numbers](https://leetcode.com/problems/add-two-numbers)|linked-list math|[solution](rustgym/src/leetcode/_2_add_two_numbers.rs)|
|300|[Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence)|binary-search dynamic-programming|[solution](rustgym/src/leetcode/_300_longest_increasing_subsequence.rs)|
|304|[Range Sum Query 2D - Immutable](https://leetcode.com/problems/range-sum-query-2d-immutable)|dynamic-programming|[solution](rustgym/src/leetcode/_304_range_sum_query_2d_immutable.rs)|
|306|[Additive Number](https://leetcode.com/problems/additive-number)|backtracking|[solution](rustgym/src/leetcode/_306_additive_number.rs)|
|307|[Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable)|binary-indexed-tree segment-tree|[solution](rustgym/src/leetcode/_307_range_sum_query_mutable.rs)|
|309|[Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown)|dynamic-programming|[solution](rustgym/src/leetcode/_309_best_time_to_buy_and_sell_stock_with_cooldown.rs)|
|310|[Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees)|breadth-first-search graph|[solution](rustgym/src/leetcode/_310_minimum_height_trees.rs)|
|311|[Sparse Matrix Multiplication](https://leetcode.com/problems/sparse-matrix-multiplication)|hash-table|[solution](rustgym/src/leetcode/_311_sparse_matrix_multiplication.rs)|
|313|[Super Ugly Number](https://leetcode.com/problems/super-ugly-number)|math heap|[solution](rustgym/src/leetcode/_313_super_ugly_number.rs)|
|314|[Binary Tree Vertical Order Traversal](https://leetcode.com/problems/binary-tree-vertical-order-traversal)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_314_binary_tree_vertical_order_traversal.rs)|
|318|[Maximum Product of Word Lengths](https://leetcode.com/problems/maximum-product-of-word-lengths)|bit-manipulation|[solution](rustgym/src/leetcode/_318_maximum_product_of_word_lengths.rs)|
|319|[Bulb Switcher](https://leetcode.com/problems/bulb-switcher)|math brainteaser|[solution](rustgym/src/leetcode/_319_bulb_switcher.rs)|
|31|[Next Permutation](https://leetcode.com/problems/next-permutation)|array|[solution](rustgym/src/leetcode/_31_next_permutation.rs)|
|320|[Generalized Abbreviation](https://leetcode.com/problems/generalized-abbreviation)|backtracking bit-manipulation|[solution](rustgym/src/leetcode/_320_generalized_abbreviation.rs)|
|322|[Coin Change](https://leetcode.com/problems/coin-change)|dynamic-programming|[solution](rustgym/src/leetcode/_322_coin_change.rs)|
|323|[Number of Connected Components in an Undirected Graph](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph)|depth-first-search breadth-first-search union-find graph|[solution](rustgym/src/leetcode/_323_number_of_connected_components_in_an_unditected_graph.rs)|
|324|[Wiggle Sort II](https://leetcode.com/problems/wiggle-sort-ii)|sort|[solution](rustgym/src/leetcode/_324_wiggle_sort_2.rs)|
|325|[Maximum Size Subarray Sum Equals k](https://leetcode.com/problems/maximum-size-subarray-sum-equals-k)|hash-table|[solution](rustgym/src/leetcode/_325_maximum_size_subarray_sum_equals_k.rs)|
|328|[Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list)|linked-list|[solution](rustgym/src/leetcode/_328_odd_even_linked_list.rs)|
|331|[Verify Preorder Serialization of a Binary Tree](https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree)|stack|[solution](rustgym/src/leetcode/_331_verify_preorder_serialization_of_a_binary_tree.rs)|
|332|[Reconstruct Itinerary](https://leetcode.com/problems/reconstruct-itinerary)|depth-first-search graph|[solution](rustgym/src/leetcode/_332_reconstruct_itinerary.rs)|
|333|[Largest BST Subtree](https://leetcode.com/problems/largest-bst-subtree)|tree|[solution](rustgym/src/leetcode/_333_largest_bst_subtree.rs)|
|334|[Increasing Triplet Subsequence](https://leetcode.com/problems/increasing-triplet-subsequence)||[solution](rustgym/src/leetcode/_334_increasing_triplet_subsequence.rs)|
|337|[House Robber III](https://leetcode.com/problems/house-robber-iii)|tree depth-first-search|[solution](rustgym/src/leetcode/_337_house_robber_3.rs)|
|338|[Counting Bits](https://leetcode.com/problems/counting-bits)|dynamic-programming bit-manipulation|[solution](rustgym/src/leetcode/_338_counting_bits.rs)|
|33|[Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array)|array binary-search|[solution](rustgym/src/leetcode/_33_search_in_rotated_sorted_array.rs)|
|341|[Flatten Nested List Iterator](https://leetcode.com/problems/flatten-nested-list-iterator)|stack design|[solution](rustgym/src/leetcode/_341_flatten_nested_list_iterator.rs)|
|343|[Integer Break](https://leetcode.com/problems/integer-break)|math dynamic-programming|[solution](rustgym/src/leetcode/_343_integer_break.rs)|
|347|[Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements)|hash-table heap|[solution](rustgym/src/leetcode/_347_top_k_frequent_elements.rs)|
|348|[Design Tic-Tac-Toe](https://leetcode.com/problems/design-tic-tac-toe)|design|[solution](rustgym/src/leetcode/_348_design_tic_tac_toe.rs)|
|34|[Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array)|array binary-search|[solution](rustgym/src/leetcode/_34_find_first_and_last_position_of_elements_in_sorted_array.rs)|
|351|[Android Unlock Patterns](https://leetcode.com/problems/android-unlock-patterns)|dynamic-programming backtracking|[solution](rustgym/src/leetcode/_351_android_unlock_patterns.rs)|
|353|[Design Snake Game](https://leetcode.com/problems/design-snake-game)|design queue|[solution](rustgym/src/leetcode/_353_design_snake_game.rs)|
|355|[Design Twitter](https://leetcode.com/problems/design-twitter)|hash-table heap design|[solution](rustgym/src/leetcode/_355_design_twitter.rs)|
|356|[Line Reflection](https://leetcode.com/problems/line-reflection)|hash-table math|[solution](rustgym/src/leetcode/_356_line_reflection.rs)|
|357|[Count Numbers with Unique Digits](https://leetcode.com/problems/count-numbers-with-unique-digits)|math dynamic-programming backtracking|[solution](rustgym/src/leetcode/_357_count_numbers_with_unique_digits.rs)|
|360|[Sort Transformed Array](https://leetcode.com/problems/sort-transformed-array)|math two-pointers sort|[solution](rustgym/src/leetcode/_360_sort_transformed_array.rs)|
|361|[Bomb Enemy](https://leetcode.com/problems/bomb-enemy)|dynamic-programming|[solution](rustgym/src/leetcode/_361_bomb_enemy.rs)|
|362|[Design Hit Counter](https://leetcode.com/problems/design-hit-counter)|design|[solution](rustgym/src/leetcode/_362_design_hit_counter.rs)|
|364|[Nested List Weight Sum II](https://leetcode.com/problems/nested-list-weight-sum-ii)|depth-first-search|[solution](rustgym/src/leetcode/_364_nested_list_weight_sum_2.rs)|
|365|[Water and Jug Problem](https://leetcode.com/problems/water-and-jug-problem)|math|[solution](rustgym/src/leetcode/_365_water_and_jug_problem.rs)|
|366|[Find Leaves of Binary Tree](https://leetcode.com/problems/find-leaves-of-binary-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_366_find_leaves_of_binary_tree.rs)|
|368|[Largest Divisible Subset](https://leetcode.com/problems/largest-divisible-subset)|math dynamic-programming|[solution](rustgym/src/leetcode/_368_largest_divisible_subset.rs)|
|369|[Plus One Linked List](https://leetcode.com/problems/plus-one-linked-list)|linked-list|[solution](rustgym/src/leetcode/_369_plus_one_linked_list.rs)|
|36|[Valid Sudoku](https://leetcode.com/problems/valid-sudoku)|hash-table|[solution](rustgym/src/leetcode/_36_valid_sudoku.rs)|
|370|[Range Addition](https://leetcode.com/problems/range-addition)|array|[solution](rustgym/src/leetcode/_370_range_addition.rs)|
|371|[Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers)|bit-manipulation|[solution](rustgym/src/leetcode/_371_sum_of_two_integers.rs)|
|372|[Super Pow](https://leetcode.com/problems/super-pow)|math|[solution](rustgym/src/leetcode/_372_super_pow.rs)|
|373|[Find K Pairs with Smallest Sums](https://leetcode.com/problems/find-k-pairs-with-smallest-sums)|heap|[solution](rustgym/src/leetcode/_373_find_k_pairs_with_smallest_sums.rs)|
|375|[Guess Number Higher or Lower II](https://leetcode.com/problems/guess-number-higher-or-lower-ii)|dynamic-programming minimax|[solution](rustgym/src/leetcode/_375_guess_number_higher_or_lower_2.rs)|
|376|[Wiggle Subsequence](https://leetcode.com/problems/wiggle-subsequence)|dynamic-programming greedy|[solution](rustgym/src/leetcode/_376_wiggle_subsequence.rs)|
|377|[Combination Sum IV](https://leetcode.com/problems/combination-sum-iv)|dynamic-programming|[solution](rustgym/src/leetcode/_377_combination_sum_4.rs)|
|378|[Kth Smallest Element in a Sorted Matrix](https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix)|binary-search heap|[solution](rustgym/src/leetcode/_378_kth_smallest_element_in_a_sorted_matrix.rs)|
|379|[Design Phone Directory](https://leetcode.com/problems/design-phone-directory)|linked-list design|[solution](rustgym/src/leetcode/_379_design_phone_directory.rs)|
|380|[Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1)|array hash-table design|[solution](rustgym/src/leetcode/_380_insert_delete_get_random_o1.rs)|
|382|[Linked List Random Node](https://leetcode.com/problems/linked-list-random-node)|reservoir-sampling|[solution](rustgym/src/leetcode/_382_linked_list_random_node.rs)|
|384|[Shuffle an Array](https://leetcode.com/problems/shuffle-an-array)||[solution](rustgym/src/leetcode/_384_shuffle_an_array.rs)|
|385|[Mini Parser](https://leetcode.com/problems/mini-parser)|string stack|[solution](rustgym/src/leetcode/_385_mini_parser.rs)|
|386|[Lexicographical Numbers](https://leetcode.com/problems/lexicographical-numbers)||[solution](rustgym/src/leetcode/_386_lexicographical_numbers.rs)|
|388|[Longest Absolute File Path](https://leetcode.com/problems/longest-absolute-file-path)||[solution](rustgym/src/leetcode/_388_longest_absolute_file_path.rs)|
|390|[Elimination Game](https://leetcode.com/problems/elimination-game)||[solution](rustgym/src/leetcode/_390_elimination_game.rs)|
|393|[UTF-8 Validation](https://leetcode.com/problems/utf-8-validation)|bit-manipulation|[solution](rustgym/src/leetcode/_393_utf8_validation.rs)|
|394|[Decode String](https://leetcode.com/problems/decode-string)|stack depth-first-search|[solution](rustgym/src/leetcode/_394_decode_string.rs)|
|395|[Longest Substring with At Least K Repeating Characters](https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters)||[solution](rustgym/src/leetcode/_395_longest_substring_with_at_least_k_repeating_characters.rs)|
|396|[Rotate Function](https://leetcode.com/problems/rotate-function)|math|[solution](rustgym/src/leetcode/_396_rotate_function.rs)|
|397|[Integer Replacement](https://leetcode.com/problems/integer-replacement)|math bit-manipulation|[solution](rustgym/src/leetcode/_397_integer_replacement.rs)|
|398|[Random Pick Index](https://leetcode.com/problems/random-pick-index)|reservoir-sampling|[solution](rustgym/src/leetcode/_398_random_pick_index.rs)|
|399|[Evaluate Division](https://leetcode.com/problems/evaluate-division)|union-find graph|[solution](rustgym/src/leetcode/_399_evaluate_division.rs)|
|39|[Combination Sum](https://leetcode.com/problems/combination-sum)|array backtracking|[solution](rustgym/src/leetcode/_39_combination_sum.rs)|
|3|[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters)|hash-table two-pointers string sliding-window|[solution](rustgym/src/leetcode/_3_longest_substring_without_repeating_characters.rs)|
|400|[Nth Digit](https://leetcode.com/problems/nth-digit)|math|[solution](rustgym/src/leetcode/_400_nth_digit.rs)|
|402|[Remove K Digits](https://leetcode.com/problems/remove-k-digits)|stack greedy|[solution](rustgym/src/leetcode/_402_remove_k_digits.rs)|
|406|[Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height)|greedy|[solution](rustgym/src/leetcode/_406_queue_reconstruction_by_height.rs)|
|40|[Combination Sum II](https://leetcode.com/problems/combination-sum-ii)|array backtracking|[solution](rustgym/src/leetcode/_40_combination_sum_2.rs)|
|413|[Arithmetic Slices](https://leetcode.com/problems/arithmetic-slices)|math dynamic-programming|[solution](rustgym/src/leetcode/_413_arithmetic_slices.rs)|
|416|[Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum)|dynamic-programming|[solution](rustgym/src/leetcode/_416_partition_equal_subset_sum.rs)|
|417|[Pacific Atlantic Water Flow](https://leetcode.com/problems/pacific-atlantic-water-flow)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_417_pacific_atlantic_water_flow.rs)|
|418|[Sentence Screen Fitting](https://leetcode.com/problems/sentence-screen-fitting)|dynamic-programming|[solution](rustgym/src/leetcode/_418_sentence_screen_fitting.rs)|
|419|[Battleships in a Board](https://leetcode.com/problems/battleships-in-a-board)||[solution](rustgym/src/leetcode/_419_battleships_in_a_board.rs)|
|421|[Maximum XOR of Two Numbers in an Array](https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array)|bit-manipulation trie|[solution](rustgym/src/leetcode/_421_maximum_xor_of_two_numbers_in_an_array.rs)|
|423|[Reconstruct Original Digits from English](https://leetcode.com/problems/reconstruct-original-digits-from-english)|math|[solution](rustgym/src/leetcode/_423_reconstruct_original_digits_from_english.rs)|
|424|[Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement)|two-pointers sliding-window|[solution](rustgym/src/leetcode/_424_longest_repeating_character_replacement.rs)|
|433|[Minimum Genetic Mutation](https://leetcode.com/problems/minimum-genetic-mutation)||[solution](rustgym/src/leetcode/_433_minimum_genetic_mutation.rs)|
|435|[Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals)|greedy|[solution](rustgym/src/leetcode/_435_non_overlapping_intervals.rs)|
|436|[Find Right Interval](https://leetcode.com/problems/find-right-interval)|binary-search|[solution](rustgym/src/leetcode/_436_find_right_interval.rs)|
|437|[Path Sum III](https://leetcode.com/problems/path-sum-iii)|tree|[solution](rustgym/src/leetcode/_437_path_sum_3.rs)|
|438|[Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string)|hash-table|[solution](rustgym/src/leetcode/_438_find_all_anagrams_in_a_string.rs)|
|439|[Ternary Expression Parser](https://leetcode.com/problems/ternary-expression-parser)|stack depth-first-search|[solution](rustgym/src/leetcode/_439_ternary_expression_parser.rs)|
|43|[Multiply Strings](https://leetcode.com/problems/multiply-strings)|math string|[solution](rustgym/src/leetcode/_43_multiply_strings.rs)|
|442|[Find All Duplicates in an Array](https://leetcode.com/problems/find-all-duplicates-in-an-array)|array|[solution](rustgym/src/leetcode/_442_find_all_duplicates_in_an_array.rs)|
|444|[Sequence Reconstruction](https://leetcode.com/problems/sequence-reconstruction)|graph topological-sort|[solution](rustgym/src/leetcode/_444_sequence_reconstruction.rs)|
|445|[Add Two Numbers II](https://leetcode.com/problems/add-two-numbers-ii)|linked-list|[solution](rustgym/src/leetcode/_445_add_two_numbers_2.rs)|
|450|[Delete Node in a BST](https://leetcode.com/problems/delete-node-in-a-bst)|tree|[solution](rustgym/src/leetcode/_450_delete_node_in_a_bst.rs)|
|451|[Sort Characters By Frequency](https://leetcode.com/problems/sort-characters-by-frequency)|hash-table heap|[solution](rustgym/src/leetcode/_451_sort_characters_by_frequency.rs)|
|452|[Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons)|greedy|[solution](rustgym/src/leetcode/_452_minimum_number_of_arrows_to_burst_ballons.rs)|
|454|[4Sum II](https://leetcode.com/problems/4sum-ii)|hash-table binary-search|[solution](rustgym/src/leetcode/_454_4sum_2.rs)|
|456|[132 Pattern](https://leetcode.com/problems/132-pattern)|stack|[solution](rustgym/src/leetcode/_456_132_pattern.rs)|
|457|[Circular Array Loop](https://leetcode.com/problems/circular-array-loop)|array two-pointers|[solution](rustgym/src/leetcode/_457_circular_array_loop.rs)|
|462|[Minimum Moves to Equal Array Elements II](https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii)|math|[solution](rustgym/src/leetcode/_462_minimum_moves_to_equal_array_elements_2.rs)|
|464|[Can I Win](https://leetcode.com/problems/can-i-win)|dynamic-programming minimax|[solution](rustgym/src/leetcode/_464_can_i_win.rs)|
|467|[Unique Substrings in Wraparound String](https://leetcode.com/problems/unique-substrings-in-wraparound-string)|dynamic-programming|[solution](rustgym/src/leetcode/_467_unique_substrings_in_wraparound_string.rs)|
|468|[Validate IP Address](https://leetcode.com/problems/validate-ip-address)|string|[solution](rustgym/src/leetcode/_468_validate_ip_address.rs)|
|469|[Convex Polygon](https://leetcode.com/problems/convex-polygon)|math|[solution](rustgym/src/leetcode/_469_convex_polygon.rs)|
|46|[Permutations](https://leetcode.com/problems/permutations)|backtracking|[solution](rustgym/src/leetcode/_46_permutations.rs)|
|470|[Implement Rand10() Using Rand7()](https://leetcode.com/problems/implement-rand10-using-rand7)|random rejection-sampling|[solution](rustgym/src/leetcode/_470_implement_rand10_using_rand7.rs)|
|473|[Matchsticks to Square](https://leetcode.com/problems/matchsticks-to-square)|depth-first-search|[solution](rustgym/src/leetcode/_473_matchsticks_to_square.rs)|
|474|[Ones and Zeroes](https://leetcode.com/problems/ones-and-zeroes)|dynamic-programming|[solution](rustgym/src/leetcode/_474_ones_and_zeroes.rs)|
|477|[Total Hamming Distance](https://leetcode.com/problems/total-hamming-distance)|bit-manipulation|[solution](rustgym/src/leetcode/_477_total_hamming_distance.rs)|
|478|[Generate Random Point in a Circle](https://leetcode.com/problems/generate-random-point-in-a-circle)|math random rejection-sampling|[solution](rustgym/src/leetcode/_478_generate_random_point_in_circle.rs)|
|47|[Permutations II](https://leetcode.com/problems/permutations-ii)|backtracking|[solution](rustgym/src/leetcode/_47_permutations_2.rs)|
|481|[Magical String](https://leetcode.com/problems/magical-string)||[solution](rustgym/src/leetcode/_481_magical_string.rs)|
|484|[Find Permutation](https://leetcode.com/problems/find-permutation)|greedy|[solution](rustgym/src/leetcode/_484_find_permutation.rs)|
|486|[Predict the Winner](https://leetcode.com/problems/predict-the-winner)|dynamic-programming minimax|[solution](rustgym/src/leetcode/_486_predict_the_winner.rs)|
|487|[Max Consecutive Ones II](https://leetcode.com/problems/max-consecutive-ones-ii)|two-pointers|[solution](rustgym/src/leetcode/_487_max_consecutive_ones_2.rs)|
|48|[Rotate Image](https://leetcode.com/problems/rotate-image)|array|[solution](rustgym/src/leetcode/_48_rotate_image.rs)|
|490|[The Maze](https://leetcode.com/problems/the-maze)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_490_the_maze.rs)|
|491|[Increasing Subsequences](https://leetcode.com/problems/increasing-subsequences)|depth-first-search|[solution](rustgym/src/leetcode/_491_increasing_subsequences.rs)|
|494|[Target Sum](https://leetcode.com/problems/target-sum)|dynamic-programming depth-first-search|[solution](rustgym/src/leetcode/_494_target_sum.rs)|
|495|[Teemo Attacking](https://leetcode.com/problems/teemo-attacking)|array|[solution](rustgym/src/leetcode/_495_teemo_attacking.rs)|
|497|[Random Point in Non-overlapping Rectangles](https://leetcode.com/problems/random-point-in-non-overlapping-rectangles)|binary-search random|[solution](rustgym/src/leetcode/_497_random_point_in_nonoverlapping_rectangles.rs)|
|498|[Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse)||[solution](rustgym/src/leetcode/_498_diagonal_traverse.rs)|
|49|[Group Anagrams](https://leetcode.com/problems/group-anagrams)|hash-table string|[solution](rustgym/src/leetcode/_49_group_anagrams.rs)|
|503|[Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii)|stack|[solution](rustgym/src/leetcode/_503_next_greater_element_2.rs)|
|505|[The Maze II](https://leetcode.com/problems/the-maze-ii)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_505_the_maze_2.rs)|
|508|[Most Frequent Subtree Sum](https://leetcode.com/problems/most-frequent-subtree-sum)|hash-table tree|[solution](rustgym/src/leetcode/_508_most_frequent_subtree_sum.rs)|
|50|[Pow(x, n)](https://leetcode.com/problems/powx-n)|math binary-search|[solution](rustgym/src/leetcode/_50_pow_x_n.rs)|
|513|[Find Bottom Left Tree Value](https://leetcode.com/problems/find-bottom-left-tree-value)|tree depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_513_find_bottom_left_tree_value.rs)|
|515|[Find Largest Value in Each Tree Row](https://leetcode.com/problems/find-largest-value-in-each-tree-row)|tree depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_515_find_largest_value_in_each_row.rs)|
|516|[Longest Palindromic Subsequence](https://leetcode.com/problems/longest-palindromic-subsequence)|dynamic-programming|[solution](rustgym/src/leetcode/_516_longest_palindromic_subsequence.rs)|
|518|[Coin Change 2](https://leetcode.com/problems/coin-change-2)||[solution](rustgym/src/leetcode/_518_coin_change_2.rs)|
|519|[Random Flip Matrix](https://leetcode.com/problems/random-flip-matrix)|random|[solution](rustgym/src/leetcode/_519_random_flip_matrix.rs)|
|522|[Longest Uncommon Subsequence II](https://leetcode.com/problems/longest-uncommon-subsequence-ii)|string|[solution](rustgym/src/leetcode/_522_longest_uncommon_subsequence_2.rs)|
|523|[Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum)|math dynamic-programming|[solution](rustgym/src/leetcode/_523_continuous_subarray_sum.rs)|
|524|[Longest Word in Dictionary through Deleting](https://leetcode.com/problems/longest-word-in-dictionary-through-deleting)|two-pointers sort|[solution](rustgym/src/leetcode/_524_longest_word_in_dictionary_through_deleting.rs)|
|525|[Contiguous Array](https://leetcode.com/problems/contiguous-array)|hash-table|[solution](rustgym/src/leetcode/_525_contiguous_array.rs)|
|526|[Beautiful Arrangement](https://leetcode.com/problems/beautiful-arrangement)|backtracking|[solution](rustgym/src/leetcode/_526_beautiful_arrangment.rs)|
|528|[Random Pick with Weight](https://leetcode.com/problems/random-pick-with-weight)|binary-search random|[solution](rustgym/src/leetcode/_528_random_pick_with_weight.rs)|
|529|[Minesweeper](https://leetcode.com/problems/minesweeper)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_529_minesweeper.rs)|
|531|[Lonely Pixel I](https://leetcode.com/problems/lonely-pixel-i)|array depth-first-search|[solution](rustgym/src/leetcode/_531_lonely_pixel_1.rs)|
|533|[Lonely Pixel II](https://leetcode.com/problems/lonely-pixel-ii)|array|[solution](rustgym/src/leetcode/_533_lonely_pixel_2.rs)|
|535|[Encode and Decode TinyURL](https://leetcode.com/problems/encode-and-decode-tinyurl)|hash-table math|[solution](rustgym/src/leetcode/_535_encode_and_decode_tiny_url.rs)|
|536|[Construct Binary Tree from String](https://leetcode.com/problems/construct-binary-tree-from-string)|string tree|[solution](rustgym/src/leetcode/_536_construct_binary_tree_from_string.rs)|
|537|[Complex Number Multiplication](https://leetcode.com/problems/complex-number-multiplication)|math string|[solution](rustgym/src/leetcode/_537_complex_number_multiplication.rs)|
|539|[Minimum Time Difference](https://leetcode.com/problems/minimum-time-difference)|string|[solution](rustgym/src/leetcode/_539_minimum_time_difference.rs)|
|540|[Single Element in a Sorted Array](https://leetcode.com/problems/single-element-in-a-sorted-array)||[solution](rustgym/src/leetcode/_540_single_element_in_a_sorted_array.rs)|
|542|[01 Matrix](https://leetcode.com/problems/01-matrix)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_542_01_matrix.rs)|
|544|[Output Contest Matches](https://leetcode.com/problems/output-contest-matches)|string recursion|[solution](rustgym/src/leetcode/_544_output_contest_matches.rs)|
|545|[Boundary of Binary Tree](https://leetcode.com/problems/boundary-of-binary-tree)|tree|[solution](rustgym/src/leetcode/_545_boundary_of_binary_tree.rs)|
|547|[Friend Circles](https://leetcode.com/problems/friend-circles)|depth-first-search union-find|[solution](rustgym/src/leetcode/_547_friend_circles.rs)|
|548|[Split Array with Equal Sum](https://leetcode.com/problems/split-array-with-equal-sum)|array|[solution](rustgym/src/leetcode/_548_split_array_with_equal_sum.rs)|
|549|[Binary Tree Longest Consecutive Sequence II](https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii)|tree|[solution](rustgym/src/leetcode/_549_binary_tree_longest_consecutive_sequence_2.rs)|
|54|[Spiral Matrix](https://leetcode.com/problems/spiral-matrix)|array|[solution](rustgym/src/leetcode/_54_spiral_matrix.rs)|
|553|[Optimal Division](https://leetcode.com/problems/optimal-division)|math string|[solution](rustgym/src/leetcode/_553_optimal_division.rs)|
|554|[Brick Wall](https://leetcode.com/problems/brick-wall)|hash-table|[solution](rustgym/src/leetcode/_554_brick_wall.rs)|
|555|[Split Concatenated Strings](https://leetcode.com/problems/split-concatenated-strings)|string|[solution](rustgym/src/leetcode/_555_split_concatenated_strings.rs)|
|556|[Next Greater Element III](https://leetcode.com/problems/next-greater-element-iii)|string|[solution](rustgym/src/leetcode/_556_next_greater_element_3.rs)|
|55|[Jump Game](https://leetcode.com/problems/jump-game)|array greedy|[solution](rustgym/src/leetcode/_55_jump_game.rs)|
|560|[Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k)|array hash-table|[solution](rustgym/src/leetcode/_560_subarray_sum_equals_k.rs)|
|562|[Longest Line of Consecutive One in Matrix](https://leetcode.com/problems/longest-line-of-consecutive-one-in-matrix)|array|[solution](rustgym/src/leetcode/_562_longest_line_of_consecutive_one_in_matrix.rs)|
|565|[Array Nesting](https://leetcode.com/problems/array-nesting)|array|[solution](rustgym/src/leetcode/_565_array_nesting.rs)|
|567|[Permutation in String](https://leetcode.com/problems/permutation-in-string)|two-pointers sliding-window|[solution](rustgym/src/leetcode/_567_permutation_in_string.rs)|
|56|[Merge Intervals](https://leetcode.com/problems/merge-intervals)|array sort|[solution](rustgym/src/leetcode/_56_merge_intervals.rs)|
|573|[Squirrel Simulation](https://leetcode.com/problems/squirrel-simulation)|math|[solution](rustgym/src/leetcode/_573_squirrel_simulation.rs)|
|576|[Out of Boundary Paths](https://leetcode.com/problems/out-of-boundary-paths)|dynamic-programming depth-first-search|[solution](rustgym/src/leetcode/_576_out_of_boundary_paths.rs)|
|582|[Kill Process](https://leetcode.com/problems/kill-process)|tree queue|[solution](rustgym/src/leetcode/_582_kill_process.rs)|
|583|[Delete Operation for Two Strings](https://leetcode.com/problems/delete-operation-for-two-strings)|string|[solution](rustgym/src/leetcode/_583_delete_operation_for_two_strings.rs)|
|592|[Fraction Addition and Subtraction](https://leetcode.com/problems/fraction-addition-and-subtraction)|math|[solution](rustgym/src/leetcode/_592_fraction_addition_and_subtraction.rs)|
|593|[Valid Square](https://leetcode.com/problems/valid-square)|math|[solution](rustgym/src/leetcode/_593_valid_square.rs)|
|59|[Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-ii)|array|[solution](rustgym/src/leetcode/_59_spiral_matrix_2.rs)|
|5|[Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring)|string dynamic-programming|[solution](rustgym/src/leetcode/_5_longest_palindromic_substring.rs)|
|609|[Find Duplicate File in System](https://leetcode.com/problems/find-duplicate-file-in-system)|hash-table string|[solution](rustgym/src/leetcode/_609_find_duplicate_file_in_system.rs)|
|611|[Valid Triangle Number](https://leetcode.com/problems/valid-triangle-number)|array|[solution](rustgym/src/leetcode/_611_valid_triangle_number.rs)|
|616|[Add Bold Tag in String](https://leetcode.com/problems/add-bold-tag-in-string)|string|[solution](rustgym/src/leetcode/_616_add_bold_tag_in_string.rs)|
|61|[Rotate List](https://leetcode.com/problems/rotate-list)|linked-list two-pointers|[solution](rustgym/src/leetcode/_61_rotate_list.rs)|
|621|[Task Scheduler](https://leetcode.com/problems/task-scheduler)|array greedy queue|[solution](rustgym/src/leetcode/_621_task_scheduler.rs)|
|622|[Design Circular Queue](https://leetcode.com/problems/design-circular-queue)|design queue|[solution](rustgym/src/leetcode/_622_design_circular_queue.rs)|
|623|[Add One Row to Tree](https://leetcode.com/problems/add-one-row-to-tree)|tree|[solution](rustgym/src/leetcode/_623_add_one_row_to_tree.rs)|
|625|[Minimum Factorization](https://leetcode.com/problems/minimum-factorization)|math recursion|[solution](rustgym/src/leetcode/_625_minimum_factorization.rs)|
|62|[Unique Paths](https://leetcode.com/problems/unique-paths)|array dynamic-programming|[solution](rustgym/src/leetcode/_62_unique_paths.rs)|
|634|[Find the Derangement of An Array](https://leetcode.com/problems/find-the-derangement-of-an-array)|math|[solution](rustgym/src/leetcode/_634_find_the_derangement_of_an_array.rs)|
|635|[Design Log Storage System](https://leetcode.com/problems/design-log-storage-system)|string design|[solution](rustgym/src/leetcode/_635_design_log_storage_system.rs)|
|636|[Exclusive Time of Functions](https://leetcode.com/problems/exclusive-time-of-functions)|stack|[solution](rustgym/src/leetcode/_636_exclusive_time_of_functions.rs)|
|638|[Shopping Offers](https://leetcode.com/problems/shopping-offers)|dynamic-programming depth-first-search|[solution](rustgym/src/leetcode/_638_shopping_offers.rs)|
|63|[Unique Paths II](https://leetcode.com/problems/unique-paths-ii)|array dynamic-programming|[solution](rustgym/src/leetcode/_63_unique_paths_2.rs)|
|640|[Solve the Equation](https://leetcode.com/problems/solve-the-equation)|math|[solution](rustgym/src/leetcode/_640_solve_the_equation.rs)|
|641|[Design Circular Deque](https://leetcode.com/problems/design-circular-deque)|design queue|[solution](rustgym/src/leetcode/_641_design_circular_deque.rs)|
|646|[Maximum Length of Pair Chain](https://leetcode.com/problems/maximum-length-of-pair-chain)|dynamic-programming|[solution](rustgym/src/leetcode/_646_maximum_length_of_pair_chain.rs)|
|647|[Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings)|string dynamic-programming|[solution](rustgym/src/leetcode/_647_palindromic_substrings.rs)|
|648|[Replace Words](https://leetcode.com/problems/replace-words)|hash-table trie|[solution](rustgym/src/leetcode/_648_replace_words.rs)|
|649|[Dota2 Senate](https://leetcode.com/problems/dota2-senate)|greedy|[solution](rustgym/src/leetcode/_649_dota2_senate.rs)|
|64|[Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum)|array dynamic-programming|[solution](rustgym/src/leetcode/_64_minimum_path_sum.rs)|
|650|[2 Keys Keyboard](https://leetcode.com/problems/2-keys-keyboard)|dynamic-programming|[solution](rustgym/src/leetcode/_650_2_keys_keyboard.rs)|
|651|[4 Keys Keyboard](https://leetcode.com/problems/4-keys-keyboard)|math dynamic-programming greedy|[solution](rustgym/src/leetcode/_651_4_keys_keyboard.rs)|
|652|[Find Duplicate Subtrees](https://leetcode.com/problems/find-duplicate-subtrees)|tree|[solution](rustgym/src/leetcode/_652_find_duplicate_subtrees.rs)|
|654|[Maximum Binary Tree](https://leetcode.com/problems/maximum-binary-tree)|tree|[solution](rustgym/src/leetcode/_654_maximum_binary_tree.rs)|
|655|[Print Binary Tree](https://leetcode.com/problems/print-binary-tree)|tree|[solution](rustgym/src/leetcode/_655_print_binary_tree.rs)|
|658|[Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements)|binary-search|[solution](rustgym/src/leetcode/_658_find_k_cloest_elements.rs)|
|659|[Split Array into Consecutive Subsequences](https://leetcode.com/problems/split-array-into-consecutive-subsequences)|heap greedy|[solution](rustgym/src/leetcode/_659_split_array_into_consecutive_subsequences.rs)|
|662|[Maximum Width of Binary Tree](https://leetcode.com/problems/maximum-width-of-binary-tree)|tree|[solution](rustgym/src/leetcode/_662_maximum_width_of_binary_tree.rs)|
|663|[Equal Tree Partition](https://leetcode.com/problems/equal-tree-partition)|tree|[solution](rustgym/src/leetcode/_663_equal_tree_partition.rs)|
|666|[Path Sum IV](https://leetcode.com/problems/path-sum-iv)|tree|[solution](rustgym/src/leetcode/_666_path_sum_4.rs)|
|667|[Beautiful Arrangement II](https://leetcode.com/problems/beautiful-arrangement-ii)|array|[solution](rustgym/src/leetcode/_667_beautiful_arrangement_2.rs)|
|670|[Maximum Swap](https://leetcode.com/problems/maximum-swap)|array math|[solution](rustgym/src/leetcode/_670_maximum_swap.rs)|
|672|[Bulb Switcher II](https://leetcode.com/problems/bulb-switcher-ii)|math|[solution](rustgym/src/leetcode/_672_bulb_switcher_2.rs)|
|676|[Implement Magic Dictionary](https://leetcode.com/problems/implement-magic-dictionary)|hash-table trie|[solution](rustgym/src/leetcode/_676_implement_magic_dictionary.rs)|
|677|[Map Sum Pairs](https://leetcode.com/problems/map-sum-pairs)|trie|[solution](rustgym/src/leetcode/_677_map_sum_pairs.rs)|
|678|[Valid Parenthesis String](https://leetcode.com/problems/valid-parenthesis-string)|string|[solution](rustgym/src/leetcode/_678_valid_parenthesis_string.rs)|
|681|[Next Closest Time](https://leetcode.com/problems/next-closest-time)|string|[solution](rustgym/src/leetcode/_681_next_closest_time.rs)|
|684|[Redundant Connection](https://leetcode.com/problems/redundant-connection)|tree union-find graph|[solution](rustgym/src/leetcode/_684_redundant_connection.rs)|
|688|[Knight Probability in Chessboard](https://leetcode.com/problems/knight-probability-in-chessboard)|dynamic-programming|[solution](rustgym/src/leetcode/_688_knight_probability_in_chessboard.rs)|
|692|[Top K Frequent Words](https://leetcode.com/problems/top-k-frequent-words)|hash-table heap trie|[solution](rustgym/src/leetcode/_692_top_k_frequent_words.rs)|
|694|[Number of Distinct Islands](https://leetcode.com/problems/number-of-distinct-islands)|hash-table depth-first-search|[solution](rustgym/src/leetcode/_694_number_of_distinct_islands.rs)|
|695|[Max Area of Island](https://leetcode.com/problems/max-area-of-island)|array depth-first-search|[solution](rustgym/src/leetcode/_695_max_area_of_island.rs)|
|698|[Partition to K Equal Sum Subsets](https://leetcode.com/problems/partition-to-k-equal-sum-subsets)|dynamic-programming recursion|[solution](rustgym/src/leetcode/_698_partition_to_k_equal_sum_subsets.rs)|
|6|[ZigZag Conversion](https://leetcode.com/problems/zigzag-conversion)|string|[solution](rustgym/src/leetcode/_6_zigzag_conversion.rs)|
|701|[Insert into a Binary Search Tree](https://leetcode.com/problems/insert-into-a-binary-search-tree)|tree|[solution](rustgym/src/leetcode/_701_insert_into_a_binary_search_tree.rs)|
|707|[Design Linked List](https://leetcode.com/problems/design-linked-list)|linked-list design|[solution](rustgym/src/leetcode/_707_design_linked_list.rs)|
|712|[Minimum ASCII Delete Sum for Two Strings](https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings)|dynamic-programming|[solution](rustgym/src/leetcode/_712_minimum_ascii_delete_sum_for_two_string.rs)|
|713|[Subarray Product Less Than K](https://leetcode.com/problems/subarray-product-less-than-k)|array two-pointers|[solution](rustgym/src/leetcode/_713_subarray_product_less_than_k.rs)|
|714|[Best Time to Buy and Sell Stock with Transaction Fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee)|array dynamic-programming greedy|[solution](rustgym/src/leetcode/_714_best_time_to_buy_and_sell_stock_with_transaction_fee.rs)|
|718|[Maximum Length of Repeated Subarray](https://leetcode.com/problems/maximum-length-of-repeated-subarray)|array hash-table binary-search dynamic-programming|[solution](rustgym/src/leetcode/_718_maximum_length_of_repeated_subarray.rs)|
|71|[Simplify Path](https://leetcode.com/problems/simplify-path)|string stack|[solution](rustgym/src/leetcode/_71_simplify_path.rs)|
|721|[Accounts Merge](https://leetcode.com/problems/accounts-merge)|depth-first-search union-find|[solution](rustgym/src/leetcode/_721_accounts_merge.rs)|
|722|[Remove Comments](https://leetcode.com/problems/remove-comments)|string|[solution](rustgym/src/leetcode/_722_remove_comments.rs)|
|723|[Candy Crush](https://leetcode.com/problems/candy-crush)|array two-pointers|[solution](rustgym/src/leetcode/_723_candy_crush.rs)|
|725|[Split Linked List in Parts](https://leetcode.com/problems/split-linked-list-in-parts)|linked-list|[solution](rustgym/src/leetcode/_725_split_linked_list_in_parts.rs)|
|729|[My Calendar I](https://leetcode.com/problems/my-calendar-i)|array|[solution](rustgym/src/leetcode/_729_my_calendar_1.rs)|
|731|[My Calendar II](https://leetcode.com/problems/my-calendar-ii)|ordered-map|[solution](rustgym/src/leetcode/_731_my_calendar_2.rs)|
|735|[Asteroid Collision](https://leetcode.com/problems/asteroid-collision)|stack|[solution](rustgym/src/leetcode/_735_asteroid_collision.rs)|
|737|[Sentence Similarity II](https://leetcode.com/problems/sentence-similarity-ii)|depth-first-search union-find|[solution](rustgym/src/leetcode/_737_sentence_similarity_2.rs)|
|738|[Monotone Increasing Digits](https://leetcode.com/problems/monotone-increasing-digits)|greedy|[solution](rustgym/src/leetcode/_738_monotone_increasing_digits.rs)|
|739|[Daily Temperatures](https://leetcode.com/problems/daily-temperatures)|hash-table stack|[solution](rustgym/src/leetcode/_739_daily_temperatures.rs)|
|73|[Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes)|array|[solution](rustgym/src/leetcode/_73_set_matrix_zeroes.rs)|
|740|[Delete and Earn](https://leetcode.com/problems/delete-and-earn)|dynamic-programming|[solution](rustgym/src/leetcode/_740_delete_and_earn.rs)|
|742|[Closest Leaf in a Binary Tree](https://leetcode.com/problems/closest-leaf-in-a-binary-tree)|tree|[solution](rustgym/src/leetcode/_742_closest_leaf_in_binary_tree.rs)|
|743|[Network Delay Time](https://leetcode.com/problems/network-delay-time)|heap depth-first-search breadth-first-search graph|[solution](rustgym/src/leetcode/_743_network_delay_time.rs)|
|74|[Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix)|array binary-search|[solution](rustgym/src/leetcode/_74_search_a_2d_matrix.rs)|
|750|[Number Of Corner Rectangles](https://leetcode.com/problems/number-of-corner-rectangles)|dynamic-programming|[solution](rustgym/src/leetcode/_750_number_of_corner_rectangles.rs)|
|751|[IP to CIDR](https://leetcode.com/problems/ip-to-cidr)|bit-manipulation|[solution](rustgym/src/leetcode/_751_ip_to_cidr.rs)|
|752|[Open the Lock](https://leetcode.com/problems/open-the-lock)|breadth-first-search|[solution](rustgym/src/leetcode/_752_open_the_lock.rs)|
|754|[Reach a Number](https://leetcode.com/problems/reach-a-number)|math|[solution](rustgym/src/leetcode/_754_reach_a_number.rs)|
|755|[Pour Water](https://leetcode.com/problems/pour-water)|array|[solution](rustgym/src/leetcode/_755_pour_water.rs)|
|756|[Pyramid Transition Matrix](https://leetcode.com/problems/pyramid-transition-matrix)|bit-manipulation depth-first-search|[solution](rustgym/src/leetcode/_756_pyramid_transition_matrix.rs)|
|75|[Sort Colors](https://leetcode.com/problems/sort-colors)|array two-pointers sort|[solution](rustgym/src/leetcode/_75_sort_colors.rs)|
|763|[Partition Labels](https://leetcode.com/problems/partition-labels)|two-pointers greedy|[solution](rustgym/src/leetcode/_763_partition_labels.rs)|
|764|[Largest Plus Sign](https://leetcode.com/problems/largest-plus-sign)|dynamic-programming|[solution](rustgym/src/leetcode/_764_largest_plus_sign.rs)|
|769|[Max Chunks To Make Sorted](https://leetcode.com/problems/max-chunks-to-make-sorted)|array|[solution](rustgym/src/leetcode/_769_max_chunks_to_make_sorted.rs)|
|775|[Global and Local Inversions](https://leetcode.com/problems/global-and-local-inversions)|array math|[solution](rustgym/src/leetcode/_775_global_and_local_inversions.rs)|
|776|[Split BST](https://leetcode.com/problems/split-bst)|tree recursion|[solution](rustgym/src/leetcode/_776_split_bst.rs)|
|777|[Swap Adjacent in LR String](https://leetcode.com/problems/swap-adjacent-in-lr-string)|brainteaser|[solution](rustgym/src/leetcode/_777_swap_adjacent_in_lr_string.rs)|
|779|[K-th Symbol in Grammar](https://leetcode.com/problems/k-th-symbol-in-grammar)|recursion|[solution](rustgym/src/leetcode/_779_kth_symbol_in_grammar.rs)|
|77|[Combinations](https://leetcode.com/problems/combinations)|backtracking|[solution](rustgym/src/leetcode/_77_combinations.rs)|
|781|[Rabbits in Forest](https://leetcode.com/problems/rabbits-in-forest)|hash-table math|[solution](rustgym/src/leetcode/_781_rabbits_in_forest.rs)|
|784|[Letter Case Permutation](https://leetcode.com/problems/letter-case-permutation)|backtracking bit-manipulation|[solution](rustgym/src/leetcode/_784_letter_case_permutation.rs)|
|785|[Is Graph Bipartite?](https://leetcode.com/problems/is-graph-bipartite)|depth-first-search breadth-first-search graph|[solution](rustgym/src/leetcode/_785_is_graph_bipartite.rs)|
|787|[Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops)|dynamic-programming heap breadth-first-search|[solution](rustgym/src/leetcode/_787_cheapest_flights_within_k_stops.rs)|
|789|[Escape The Ghosts](https://leetcode.com/problems/escape-the-ghosts)|math|[solution](rustgym/src/leetcode/_789_escape_the_ghosts.rs)|
|78|[Subsets](https://leetcode.com/problems/subsets)|array backtracking bit-manipulation|[solution](rustgym/src/leetcode/_78_subsets.rs)|
|790|[Domino and Tromino Tiling](https://leetcode.com/problems/domino-and-tromino-tiling)|dynamic-programming|[solution](rustgym/src/leetcode/_790_domino_and_tromino_tiling.rs)|
|791|[Custom Sort String](https://leetcode.com/problems/custom-sort-string)|string|[solution](rustgym/src/leetcode/_791_custom_sort_string.rs)|
|792|[Number of Matching Subsequences](https://leetcode.com/problems/number-of-matching-subsequences)|array|[solution](rustgym/src/leetcode/_792_number_of_matching_subsequences.rs)|
|794|[Valid Tic-Tac-Toe State](https://leetcode.com/problems/valid-tic-tac-toe-state)|math recursion|[solution](rustgym/src/leetcode/_794_valid_tic_tac_toe_state.rs)|
|795|[Number of Subarrays with Bounded Maximum](https://leetcode.com/problems/number-of-subarrays-with-bounded-maximum)|array|[solution](rustgym/src/leetcode/_795_number_of_subarrays_with_bounded_maximum.rs)|
|797|[All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target)||[solution](rustgym/src/leetcode/_797_all_paths_from_source_to_target.rs)|
|799|[Champagne Tower](https://leetcode.com/problems/champagne-tower)||[solution](rustgym/src/leetcode/_799_champagne_tower.rs)|
|79|[Word Search](https://leetcode.com/problems/word-search)|array backtracking|[solution](rustgym/src/leetcode/_79_word_search.rs)|
|801|[Minimum Swaps To Make Sequences Increasing](https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing)|dynamic-programming|[solution](rustgym/src/leetcode/_801_minimum_swaps_to_make_sequences_increasing.rs)|
|802|[Find Eventual Safe States](https://leetcode.com/problems/find-eventual-safe-states)|depth-first-search graph|[solution](rustgym/src/leetcode/_802_find_eventual_safe_states.rs)|
|807|[Max Increase to Keep City Skyline](https://leetcode.com/problems/max-increase-to-keep-city-skyline)||[solution](rustgym/src/leetcode/_807_max_increase_to_keep_city_skyline.rs)|
|808|[Soup Servings](https://leetcode.com/problems/soup-servings)|dynamic-programming|[solution](rustgym/src/leetcode/_808_soup_servings.rs)|
|809|[Expressive Words](https://leetcode.com/problems/expressive-words)|string|[solution](rustgym/src/leetcode/_809_expressive_words.rs)|
|80|[Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii)|array two-pointers|[solution](rustgym/src/leetcode/_80_remove_duplicates_from_sorted_array_2.rs)|
|813|[Largest Sum of Averages](https://leetcode.com/problems/largest-sum-of-averages)|dynamic-programming|[solution](rustgym/src/leetcode/_813_largest_sum_of_averages.rs)|
|814|[Binary Tree Pruning](https://leetcode.com/problems/binary-tree-pruning)|tree|[solution](rustgym/src/leetcode/_814_binary_tree_pruning.rs)|
|816|[Ambiguous Coordinates](https://leetcode.com/problems/ambiguous-coordinates)|string|[solution](rustgym/src/leetcode/_816_ambiguous_coordinates.rs)|
|817|[Linked List Components](https://leetcode.com/problems/linked-list-components)|linked-list|[solution](rustgym/src/leetcode/_817_linked_list_components.rs)|
|81|[Search in Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii)|array binary-search|[solution](rustgym/src/leetcode/_81_search_in_rotated_sorted_array_2.rs)|
|820|[Short Encoding of Words](https://leetcode.com/problems/short-encoding-of-words)||[solution](rustgym/src/leetcode/_820_short_encoding_of_words.rs)|
|822|[Card Flipping Game](https://leetcode.com/problems/card-flipping-game)||[solution](rustgym/src/leetcode/_822_card_flipping_game.rs)|
|823|[Binary Trees With Factors](https://leetcode.com/problems/binary-trees-with-factors)||[solution](rustgym/src/leetcode/_823_binary_trees_with_factors.rs)|
|825|[Friends Of Appropriate Ages](https://leetcode.com/problems/friends-of-appropriate-ages)|array|[solution](rustgym/src/leetcode/_825_friends_of_appropriate_ages.rs)|
|826|[Most Profit Assigning Work](https://leetcode.com/problems/most-profit-assigning-work)|two-pointers|[solution](rustgym/src/leetcode/_826_most_profix_assigning_work.rs)|
|82|[Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii)|linked-list|[solution](rustgym/src/leetcode/_82_remove_duplicates_from_sorted_list_2.rs)|
|831|[Masking Personal Information](https://leetcode.com/problems/masking-personal-information)|string|[solution](rustgym/src/leetcode/_831_masking_personal_information.rs)|
|833|[Find And Replace in String](https://leetcode.com/problems/find-and-replace-in-string)|string|[solution](rustgym/src/leetcode/_833_find_and_replace_in_string.rs)|
|835|[Image Overlap](https://leetcode.com/problems/image-overlap)|array|[solution](rustgym/src/leetcode/_835_image_overlap.rs)|
|837|[New 21 Game](https://leetcode.com/problems/new-21-game)|dynamic-programming|[solution](rustgym/src/leetcode/_837_new_21_game.rs)|
|838|[Push Dominoes](https://leetcode.com/problems/push-dominoes)|two-pointers dynamic-programming|[solution](rustgym/src/leetcode/_838_push_dominoes.rs)|
|841|[Keys and Rooms](https://leetcode.com/problems/keys-and-rooms)|depth-first-search graph|[solution](rustgym/src/leetcode/_841_keys_and_rooms.rs)|
|842|[Split Array into Fibonacci Sequence](https://leetcode.com/problems/split-array-into-fibonacci-sequence)|string backtracking greedy|[solution](rustgym/src/leetcode/_842_split_array_into_fibonacci_sequence.rs)|
|845|[Longest Mountain in Array](https://leetcode.com/problems/longest-mountain-in-array)|two-pointers|[solution](rustgym/src/leetcode/_845_longest_mountain_in_array.rs)|
|846|[Hand of Straights](https://leetcode.com/problems/hand-of-straights)|ordered-map|[solution](rustgym/src/leetcode/_846_hand_of_straights.rs)|
|848|[Shifting Letters](https://leetcode.com/problems/shifting-letters)|string|[solution](rustgym/src/leetcode/_848_shifting_letters.rs)|
|851|[Loud and Rich](https://leetcode.com/problems/loud-and-rich)|depth-first-search|[solution](rustgym/src/leetcode/_851_loud_and_rich.rs)|
|853|[Car Fleet](https://leetcode.com/problems/car-fleet)|sort|[solution](rustgym/src/leetcode/_853_car_fleet.rs)|
|855|[Exam Room](https://leetcode.com/problems/exam-room)|ordered-map|[solution](rustgym/src/leetcode/_855_exam_room.rs)|
|856|[Score of Parentheses](https://leetcode.com/problems/score-of-parentheses)|string stack|[solution](rustgym/src/leetcode/_856_score_of_parentheses.rs)|
|858|[Mirror Reflection](https://leetcode.com/problems/mirror-reflection)|math|[solution](rustgym/src/leetcode/_858_mirror_reflection.rs)|
|861|[Score After Flipping Matrix](https://leetcode.com/problems/score-after-flipping-matrix)|greedy|[solution](rustgym/src/leetcode/_861_score_after_flipping_matrix.rs)|
|865|[Smallest Subtree with all the Deepest Nodes](https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes)|tree|[solution](rustgym/src/leetcode/_865_smallest_subtree_with_all_the_deepest_nodes.rs)|
|866|[Prime Palindrome](https://leetcode.com/problems/prime-palindrome)|math|[solution](rustgym/src/leetcode/_866_prime_palindrome.rs)|
|869|[Reordered Power of 2](https://leetcode.com/problems/reordered-power-of-2)|math|[solution](rustgym/src/leetcode/_869_reordered_power_of_2.rs)|
|86|[Partition List](https://leetcode.com/problems/partition-list)|linked-list two-pointers|[solution](rustgym/src/leetcode/_86_partition_list.rs)|
|870|[Advantage Shuffle](https://leetcode.com/problems/advantage-shuffle)|array greedy|[solution](rustgym/src/leetcode/_870_advantage_shuffle.rs)|
|873|[Length of Longest Fibonacci Subsequence](https://leetcode.com/problems/length-of-longest-fibonacci-subsequence)|array dynamic-programming|[solution](rustgym/src/leetcode/_873_length_of_longest_fibonacci_subsequence.rs)|
|875|[Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas)|binary-search|[solution](rustgym/src/leetcode/_875_koko_eating_bananas.rs)|
|877|[Stone Game](https://leetcode.com/problems/stone-game)|math dynamic-programming minimax|[solution](rustgym/src/leetcode/_877_stone_game.rs)|
|880|[Decoded String at Index](https://leetcode.com/problems/decoded-string-at-index)|stack|[solution](rustgym/src/leetcode/_880_decoded_string_at_index.rs)|
|881|[Boats to Save People](https://leetcode.com/problems/boats-to-save-people)|two-pointers greedy|[solution](rustgym/src/leetcode/_881_boats_to_save_people.rs)|
|885|[Spiral Matrix III](https://leetcode.com/problems/spiral-matrix-iii)|math|[solution](rustgym/src/leetcode/_885_spiral_matrix_3.rs)|
|886|[Possible Bipartition](https://leetcode.com/problems/possible-bipartition)|depth-first-search graph|[solution](rustgym/src/leetcode/_886_possible_bipartition.rs)|
|889|[Construct Binary Tree from Preorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal)|tree|[solution](rustgym/src/leetcode/_889_construct_binary_tree_from_preorder_and_postorder_traversal.rs)|
|890|[Find and Replace Pattern](https://leetcode.com/problems/find-and-replace-pattern)|string|[solution](rustgym/src/leetcode/_890_find_and_replace_pattern.rs)|
|894|[All Possible Full Binary Trees](https://leetcode.com/problems/all-possible-full-binary-trees)|tree recursion|[solution](rustgym/src/leetcode/_894_all_possible_full_binary_trees.rs)|
|898|[Bitwise ORs of Subarrays](https://leetcode.com/problems/bitwise-ors-of-subarrays)|dynamic-programming bit-manipulation|[solution](rustgym/src/leetcode/_898_bitwise_ors_of_subarrays.rs)|
|89|[Gray Code](https://leetcode.com/problems/gray-code)|backtracking|[solution](rustgym/src/leetcode/_89_gray_code.rs)|
|8|[String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi)|math string|[solution](rustgym/src/leetcode/_8_string_to_integer.rs)|
|900|[RLE Iterator](https://leetcode.com/problems/rle-iterator)|array|[solution](rustgym/src/leetcode/_900_rle_iterator.rs)|
|901|[Online Stock Span](https://leetcode.com/problems/online-stock-span)|stack|[solution](rustgym/src/leetcode/_901_online_stock_span.rs)|
|904|[Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets)|two-pointers|[solution](rustgym/src/leetcode/_904_fruit_into_baskets.rs)|
|907|[Sum of Subarray Minimums](https://leetcode.com/problems/sum-of-subarray-minimums)|array stack|[solution](rustgym/src/leetcode/_907_sum_of_subarray_minimums.rs)|
|909|[Snakes and Ladders](https://leetcode.com/problems/snakes-and-ladders)|breadth-first-search|[solution](rustgym/src/leetcode/_909_snakes_and_ladders.rs)|
|90|[Subsets II](https://leetcode.com/problems/subsets-ii)|array backtracking|[solution](rustgym/src/leetcode/_90_subsets_2.rs)|
|910|[Smallest Range II](https://leetcode.com/problems/smallest-range-ii)|math greedy|[solution](rustgym/src/leetcode/_910_smallest_range_2.rs)|
|911|[Online Election](https://leetcode.com/problems/online-election)|binary-search|[solution](rustgym/src/leetcode/_911_online_election.rs)|
|912|[Sort an Array](https://leetcode.com/problems/sort-an-array)||[solution](rustgym/src/leetcode/_912_sort_an_array.rs)|
|915|[Partition Array into Disjoint Intervals](https://leetcode.com/problems/partition-array-into-disjoint-intervals)|array|[solution](rustgym/src/leetcode/_915_partition_array_into_disjoint_intervals.rs)|
|916|[Word Subsets](https://leetcode.com/problems/word-subsets)|string|[solution](rustgym/src/leetcode/_916_word_subsets.rs)|
|918|[Maximum Sum Circular Subarray](https://leetcode.com/problems/maximum-sum-circular-subarray)|array|[solution](rustgym/src/leetcode/_918_maximum_sum_circular_subarray.rs)|
|919|[Complete Binary Tree Inserter](https://leetcode.com/problems/complete-binary-tree-inserter)|tree|[solution](rustgym/src/leetcode/_919_complete_binary_tree_inserter.rs)|
|91|[Decode Ways](https://leetcode.com/problems/decode-ways)|string dynamic-programming|[solution](rustgym/src/leetcode/_91_decode_ways.rs)|
|921|[Minimum Add to Make Parentheses Valid](https://leetcode.com/problems/minimum-add-to-make-parentheses-valid)|stack greedy|[solution](rustgym/src/leetcode/_921_minimum_add_to_make_parentheses_valid.rs)|
|923|[3Sum With Multiplicity](https://leetcode.com/problems/3sum-with-multiplicity)|two-pointers|[solution](rustgym/src/leetcode/_923_3sum_with_multiplicity.rs)|
|926|[Flip String to Monotone Increasing](https://leetcode.com/problems/flip-string-to-monotone-increasing)|array|[solution](rustgym/src/leetcode/_926_flip_string_to_monotone_increasing.rs)|
|92|[Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii)|linked-list|[solution](rustgym/src/leetcode/_92_reverse_linked_list_2.rs)|
|930|[Binary Subarrays With Sum](https://leetcode.com/problems/binary-subarrays-with-sum)|hash-table two-pointers|[solution](rustgym/src/leetcode/_930_binary_subarrays_with_sum.rs)|
|931|[Minimum Falling Path Sum](https://leetcode.com/problems/minimum-falling-path-sum)|dynamic-programming|[solution](rustgym/src/leetcode/_931_minimum_falling_path_sum.rs)|
|932|[Beautiful Array](https://leetcode.com/problems/beautiful-array)|divide-and-conquer|[solution](rustgym/src/leetcode/_932_beautiful_array.rs)|
|934|[Shortest Bridge](https://leetcode.com/problems/shortest-bridge)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_934_shortest_bridge.rs)|
|935|[Knight Dialer](https://leetcode.com/problems/knight-dialer)|dynamic-programming|[solution](rustgym/src/leetcode/_935_knight_dialer.rs)|
|939|[Minimum Area Rectangle](https://leetcode.com/problems/minimum-area-rectangle)|hash-table|[solution](rustgym/src/leetcode/_939_minimum_area_rectangle.rs)|
|93|[Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses)|string backtracking|[solution](rustgym/src/leetcode/_93_restore_ip_addresses.rs)|
|945|[Minimum Increment to Make Array Unique](https://leetcode.com/problems/minimum-increment-to-make-array-unique)|array|[solution](rustgym/src/leetcode/_945_minimum_increment_to_make_array_unique.rs)|
|946|[Validate Stack Sequences](https://leetcode.com/problems/validate-stack-sequences)|stack|[solution](rustgym/src/leetcode/_946_validate_stack_sequences.rs)|
|947|[Most Stones Removed with Same Row or Column](https://leetcode.com/problems/most-stones-removed-with-same-row-or-column)|depth-first-search union-find|[solution](rustgym/src/leetcode/_947_most_stones_removed_with_same_row_or_column.rs)|
|948|[Bag of Tokens](https://leetcode.com/problems/bag-of-tokens)|greedy|[solution](rustgym/src/leetcode/_948_bag_of_tokens.rs)|
|94|[Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal)|hash-table stack tree|[solution](rustgym/src/leetcode/_94_binary_tree_inorder_traversal.rs)|
|950|[Reveal Cards In Increasing Order](https://leetcode.com/problems/reveal-cards-in-increasing-order)|array|[solution](rustgym/src/leetcode/_950_reveal_cards_in_increasing_order.rs)|
|951|[Flip Equivalent Binary Trees](https://leetcode.com/problems/flip-equivalent-binary-trees)|tree|[solution](rustgym/src/leetcode/_951_flip_equivalent_binary_trees.rs)|
|954|[Array of Doubled Pairs](https://leetcode.com/problems/array-of-doubled-pairs)|array hash-table|[solution](rustgym/src/leetcode/_954_array_of_doubled_pairs.rs)|
|955|[Delete Columns to Make Sorted II](https://leetcode.com/problems/delete-columns-to-make-sorted-ii)|greedy|[solution](rustgym/src/leetcode/_955_delete_columns_to_make_sorted_2.rs)|
|957|[Prison Cells After N Days](https://leetcode.com/problems/prison-cells-after-n-days)|hash-table|[solution](rustgym/src/leetcode/_957_prison_cells_after_n_days.rs)|
|958|[Check Completeness of a Binary Tree](https://leetcode.com/problems/check-completeness-of-a-binary-tree)|tree|[solution](rustgym/src/leetcode/_958_check_completeness_of_a_binary_tree.rs)|
|959|[Regions Cut By Slashes](https://leetcode.com/problems/regions-cut-by-slashes)|depth-first-search union-find graph|[solution](rustgym/src/leetcode/_959_regions_cut_by_slashes.rs)|
|95|[Unique Binary Search Trees II](https://leetcode.com/problems/unique-binary-search-trees-ii)|dynamic-programming tree|[solution](rustgym/src/leetcode/_95_unique_binary_search_trees_2.rs)|
|962|[Maximum Width Ramp](https://leetcode.com/problems/maximum-width-ramp)|array|[solution](rustgym/src/leetcode/_962_maximum_with_ramp.rs)|
|963|[Minimum Area Rectangle II](https://leetcode.com/problems/minimum-area-rectangle-ii)|math geometry|[solution](rustgym/src/leetcode/_963_minimum_area_rectangle_2.rs)|
|966|[Vowel Spellchecker](https://leetcode.com/problems/vowel-spellchecker)|hash-table string|[solution](rustgym/src/leetcode/_966_vowel_spellchecker.rs)|
|967|[Numbers With Same Consecutive Differences](https://leetcode.com/problems/numbers-with-same-consecutive-differences)|dynamic-programming|[solution](rustgym/src/leetcode/_967_numbers_with_same_consecutive_differences.rs)|
|969|[Pancake Sorting](https://leetcode.com/problems/pancake-sorting)|array sort|[solution](rustgym/src/leetcode/_969_pancake_sorting.rs)|
|96|[Unique Binary Search Trees](https://leetcode.com/problems/unique-binary-search-trees)|dynamic-programming tree|[solution](rustgym/src/leetcode/_96_unique_binary_search_trees.rs)|
|971|[Flip Binary Tree To Match Preorder Traversal](https://leetcode.com/problems/flip-binary-tree-to-match-preorder-traversal)|tree depth-first-search|[solution](rustgym/src/leetcode/_971_flip_binary_tree_to_match_preorder_traversal.rs)|
|973|[K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin)|divide-and-conquer heap sort|[solution](rustgym/src/leetcode/_973_k_closest_points_to_origin.rs)|
|974|[Subarray Sums Divisible by K](https://leetcode.com/problems/subarray-sums-divisible-by-k)|array hash-table|[solution](rustgym/src/leetcode/_974_subarray_sums_divisible_by_k.rs)|
|978|[Longest Turbulent Subarray](https://leetcode.com/problems/longest-turbulent-subarray)|array dynamic-programming sliding-window|[solution](rustgym/src/leetcode/_978_longest_turbulent_subarray.rs)|
|979|[Distribute Coins in Binary Tree](https://leetcode.com/problems/distribute-coins-in-binary-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_979_distribute_coins_in_binary_tree.rs)|
|981|[Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store)|hash-table binary-search|[solution](rustgym/src/leetcode/_981_time_based_key_value_store.rs)|
|983|[Minimum Cost For Tickets](https://leetcode.com/problems/minimum-cost-for-tickets)|dynamic-programming|[solution](rustgym/src/leetcode/_983_minimum_cost_for_tickets.rs)|
|984|[String Without AAA or BBB](https://leetcode.com/problems/string-without-aaa-or-bbb)|greedy|[solution](rustgym/src/leetcode/_984_string_without_aaa_or_bbb.rs)|
|986|[Interval List Intersections](https://leetcode.com/problems/interval-list-intersections)|two-pointers|[solution](rustgym/src/leetcode/_986_interval_list_intersections.rs)|
|987|[Vertical Order Traversal of a Binary Tree](https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree)|hash-table tree|[solution](rustgym/src/leetcode/_987_vertical_order_traversal_of_a_binary_tree.rs)|
|988|[Smallest String Starting From Leaf](https://leetcode.com/problems/smallest-string-starting-from-leaf)|tree depth-first-search|[solution](rustgym/src/leetcode/_988_smallest_string_starting_from_leaf.rs)|
|98|[Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_98_validate_binary_search_tree.rs)|
|990|[Satisfiability of Equality Equations](https://leetcode.com/problems/satisfiability-of-equality-equations)|union-find graph|[solution](rustgym/src/leetcode/_990_satisfiability_of_equality_equations.rs)|
|991|[Broken Calculator](https://leetcode.com/problems/broken-calculator)|math greedy|[solution](rustgym/src/leetcode/_991_broken_calculator.rs)|
|994|[Rotting Oranges](https://leetcode.com/problems/rotting-oranges)|breadth-first-search|[solution](rustgym/src/leetcode/_994_rotting_oranges.rs)|
|998|[Maximum Binary Tree II](https://leetcode.com/problems/maximum-binary-tree-ii)|tree|[solution](rustgym/src/leetcode/_998_maximum_binary_tree_2.rs)|
</details>
<details><summary>Hard 165/310 53.23%</summary>


|id|310 Hard Questions|Tags|145 Solutions|
|---|---|---|---|
|420|[Strong Password Checker](https://leetcode.com/problems/strong-password-checker)||   |
|440|[K-th Smallest in Lexicographical Order](https://leetcode.com/problems/k-th-smallest-in-lexicographical-order)||   |
|479|[Largest Palindrome Product](https://leetcode.com/problems/largest-palindrome-product)||   |
|798|[Smallest Rotation with Highest Score](https://leetcode.com/problems/smallest-rotation-with-highest-score)||   |
|154|[Find Minimum in Rotated Sorted Array II](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii)|array binary-search|   |
|644|[Maximum Average Subarray II](https://leetcode.com/problems/maximum-average-subarray-ii)|array binary-search|   |
|719|[Find K-th Smallest Pair Distance](https://leetcode.com/problems/find-k-th-smallest-pair-distance)|array binary-search heap|   |
|1157|[Online Majority Element In Subarray](https://leetcode.com/problems/online-majority-element-in-subarray)|array binary-search segment-tree|   |
|689|[Maximum Sum of 3 Non-Overlapping Subarrays](https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays)|array dynamic-programming|   |
|381|[Insert Delete GetRandom O(1) - Duplicates allowed](https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed)|array hash-table design|   |
|85|[Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle)|array hash-table dynamic-programming stack|   |
|782|[Transform to Chessboard](https://leetcode.com/problems/transform-to-chessboard)|array math|   |
|891|[Sum of Subsequence Widths](https://leetcode.com/problems/sum-of-subsequence-widths)|array math|   |
|1330|[Reverse Subarray To Maximize Array Value](https://leetcode.com/problems/reverse-subarray-to-maximize-array-value)|array math|   |
|1499|[Max Value of Equation](https://leetcode.com/problems/max-value-of-equation)|array sliding-window|   |
|84|[Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram)|array stack|   |
|308|[Range Sum Query 2D - Mutable](https://leetcode.com/problems/range-sum-query-2d-mutable)|binary-indexed-tree segment-tree|   |
|793|[Preimage Size of Factorial Zeroes Function](https://leetcode.com/problems/preimage-size-of-factorial-zeroes-function)|binary-search|   |
|1095|[Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array)|binary-search|   |
|1521|[Find a Value of a Mysterious Function Closest to Target](https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target)|binary-search bit-manipulation segment-tree|   |
|327|[Count of Range Sum](https://leetcode.com/problems/count-of-range-sum)|binary-search divide-and-conquer sort binary-indexed-tree segment-tree|   |
|410|[Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum)|binary-search dynamic-programming|   |
|1235|[Maximum Profit in Job Scheduling](https://leetcode.com/problems/maximum-profit-in-job-scheduling)|binary-search dynamic-programming sort|   |
|352|[Data Stream as Disjoint Intervals](https://leetcode.com/problems/data-stream-as-disjoint-intervals)|binary-search ordered-map|   |
|862|[Shortest Subarray with Sum at Least K](https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k)|binary-search queue|   |
|815|[Bus Routes](https://leetcode.com/problems/bus-routes)|breadth-first-search|   |
|1036|[Escape a Large Maze](https://leetcode.com/problems/escape-a-large-maze)|breadth-first-search|   |
|1210|[Minimum Moves to Reach Target with Rotations](https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations)|breadth-first-search|   |
|1263|[Minimum Moves to Move a Box to Their Target Location](https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location)|breadth-first-search|   |
|1293|[Shortest Path in a Grid with Obstacles Elimination](https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination)|breadth-first-search|   |
|1345|[Jump Game IV](https://leetcode.com/problems/jump-game-iv)|breadth-first-search|   |
|1368|[Minimum Cost to Make at Least One Valid Path in a Grid](https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid)|breadth-first-search|   |
|854|[K-Similar Strings](https://leetcode.com/problems/k-similar-strings)|breadth-first-search graph|   |
|913|[Cat and Mouse](https://leetcode.com/problems/cat-and-mouse)|breadth-first-search minimax|   |
|488|[Zuma Game](https://leetcode.com/problems/zuma-game)|depth-first-search|   |
|489|[Robot Room Cleaner](https://leetcode.com/problems/robot-room-cleaner)|depth-first-search|   |
|679|[24 Game](https://leetcode.com/problems/24-game)|depth-first-search|   |
|749|[Contain Virus](https://leetcode.com/problems/contain-virus)|depth-first-search|   |
|1377|[Frog Position After T Seconds](https://leetcode.com/problems/frog-position-after-t-seconds)|depth-first-search|   |
|499|[The Maze III](https://leetcode.com/problems/the-maze-iii)|depth-first-search breadth-first-search|   |
|924|[Minimize Malware Spread](https://leetcode.com/problems/minimize-malware-spread)|depth-first-search union-find|   |
|1489|[Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree](https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree)|depth-first-search union-find|   |
|839|[Similar String Groups](https://leetcode.com/problems/similar-string-groups)|depth-first-search union-find graph|   |
|928|[Minimize Malware Spread II](https://leetcode.com/problems/minimize-malware-spread-ii)|depth-first-search union-find graph|   |
|631|[Design Excel Sum Formula](https://leetcode.com/problems/design-excel-sum-formula)|design|   |
|1172|[Dinner Plate Stacks](https://leetcode.com/problems/dinner-plate-stacks)|design|   |
|1274|[Number of Ships in a Rectangle](https://leetcode.com/problems/number-of-ships-in-a-rectangle)|divide-and-conquer|   |
|903|[Valid Permutations for DI Sequence](https://leetcode.com/problems/valid-permutations-for-di-sequence)|divide-and-conquer dynamic-programming|   |
|514|[Freedom Trail](https://leetcode.com/problems/freedom-trail)|divide-and-conquer dynamic-programming depth-first-search|   |
|218|[The Skyline Problem](https://leetcode.com/problems/the-skyline-problem)|divide-and-conquer heap binary-indexed-tree segment-tree line-sweep|   |
|446|[Arithmetic Slices II - Subsequence](https://leetcode.com/problems/arithmetic-slices-ii-subsequence)|dynamic-programming|   |
|466|[Count The Repetitions](https://leetcode.com/problems/count-the-repetitions)|dynamic-programming|   |
|471|[Encode String with Shortest Length](https://leetcode.com/problems/encode-string-with-shortest-length)|dynamic-programming|   |
|552|[Student Attendance Record II](https://leetcode.com/problems/student-attendance-record-ii)|dynamic-programming|   |
|568|[Maximum Vacation Days](https://leetcode.com/problems/maximum-vacation-days)|dynamic-programming|   |
|600|[Non-negative Integers without Consecutive Ones](https://leetcode.com/problems/non-negative-integers-without-consecutive-ones)|dynamic-programming|   |
|629|[K Inverse Pairs Array](https://leetcode.com/problems/k-inverse-pairs-array)|dynamic-programming|   |
|639|[Decode Ways II](https://leetcode.com/problems/decode-ways-ii)|dynamic-programming|   |
|656|[Coin Path](https://leetcode.com/problems/coin-path)|dynamic-programming|   |
|741|[Cherry Pickup](https://leetcode.com/problems/cherry-pickup)|dynamic-programming|   |
|879|[Profitable Schemes](https://leetcode.com/problems/profitable-schemes)|dynamic-programming|   |
|920|[Number of Music Playlists](https://leetcode.com/problems/number-of-music-playlists)|dynamic-programming|   |
|940|[Distinct Subsequences II](https://leetcode.com/problems/distinct-subsequences-ii)|dynamic-programming|   |
|943|[Find the Shortest Superstring](https://leetcode.com/problems/find-the-shortest-superstring)|dynamic-programming|   |
|956|[Tallest Billboard](https://leetcode.com/problems/tallest-billboard)|dynamic-programming|   |
|1000|[Minimum Cost to Merge Stones](https://leetcode.com/problems/minimum-cost-to-merge-stones)|dynamic-programming|   |
|1187|[Make Array Strictly Increasing](https://leetcode.com/problems/make-array-strictly-increasing)|dynamic-programming|   |
|1246|[Palindrome Removal](https://leetcode.com/problems/palindrome-removal)|dynamic-programming|   |
|1269|[Number of Ways to Stay in the Same Place After Some Steps](https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps)|dynamic-programming|   |
|1301|[Number of Paths with Max Score](https://leetcode.com/problems/number-of-paths-with-max-score)|dynamic-programming|   |
|1349|[Maximum Students Taking Exam](https://leetcode.com/problems/maximum-students-taking-exam)|dynamic-programming|   |
|1388|[Pizza With 3n Slices](https://leetcode.com/problems/pizza-with-3n-slices)|dynamic-programming|   |
|1397|[Find All Good Strings](https://leetcode.com/problems/find-all-good-strings)|dynamic-programming|   |
|1416|[Restore The Array](https://leetcode.com/problems/restore-the-array)|dynamic-programming|   |
|1425|[Constrained Subsequence Sum](https://leetcode.com/problems/constrained-subsequence-sum)|dynamic-programming|   |
|1458|[Max Dot Product of Two Subsequences](https://leetcode.com/problems/max-dot-product-of-two-subsequences)|dynamic-programming|   |
|1473|[Paint House III](https://leetcode.com/problems/paint-house-iii)|dynamic-programming|   |
|1483|[Kth Ancestor of a Tree Node](https://leetcode.com/problems/kth-ancestor-of-a-tree-node)|dynamic-programming|   |
|691|[Stickers to Spell Word](https://leetcode.com/problems/stickers-to-spell-word)|dynamic-programming backtracking|   |
|1373|[Maximum Sum BST in Binary Tree](https://leetcode.com/problems/maximum-sum-bst-in-binary-tree)|dynamic-programming binary-search-tree|   |
|1125|[Smallest Sufficient Team](https://leetcode.com/problems/smallest-sufficient-team)|dynamic-programming bit-manipulation|   |
|1434|[Number of Ways to Wear Different Hats to Each Other](https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other)|dynamic-programming bit-manipulation|   |
|546|[Remove Boxes](https://leetcode.com/problems/remove-boxes)|dynamic-programming depth-first-search|   |
|664|[Strange Printer](https://leetcode.com/problems/strange-printer)|dynamic-programming depth-first-search|   |
|1326|[Minimum Number of Taps to Open to Water a Garden](https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden)|dynamic-programming greedy|   |
|727|[Minimum Window Subsequence](https://leetcode.com/problems/minimum-window-subsequence)|dynamic-programming sliding-window|   |
|975|[Odd Even Jump](https://leetcode.com/problems/odd-even-jump)|dynamic-programming stack ordered-map|   |
|968|[Binary Tree Cameras](https://leetcode.com/problems/binary-tree-cameras)|dynamic-programming tree depth-first-search|   |
|587|[Erect the Fence](https://leetcode.com/problems/erect-the-fence)|geometry|   |
|1453|[Maximum Number of Darts Inside of a Circular Dartboard](https://leetcode.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard)|geometry|   |
|1515|[Best Position for a Service Centre](https://leetcode.com/problems/best-position-for-a-service-centre)|geometry|   |
|1153|[String Transforms Into Another String](https://leetcode.com/problems/string-transforms-into-another-string)|graph|   |
|1494|[Parallel Courses II](https://leetcode.com/problems/parallel-courses-ii)|graph|   |
|757|[Set Intersection Size At Least Two](https://leetcode.com/problems/set-intersection-size-at-least-two)|greedy|   |
|1354|[Construct Target Array With Multiple Sums](https://leetcode.com/problems/construct-target-array-with-multiple-sums)|greedy|   |
|1505|[Minimum Possible Integer After at Most K Adjacent Swaps On Digits](https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits)|greedy|   |
|995|[Minimum Number of K Consecutive Bit Flips](https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips)|greedy sliding-window|   |
|1383|[Maximum Performance of a Team](https://leetcode.com/problems/maximum-performance-of-a-team)|greedy sort|   |
|1001|[Grid Illumination](https://leetcode.com/problems/grid-illumination)|hash-table|   |
|1224|[Maximum Equal Frequency](https://leetcode.com/problems/maximum-equal-frequency)|hash-table|   |
|1044|[Longest Duplicate Substring](https://leetcode.com/problems/longest-duplicate-substring)|hash-table binary-search|   |
|710|[Random Pick with Blacklist](https://leetcode.com/problems/random-pick-with-blacklist)|hash-table binary-search sort random|   |
|1178|[Number of Valid Words for Each Puzzle](https://leetcode.com/problems/number-of-valid-words-for-each-puzzle)|hash-table bit-manipulation|   |
|711|[Number of Distinct Islands II](https://leetcode.com/problems/number-of-distinct-islands-ii)|hash-table depth-first-search|   |
|770|[Basic Calculator IV](https://leetcode.com/problems/basic-calculator-iv)|hash-table string stack|   |
|992|[Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers)|hash-table two-pointers sliding-window|   |
|857|[Minimum Cost to Hire K Workers](https://leetcode.com/problems/minimum-cost-to-hire-k-workers)|heap|   |
|882|[Reachable Nodes In Subdivided Graph](https://leetcode.com/problems/reachable-nodes-in-subdivided-graph)|heap|   |
|407|[Trapping Rain Water II](https://leetcode.com/problems/trapping-rain-water-ii)|heap breadth-first-search|   |
|864|[Shortest Path to Get All Keys](https://leetcode.com/problems/shortest-path-to-get-all-keys)|heap breadth-first-search|   |
|502|[IPO](https://leetcode.com/problems/ipo)|heap greedy|   |
|759|[Employee Free Time](https://leetcode.com/problems/employee-free-time)|heap greedy|   |
|391|[Perfect Rectangle](https://leetcode.com/problems/perfect-rectangle)|line-sweep|   |
|335|[Self Crossing](https://leetcode.com/problems/self-crossing)|math|   |
|458|[Poor Pigs](https://leetcode.com/problems/poor-pigs)|math|   |
|780|[Reaching Points](https://leetcode.com/problems/reaching-points)|math|   |
|805|[Split Array With Same Average](https://leetcode.com/problems/split-array-with-same-average)|math|   |
|810|[Chalkboard XOR Game](https://leetcode.com/problems/chalkboard-xor-game)|math|   |
|906|[Super Palindromes](https://leetcode.com/problems/super-palindromes)|math|   |
|972|[Equal Rational Numbers](https://leetcode.com/problems/equal-rational-numbers)|math|   |
|1088|[Confusing Number II](https://leetcode.com/problems/confusing-number-ii)|math backtracking|   |
|1307|[Verbal Arithmetic Puzzle](https://leetcode.com/problems/verbal-arithmetic-puzzle)|math backtracking|   |
|1467|[Probability of a Two Boxes Having The Same Number of Distinct Balls](https://leetcode.com/problems/probability-of-a-two-boxes-having-the-same-number-of-distinct-balls)|math backtracking|   |
|996|[Number of Squareful Arrays](https://leetcode.com/problems/number-of-squareful-arrays)|math backtracking graph|   |
|483|[Smallest Good Base](https://leetcode.com/problems/smallest-good-base)|math binary-search|   |
|878|[Nth Magical Number](https://leetcode.com/problems/nth-magical-number)|math binary-search|   |
|887|[Super Egg Drop](https://leetcode.com/problems/super-egg-drop)|math binary-search dynamic-programming|   |
|927|[Three Equal Parts](https://leetcode.com/problems/three-equal-parts)|math binary-search greedy|   |
|753|[Cracking the Safe](https://leetcode.com/problems/cracking-the-safe)|math depth-first-search|   |
|517|[Super Washing Machines](https://leetcode.com/problems/super-washing-machines)|math dynamic-programming|   |
|902|[Numbers At Most N Given Digit Set](https://leetcode.com/problems/numbers-at-most-n-given-digit-set)|math dynamic-programming|   |
|964|[Least Operators to Express Number](https://leetcode.com/problems/least-operators-to-express-number)|math dynamic-programming|   |
|1012|[Numbers With Repeated Digits](https://leetcode.com/problems/numbers-with-repeated-digits)|math dynamic-programming|   |
|1067|[Digit Count in Range](https://leetcode.com/problems/digit-count-in-range)|math dynamic-programming|   |
|1199|[Minimum Time to Build Blocks](https://leetcode.com/problems/minimum-time-to-build-blocks)|math dynamic-programming|   |
|1363|[Largest Multiple of Three](https://leetcode.com/problems/largest-multiple-of-three)|math dynamic-programming|   |
|952|[Largest Component Size by Common Factor](https://leetcode.com/problems/largest-component-size-by-common-factor)|math union-find|   |
|843|[Guess the Word](https://leetcode.com/problems/guess-the-word)|minimax|   |
|683|[K Empty Slots](https://leetcode.com/problems/k-empty-slots)|ordered-map|   |
|850|[Rectangle Area II](https://leetcode.com/problems/rectangle-area-ii)|segment-tree line-sweep|   |
|699|[Falling Squares](https://leetcode.com/problems/falling-squares)|segment-tree ordered-map|   |
|715|[Range Module](https://leetcode.com/problems/range-module)|segment-tree ordered-map|   |
|480|[Sliding Window Median](https://leetcode.com/problems/sliding-window-median)|sliding-window|   |
|158|[Read N Characters Given Read4 II - Call multiple times](https://leetcode.com/problems/read-n-characters-given-read4-ii-call-multiple-times)|string|   |
|564|[Find the Closest Palindrome](https://leetcode.com/problems/find-the-closest-palindrome)|string|   |
|736|[Parse Lisp Expression](https://leetcode.com/problems/parse-lisp-expression)|string|   |
|1392|[Longest Happy Prefix](https://leetcode.com/problems/longest-happy-prefix)|string|   |
|1542|[Find Longest Awesome Substring](https://leetcode.com/problems/find-longest-awesome-substring)|string bit-manipulation|   |
|730|[Count Different Palindromic Subsequences](https://leetcode.com/problems/count-different-palindromic-subsequences)|string dynamic-programming|   |
|1449|[Form Largest Integer With Digits That Add up to Target](https://leetcode.com/problems/form-largest-integer-with-digits-that-add-up-to-target)|string dynamic-programming|   |
|1531|[String Compression II](https://leetcode.com/problems/string-compression-ii)|string dynamic-programming|   |
|936|[Stamping The Sequence](https://leetcode.com/problems/stamping-the-sequence)|string greedy|   |
|1316|[Distinct Echo Substrings](https://leetcode.com/problems/distinct-echo-substrings)|string rolling-hash|   |
|591|[Tag Validator](https://leetcode.com/problems/tag-validator)|string stack|   |
|772|[Basic Calculator III](https://leetcode.com/problems/basic-calculator-iii)|string stack|   |
|1163|[Last Substring in Lexicographical Order](https://leetcode.com/problems/last-substring-in-lexicographical-order)|string suffix-array|   |
|428|[Serialize and Deserialize N-ary Tree](https://leetcode.com/problems/serialize-and-deserialize-n-ary-tree)|tree|   |
|431|[Encode N-ary Tree to Binary Tree](https://leetcode.com/problems/encode-n-ary-tree-to-binary-tree)|tree|   |
|1516|[Move Sub-Tree of N-Ary Tree](https://leetcode.com/problems/move-sub-tree-of-n-ary-tree)|tree|   |
|834|[Sum of Distances in Tree](https://leetcode.com/problems/sum-of-distances-in-tree)|tree depth-first-search|   |
|685|[Redundant Connection II](https://leetcode.com/problems/redundant-connection-ii)|tree depth-first-search union-find graph|   |
|297|[Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree)|tree design|   |
|745|[Prefix and Suffix Search](https://leetcode.com/problems/prefix-and-suffix-search)|trie|   |
|828|[Count Unique Characters of All Substrings of a Given String](https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string)|two-pointers|   |
|803|[Bricks Falling When Hit](https://leetcode.com/problems/bricks-falling-when-hit)|union-find|   |
|1028|[Recover a Tree From Preorder Traversal](https://leetcode.com/problems/recover-a-tree-from-preorder-traversal)|tree depth-first-search|[solution](rustgym/src/leetcode/_1028_recover_a_tree_from_preorder_traversal.rs)|
|1032|[Stream of Characters](https://leetcode.com/problems/stream-of-characters)|trie|[solution](rustgym/src/leetcode/_1032_stream_of_characters.rs)|
|1063|[Number of Valid Subarrays](https://leetcode.com/problems/number-of-valid-subarrays)|stack|[solution](rustgym/src/leetcode/_1063_number_of_valid_subarrays.rs)|
|1074|[Number of Submatrices That Sum to Target](https://leetcode.com/problems/number-of-submatrices-that-sum-to-target)|array dynamic-programming sliding-window|[solution](rustgym/src/leetcode/_1074_number_of_submatrices_that_sum_to_target.rs)|
|1092|[Shortest Common Supersequence ](https://leetcode.com/problems/shortest-common-supersequence)|dynamic-programming|[solution](rustgym/src/leetcode/_1092_shortest_common_supersequence.rs)|
|1096|[Brace Expansion II](https://leetcode.com/problems/brace-expansion-ii)|string|[solution](rustgym/src/leetcode/_1096_brace_expansion_2.rs)|
|10|[Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching)|string dynamic-programming backtracking|[solution](rustgym/src/leetcode/_10_regular_expression_matching.rs)|
|1106|[Parsing A Boolean Expression](https://leetcode.com/problems/parsing-a-boolean-expression)|string|[solution](rustgym/src/leetcode/_1106_parsing_a_boolean_expression.rs)|
|1121|[Divide Array Into Increasing Sequences](https://leetcode.com/problems/divide-array-into-increasing-sequences)|math|[solution](rustgym/src/leetcode/_1121_divide_array_into_increasing_sequences.rs)|
|1136|[Parallel Courses](https://leetcode.com/problems/parallel-courses)|dynamic-programming depth-first-search graph|[solution](rustgym/src/leetcode/_1136_parallel_courses.rs)|
|1147|[Longest Chunked Palindrome Decomposition](https://leetcode.com/problems/longest-chunked-palindrome-decomposition)|dynamic-programming rolling-hash|[solution](rustgym/src/leetcode/_1147_longest_chunked_palindrome_decomposition.rs)|
|115|[Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences)|string dynamic-programming|[solution](rustgym/src/leetcode/_115_distinct_subsequences.rs)|
|1168|[Optimize Water Distribution in a Village](https://leetcode.com/problems/optimize-water-distribution-in-a-village)|union-find graph|[solution](rustgym/src/leetcode/_1168_optimize_water_distribution_in_a_village.rs)|
|1183|[Maximum Number of Ones](https://leetcode.com/problems/maximum-number-of-ones)|math sort|[solution](rustgym/src/leetcode/_1183_maximum_number_of_ones.rs)|
|1192|[Critical Connections in a Network](https://leetcode.com/problems/critical-connections-in-a-network)|depth-first-search|[solution](rustgym/src/leetcode/_1192_critical_connections_in_a_network.rs)|
|1203|[Sort Items by Groups Respecting Dependencies](https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies)|depth-first-search graph topological-sort|[solution](rustgym/src/leetcode/_1203_sort_items_by_groups_respecting_dependencies.rs)|
|1206|[Design Skiplist](https://leetcode.com/problems/design-skiplist)|design|[solution](rustgym/src/leetcode/_1206_design_skiplist.rs)|
|1216|[Valid Palindrome III](https://leetcode.com/problems/valid-palindrome-iii)|string dynamic-programming|[solution](rustgym/src/leetcode/_1216_valid_palindrome_3.rs)|
|1220|[Count Vowels Permutation](https://leetcode.com/problems/count-vowels-permutation)|dynamic-programming|[solution](rustgym/src/leetcode/_1220_count_vowels_permutation.rs)|
|1231|[Divide Chocolate](https://leetcode.com/problems/divide-chocolate)|binary-search greedy|[solution](rustgym/src/leetcode/_1231_divide_chocolate.rs)|
|123|[Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii)|array dynamic-programming|[solution](rustgym/src/leetcode/_123_best_time_to_buy_and_sell_stock_3.rs)|
|1240|[Tiling a Rectangle with the Fewest Squares](https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares)|dynamic-programming backtracking|[solution](rustgym/src/leetcode/_1240_tiling_a_rectangle_with_the_fewest_squares.rs)|
|124|[Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum)|tree depth-first-search|[solution](rustgym/src/leetcode/_124_binary_tree_maximum_path_sum.rs)|
|1250|[Check If It Is a Good Array](https://leetcode.com/problems/check-if-it-is-a-good-array)|math|[solution](rustgym/src/leetcode/_1250_check_if_it_is_a_good_array.rs)|
|1255|[Maximum Score Words Formed by Letters](https://leetcode.com/problems/maximum-score-words-formed-by-letters)|bit-manipulation|[solution](rustgym/src/leetcode/_1255_maximum_score_words_formed_by_letters.rs)|
|1259|[Handshakes That Don't Cross](https://leetcode.com/problems/handshakes-that-dont-cross)|math dynamic-programming|[solution](rustgym/src/leetcode/_1259_handshakes_that_don_t_cross.rs)|
|126|[Word Ladder II](https://leetcode.com/problems/word-ladder-ii)|array string backtracking breadth-first-search|[solution](rustgym/src/leetcode/_126_word_ladder_2.rs)|
|1278|[Palindrome Partitioning III](https://leetcode.com/problems/palindrome-partitioning-iii)|dynamic-programming|[solution](rustgym/src/leetcode/_1278_palindrome_partitioning_3.rs)|
|1284|[Minimum Number of Flips to Convert Binary Matrix to Zero Matrix](https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix)|breadth-first-search|[solution](rustgym/src/leetcode/_1284_minimum_number_of_flips_to_convert_binary_matrix_to_zero_matrix.rs)|
|1289|[Minimum Falling Path Sum II](https://leetcode.com/problems/minimum-falling-path-sum-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_1289_minimum_falling_path_sum_2.rs)|
|128|[Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence)|array union-find|[solution](rustgym/src/leetcode/_128_longest_consecutive_sequence.rs)|
|1298|[Maximum Candies You Can Get from Boxes](https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes)|breadth-first-search|[solution](rustgym/src/leetcode/_1298_maximum_candies_you_can_get_from_boxes.rs)|
|1312|[Minimum Insertion Steps to Make a String Palindrome](https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome)|dynamic-programming|[solution](rustgym/src/leetcode/_1312_minimum_insertion_steps_to_make_a_string_palindrome.rs)|
|1320|[Minimum Distance to Type a Word Using Two Fingers](https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers)|dynamic-programming|[solution](rustgym/src/leetcode/_1320_minimum_distance_to_type_a_word_using_two_fingers.rs)|
|132|[Palindrome Partitioning II](https://leetcode.com/problems/palindrome-partitioning-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_132_palindrome_partitioning_2.rs)|
|1335|[Minimum Difficulty of a Job Schedule](https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule)|dynamic-programming|[solution](rustgym/src/leetcode/_1335_minimum_difficulty_of_a_job_schedule.rs)|
|1340|[Jump Game V](https://leetcode.com/problems/jump-game-v)|dynamic-programming|[solution](rustgym/src/leetcode/_1340_jump_game_5.rs)|
|1359|[Count All Valid Pickup and Delivery Options](https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options)|math dynamic-programming|[solution](rustgym/src/leetcode/_1359_count_all_valid_pickup_and_delivery_options.rs)|
|135|[Candy](https://leetcode.com/problems/candy)|greedy|[solution](rustgym/src/leetcode/_135_candy.rs)|
|1402|[Reducing Dishes](https://leetcode.com/problems/reducing-dishes)|dynamic-programming|[solution](rustgym/src/leetcode/_1402_reducing_dishes.rs)|
|1406|[Stone Game III](https://leetcode.com/problems/stone-game-iii)|dynamic-programming|[solution](rustgym/src/leetcode/_1406_stone_game_3.rs)|
|140|[Word Break II](https://leetcode.com/problems/word-break-ii)|dynamic-programming backtracking|[solution](rustgym/src/leetcode/_140_word_break_2.rs)|
|1411|[Number of Ways to Paint N × 3 Grid](https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid)|dynamic-programming|[solution](rustgym/src/leetcode/_1411_number_of_ways_to_paint_n3_grid.rs)|
|1420|[Build Array Where You Can Find The Maximum Exactly K Comparisons](https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons)|dynamic-programming|[solution](rustgym/src/leetcode/_1420_build_array_where_you_can_find_the_maximum_exactly_k_comparisons.rs)|
|1439|[Find the Kth Smallest Sum of a Matrix With Sorted Rows](https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows)|heap|[solution](rustgym/src/leetcode/_1439_find_kth_smallest_sum_of_a_matrix_with_sorted_rows.rs)|
|1444|[Number of Ways of Cutting a Pizza](https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza)|dynamic-programming|[solution](rustgym/src/leetcode/_1444_number_of_ways_of_cutting_a_pizza.rs)|
|145|[Binary Tree Postorder Traversal](https://leetcode.com/problems/binary-tree-postorder-traversal)|stack tree|[solution](rustgym/src/leetcode/_145_binary_tree_postorder_traversal.rs)|
|1463|[Cherry Pickup II](https://leetcode.com/problems/cherry-pickup-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_1463_cherry_pickup_2.rs)|
|1478|[Allocate Mailboxes](https://leetcode.com/problems/allocate-mailboxes)|math dynamic-programming|[solution](rustgym/src/leetcode/_1478_allocate_mailboxes.rs)|
|149|[Max Points on a Line](https://leetcode.com/problems/max-points-on-a-line)|hash-table math|[solution](rustgym/src/leetcode/_149_max_points_on_a_line.rs)|
|1510|[Stone Game IV](https://leetcode.com/problems/stone-game-iv)|dynamic-programming|[solution](rustgym/src/leetcode/_1510_stone_game_4.rs)|
|1520|[Maximum Number of Non-Overlapping Substrings](https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings)|greedy|[solution](rustgym/src/leetcode/_1520_maximum_number_of_non_overlapping_substrings.rs)|
|1526|[Minimum Number of Increments on Subarrays to Form a Target Array](https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array)|segment-tree|[solution](rustgym/src/leetcode/_1526_minimum_number_of_increments_on_subarrays_to_form_a_target_array.rs)|
|1537|[Get the Maximum Score](https://leetcode.com/problems/get-the-maximum-score)|dynamic-programming|[solution](rustgym/src/leetcode/_1537_get_the_maximum_score.rs)|
|1547|[Minimum Cost to Cut a Stick](https://leetcode.com/problems/minimum-cost-to-cut-a-stick)|dynamic-programming|[solution](rustgym/src/leetcode/_1547_minimum_cost_to_cut_a_stick.rs)|
|1548|[The Most Similar Path in a Graph](https://leetcode.com/problems/the-most-similar-path-in-a-graph)|dynamic-programming graph|[solution](rustgym/src/leetcode/_1548_the_most_similar_path_in_a_graph.rs)|
|1553|[Minimum Number of Days to Eat N Oranges](https://leetcode.com/problems/minimum-number-of-days-to-eat-n-oranges)|dynamic-programming|[solution](rustgym/src/leetcode/_1553_minimum_number_of_days_to_eat_n_oranges.rs)|
|164|[Maximum Gap](https://leetcode.com/problems/maximum-gap)|sort|[solution](rustgym/src/leetcode/_164_maximum_gap.rs)|
|174|[Dungeon Game](https://leetcode.com/problems/dungeon-game)|binary-search dynamic-programming|[solution](rustgym/src/leetcode/_174_dungeon_game.rs)|
|188|[Best Time to Buy and Sell Stock IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv)|dynamic-programming|[solution](rustgym/src/leetcode/_188_best_time_to_buy_and_sell_stock_4.rs)|
|212|[Word Search II](https://leetcode.com/problems/word-search-ii)|backtracking trie|[solution](rustgym/src/leetcode/_212_word_search_2.rs)|
|214|[Shortest Palindrome](https://leetcode.com/problems/shortest-palindrome)|string|[solution](rustgym/src/leetcode/_214_shortest_palindrome.rs)|
|224|[Basic Calculator](https://leetcode.com/problems/basic-calculator)|math stack|[solution](rustgym/src/leetcode/_224_basic_calculator.rs)|
|233|[Number of Digit One](https://leetcode.com/problems/number-of-digit-one)|math|[solution](rustgym/src/leetcode/_233_number_of_digit_one.rs)|
|239|[Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum)|heap sliding-window|[solution](rustgym/src/leetcode/_239_sliding_window_maximum.rs)|
|23|[Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists)|linked-list divide-and-conquer heap|[solution](rustgym/src/leetcode/_23_merge_k_sorted_lists.rs)|
|248|[Strobogrammatic Number III](https://leetcode.com/problems/strobogrammatic-number-iii)|math recursion|[solution](rustgym/src/leetcode/_248_strobogrammatic_number_3.rs)|
|25|[Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group)|linked-list|[solution](rustgym/src/leetcode/_25_reverse_nodes_in_k_group.rs)|
|265|[Paint House II](https://leetcode.com/problems/paint-house-ii)|dynamic-programming|[solution](rustgym/src/leetcode/_265_paint_house_2.rs)|
|269|[Alien Dictionary](https://leetcode.com/problems/alien-dictionary)|graph topological-sort|[solution](rustgym/src/leetcode/_269_alien_dictionary.rs)|
|272|[Closest Binary Search Tree Value II](https://leetcode.com/problems/closest-binary-search-tree-value-ii)|stack tree|[solution](rustgym/src/leetcode/_272_closest_binary_search_tree_value_2.rs)|
|273|[Integer to English Words](https://leetcode.com/problems/integer-to-english-words)|math string|[solution](rustgym/src/leetcode/_273_integer_to_english_words.rs)|
|282|[Expression Add Operators](https://leetcode.com/problems/expression-add-operators)|divide-and-conquer|[solution](rustgym/src/leetcode/_282_expression_add_operators.rs)|
|291|[Word Pattern II](https://leetcode.com/problems/word-pattern-ii)|backtracking|[solution](rustgym/src/leetcode/_291_word_pattern_2.rs)|
|295|[Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream)|heap design|[solution](rustgym/src/leetcode/_295_find_median_from_data_stream.rs)|
|296|[Best Meeting Point](https://leetcode.com/problems/best-meeting-point)|math sort|[solution](rustgym/src/leetcode/_296_best_meeting_point.rs)|
|301|[Remove Invalid Parentheses](https://leetcode.com/problems/remove-invalid-parentheses)|depth-first-search breadth-first-search|[solution](rustgym/src/leetcode/_301_remove_invalid_parentheses.rs)|
|302|[Smallest Rectangle Enclosing Black Pixels](https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels)|binary-search|[solution](rustgym/src/leetcode/_302_smallest_rectangle_enclosing_black_pixels.rs)|
|305|[Number of Islands II](https://leetcode.com/problems/number-of-islands-ii)|union-find|[solution](rustgym/src/leetcode/_305_number_of_islands_2.rs)|
|30|[Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words)|hash-table two-pointers string|[solution](rustgym/src/leetcode/_30_substring_with_concatenation_of_all_words.rs)|
|312|[Burst Balloons](https://leetcode.com/problems/burst-balloons)|divide-and-conquer dynamic-programming|[solution](rustgym/src/leetcode/_312_burst_balloons.rs)|
|315|[Count of Smaller Numbers After Self](https://leetcode.com/problems/count-of-smaller-numbers-after-self)|binary-search divide-and-conquer sort binary-indexed-tree segment-tree|[solution](rustgym/src/leetcode/_315_count_of_smaller_numbers_after_self.rs)|
|316|[Remove Duplicate Letters](https://leetcode.com/problems/remove-duplicate-letters)|stack greedy|[solution](rustgym/src/leetcode/_316_remove_duplicate_letters.rs)|
|317|[Shortest Distance from All Buildings](https://leetcode.com/problems/shortest-distance-from-all-buildings)|breadth-first-search|[solution](rustgym/src/leetcode/_317_shortest_distance_from_all_buildings.rs)|
|321|[Create Maximum Number](https://leetcode.com/problems/create-maximum-number)|dynamic-programming greedy|[solution](rustgym/src/leetcode/_321_create_maximum_number.rs)|
|329|[Longest Increasing Path in a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix)|depth-first-search topological-sort memoization|[solution](rustgym/src/leetcode/_329_longest_increasing_path_in_a_matrix.rs)|
|32|[Longest Valid Parentheses](https://leetcode.com/problems/longest-valid-parentheses)|string dynamic-programming|[solution](rustgym/src/leetcode/_32_longest_valid_parentheses.rs)|
|330|[Patching Array](https://leetcode.com/problems/patching-array)|greedy|[solution](rustgym/src/leetcode/_330_patching_array.rs)|
|336|[Palindrome Pairs](https://leetcode.com/problems/palindrome-pairs)|hash-table string trie|[solution](rustgym/src/leetcode/_336_palindrome_pairs.rs)|
|340|[Longest Substring with At Most K Distinct Characters](https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters)|hash-table string sliding-window|[solution](rustgym/src/leetcode/_340_longest_substring_with_at_most_k_distinct_characters.rs)|
|354|[Russian Doll Envelopes](https://leetcode.com/problems/russian-doll-envelopes)|binary-search dynamic-programming|[solution](rustgym/src/leetcode/_354_russian_doll_envelopes.rs)|
|358|[Rearrange String k Distance Apart](https://leetcode.com/problems/rearrange-string-k-distance-apart)|hash-table heap greedy|[solution](rustgym/src/leetcode/_358_rearrange_string_k_distance_apart.rs)|
|363|[Max Sum of Rectangle No Larger Than K](https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k)|binary-search dynamic-programming queue|[solution](rustgym/src/leetcode/_363_max_sum_of_rectangle_no_larger_than_k.rs)|
|37|[Sudoku Solver](https://leetcode.com/problems/sudoku-solver)|hash-table backtracking|[solution](rustgym/src/leetcode/_37_sudoku_solver.rs)|
|403|[Frog Jump](https://leetcode.com/problems/frog-jump)|dynamic-programming|[solution](rustgym/src/leetcode/_403_frog_jump.rs)|
|411|[Minimum Unique Word Abbreviation](https://leetcode.com/problems/minimum-unique-word-abbreviation)|backtracking bit-manipulation|[solution](rustgym/src/leetcode/_411_minimum_unique_word_abbreviation.rs)|
|41|[First Missing Positive](https://leetcode.com/problems/first-missing-positive)|array|[solution](rustgym/src/leetcode/_41_first_missing_positive.rs)|
|425|[Word Squares](https://leetcode.com/problems/word-squares)|backtracking trie|[solution](rustgym/src/leetcode/_425_word_squares.rs)|
|42|[Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water)|array two-pointers stack|[solution](rustgym/src/leetcode/_42_trapping_rain_water.rs)|
|432|[All O`one Data Structure](https://leetcode.com/problems/all-oone-data-structure)|design|[solution](rustgym/src/leetcode/_432_all_o_one_data_structure.rs)|
|44|[Wildcard Matching](https://leetcode.com/problems/wildcard-matching)|string dynamic-programming backtracking greedy|[solution](rustgym/src/leetcode/_44_wildcard_matching.rs)|
|45|[Jump Game II](https://leetcode.com/problems/jump-game-ii)|array greedy|[solution](rustgym/src/leetcode/_45_jump_game_2.rs)|
|460|[LFU Cache](https://leetcode.com/problems/lfu-cache)|design|[solution](rustgym/src/leetcode/_460_lfu_cache.rs)|
|465|[Optimal Account Balancing](https://leetcode.com/problems/optimal-account-balancing)||[solution](rustgym/src/leetcode/_465_optimal_account_balancing.rs)|
|472|[Concatenated Words](https://leetcode.com/problems/concatenated-words)|dynamic-programming depth-first-search trie|[solution](rustgym/src/leetcode/_472_concatenated_words.rs)|
|493|[Reverse Pairs](https://leetcode.com/problems/reverse-pairs)|binary-search divide-and-conquer sort binary-indexed-tree segment-tree|[solution](rustgym/src/leetcode/_493_reverse_pairs.rs)|
|4|[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays)|array binary-search divide-and-conquer|[solution](rustgym/src/leetcode/_4_median_of_two_sorted_arrays.rs)|
|51|[N-Queens](https://leetcode.com/problems/n-queens)|backtracking|[solution](rustgym/src/leetcode/_51_n_queens.rs)|
|527|[Word Abbreviation](https://leetcode.com/problems/word-abbreviation)|string sort|[solution](rustgym/src/leetcode/_527_word_abbreviation.rs)|
|52|[N-Queens II](https://leetcode.com/problems/n-queens-ii)|backtracking|[solution](rustgym/src/leetcode/_52_n_queens_2.rs)|
|57|[Insert Interval](https://leetcode.com/problems/insert-interval)|array sort|[solution](rustgym/src/leetcode/_57_insert_interval.rs)|
|588|[Design In-Memory File System](https://leetcode.com/problems/design-in-memory-file-system)|design|[solution](rustgym/src/leetcode/_588_design_in_memory_file_system.rs)|
|60|[Permutation Sequence](https://leetcode.com/problems/permutation-sequence)|math backtracking|[solution](rustgym/src/leetcode/_60_permutation_sequence.rs)|
|630|[Course Schedule III](https://leetcode.com/problems/course-schedule-iii)|greedy|[solution](rustgym/src/leetcode/_630_course_schedule_3.rs)|
|632|[Smallest Range Covering Elements from K Lists](https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists)|hash-table two-pointers string|[solution](rustgym/src/leetcode/_632_smallest_range_covering_elements_from_k_lists.rs)|
|642|[Design Search Autocomplete System](https://leetcode.com/problems/design-search-autocomplete-system)|design trie|[solution](rustgym/src/leetcode/_642_design_search_autocomplete_system.rs)|
|65|[Valid Number](https://leetcode.com/problems/valid-number)|math string|[solution](rustgym/src/leetcode/_65_valid_number.rs)|
|660|[Remove 9](https://leetcode.com/problems/remove-9)|math|[solution](rustgym/src/leetcode/_660_remove_9.rs)|
|668|[Kth Smallest Number in Multiplication Table](https://leetcode.com/problems/kth-smallest-number-in-multiplication-table)|binary-search|[solution](rustgym/src/leetcode/_668_kth_smallest_number_in_multiplication.rs)|
|675|[Cut Off Trees for Golf Event](https://leetcode.com/problems/cut-off-trees-for-golf-event)|breadth-first-search|[solution](rustgym/src/leetcode/_675_cut_off_trees_for_golf_event.rs)|
|68|[Text Justification](https://leetcode.com/problems/text-justification)|string|[solution](rustgym/src/leetcode/_68_text_justification.rs)|
|726|[Number of Atoms](https://leetcode.com/problems/number-of-atoms)|hash-table stack recursion|[solution](rustgym/src/leetcode/_726_number_of_atoms.rs)|
|72|[Edit Distance](https://leetcode.com/problems/edit-distance)|string dynamic-programming|[solution](rustgym/src/leetcode/_72_edit_distance.rs)|
|732|[My Calendar III](https://leetcode.com/problems/my-calendar-iii)|segment-tree ordered-map|[solution](rustgym/src/leetcode/_732_my_calendar_3.rs)|
|761|[Special Binary String](https://leetcode.com/problems/special-binary-string)|string recursion|[solution](rustgym/src/leetcode/_761_special_binary_string.rs)|
|765|[Couples Holding Hands](https://leetcode.com/problems/couples-holding-hands)|greedy union-find graph|[solution](rustgym/src/leetcode/_765_couples_holding_hands.rs)|
|768|[Max Chunks To Make Sorted II](https://leetcode.com/problems/max-chunks-to-make-sorted-ii)|array|[solution](rustgym/src/leetcode/_768_max_chunks_to_make_sorted_2.rs)|
|76|[Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring)|hash-table two-pointers string sliding-window|[solution](rustgym/src/leetcode/_76_minimum_window_substring.rs)|
|773|[Sliding Puzzle](https://leetcode.com/problems/sliding-puzzle)|breadth-first-search|[solution](rustgym/src/leetcode/_773_sliding_puzzle.rs)|
|774|[Minimize Max Distance to Gas Station](https://leetcode.com/problems/minimize-max-distance-to-gas-station)|binary-search|[solution](rustgym/src/leetcode/_774_minimize_max_distance_to_gas_station.rs)|
|778|[Swim in Rising Water](https://leetcode.com/problems/swim-in-rising-water)|binary-search heap depth-first-search union-find|[solution](rustgym/src/leetcode/_778_swim_in_rising_water.rs)|
|786|[K-th Smallest Prime Fraction](https://leetcode.com/problems/k-th-smallest-prime-fraction)|binary-search heap|[solution](rustgym/src/leetcode/_786_kth_smallest_prime_faction.rs)|
|818|[Race Car](https://leetcode.com/problems/race-car)|dynamic-programming heap|[solution](rustgym/src/leetcode/_818_race_car.rs)|
|827|[Making A Large Island](https://leetcode.com/problems/making-a-large-island)|depth-first-search|[solution](rustgym/src/leetcode/_827_making_a_large_island.rs)|
|829|[Consecutive Numbers Sum](https://leetcode.com/problems/consecutive-numbers-sum)|math|[solution](rustgym/src/leetcode/_829_consecutive_numbers_sum.rs)|
|847|[Shortest Path Visiting All Nodes](https://leetcode.com/problems/shortest-path-visiting-all-nodes)|dynamic-programming breadth-first-search|[solution](rustgym/src/leetcode/_847_shortest_path_visiting_all_nodes.rs)|
|871|[Minimum Number of Refueling Stops](https://leetcode.com/problems/minimum-number-of-refueling-stops)|dynamic-programming heap|[solution](rustgym/src/leetcode/_871_minimum_number_of_refueling_stops.rs)|
|87|[Scramble String](https://leetcode.com/problems/scramble-string)|string dynamic-programming|[solution](rustgym/src/leetcode/_87_scramble_string.rs)|
|895|[Maximum Frequency Stack](https://leetcode.com/problems/maximum-frequency-stack)|hash-table stack|[solution](rustgym/src/leetcode/_895_maximum_frequency_stack.rs)|
|899|[Orderly Queue](https://leetcode.com/problems/orderly-queue)|math string|[solution](rustgym/src/leetcode/_899_orderly_queue.rs)|
|960|[Delete Columns to Make Sorted III](https://leetcode.com/problems/delete-columns-to-make-sorted-iii)|dynamic-programming|[solution](rustgym/src/leetcode/_960_delete_columns_to_make_sorted_3.rs)|
|97|[Interleaving String](https://leetcode.com/problems/interleaving-string)|string dynamic-programming|[solution](rustgym/src/leetcode/_97_interleaving_string.rs)|
|980|[Unique Paths III](https://leetcode.com/problems/unique-paths-iii)|backtracking depth-first-search|[solution](rustgym/src/leetcode/_980_unique_paths_3.rs)|
|982|[Triples with Bitwise AND Equal To Zero](https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero)|dynamic-programming|[solution](rustgym/src/leetcode/_982_triples_with_bitwise_and_equal_to_zero.rs)|
|99|[Recover Binary Search Tree](https://leetcode.com/problems/recover-binary-search-tree)|tree depth-first-search|[solution](rustgym/src/leetcode/_99_recover_binary_search_tree.rs)|
</details>

### Coding Interview
Leetcode is a website where people–mostly software engineers–practice their coding skills. There are 1200+ questions (and growing), each with multiple solutions. Questions are ranked by level of difficulty: easy, medium, and hard. Within the last decade or so, the technical interview process has become formulaic and what some describe “unnatural” for engineers. What people are asked to perform in an interview–solving word or code based teasers, coding on a whiteboard, and being asked to produce clean optimized solutions in a short time frame–is not what they would experience in a daily work environment.


![test svg](./test.svg)

