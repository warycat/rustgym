fn main() {
    let word = "goodbye";

    let count = word.char_indices().count();
    assert_eq!(7, count);

    let mut char_indices = word.char_indices();
    println!("{:?}",char_indices); // CharIndices { iter: Chars { iter: <core::str::split::SplitWhitespace<'_>> } }

    let first = char_indices.next(); // CharIndices.next() returns an Option<(usize, char)>
    assert_eq!(Some((0, 'g')), first);

    // assert_eq!(Some((0, 'g')), char_indices.next());
    assert_eq!(Some((1, 'o')), char_indices.next());
    assert_eq!(Some((2, 'o')), char_indices.next());
    assert_eq!(Some((3, 'd')), char_indices.next());
    assert_eq!(Some((4, 'b')), char_indices.next());
    assert_eq!(Some((5, 'y')), char_indices.next());
    assert_eq!(Some((6, 'e')), char_indices.next());

    assert_eq!(None, char_indices.next());
}
