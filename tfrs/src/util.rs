pub fn hello_tfrs() -> String {
    "tfrs".to_string()
}

pub fn size_from_shape(shape: &[usize]) -> usize {
    let mut prod = 1;
    for v in shape {
        prod *= v;
    }
    prod
}

#[test]
fn test_size_from_shape() {
    let shape = vec![];
    assert_eq!(1, size_from_shape(&shape));
    let shape = vec![3];
    assert_eq!(3, size_from_shape(&shape));
    let shape = vec![3, 4];
    assert_eq!(12, size_from_shape(&shape));
    let shape = vec![1, 3, 5];
    assert_eq!(15, size_from_shape(&shape));
    let shape = vec![2, 3, 4];
    assert_eq!(24, size_from_shape(&shape));
    let shape = vec![2, 3, 4, 5];
    assert_eq!(120, size_from_shape(&shape));
}

pub fn offset_to_loc(mut index: usize, strides: &[usize]) -> Vec<usize> {
    let rank = strides.len() + 1;
    let mut loc = vec![0; rank];
    for i in 0..rank - 1 {
        let stride = strides[i];
        loc[i] = index / stride;
        index -= loc[i] * stride;
    }
    loc[rank - 1] = index;
    loc
}

#[test]
fn test_offset_to_loc() {
    let offset: usize = 5;
    let strides = vec![];
    assert_eq!(vec![5], offset_to_loc(offset, &strides));
    let offset: usize = 26;
    let strides = vec![7];
    assert_eq!(vec![3, 5], offset_to_loc(offset, &strides));
    let offset: usize = 5;
    let strides = vec![];
    assert_eq!(vec![5], offset_to_loc(offset, &strides));
    let offset: usize = 51;
    let strides = vec![8, 4];
    assert_eq!(vec![6, 0, 3], offset_to_loc(offset, &strides));
    let offset: usize = 67;
    let strides = vec![8, 4, 2];
    assert_eq!(vec![8, 0, 1, 1], offset_to_loc(offset, &strides));
}

pub fn loc_to_offset(loc: &[usize], strides: &[usize]) -> usize {
    let rank = loc.len();
    if rank == 0 {
        return 0;
    }
    let mut index = loc[rank - 1];
    for i in 0..rank - 1 {
        index += strides[i] * loc[i];
    }
    index
}

#[test]
fn test_loc_to_offset() {
    let loc = vec![];
    let strides = vec![];
    assert_eq!(0, loc_to_offset(&loc, &strides));
    let loc = vec![5];
    let strides = vec![];
    assert_eq!(5, loc_to_offset(&loc, &strides));
    let loc = vec![3, 5];
    let strides = vec![7];
    assert_eq!(26, loc_to_offset(&loc, &strides));
    let loc = vec![6, 0, 3];
    let strides = vec![8, 4];
    assert_eq!(51, loc_to_offset(&loc, &strides));
    let loc = vec![8, 0, 1, 1];
    let strides = vec![8, 4, 2];
    assert_eq!(67, loc_to_offset(&loc, &strides));
}

pub fn offset_2d(i1: usize, i2: usize, s1: usize) -> usize {
    i1 * s1 + i2
}

#[test]
fn test_offset_2d() {
    let coord = vec![0, 0];
    let stride = vec![0];
    assert_eq!(0, offset_2d(coord[0], coord[1], stride[0]));
    let coord = vec![2, 3];
    let stride = vec![5];
    assert_eq!(13, offset_2d(coord[0], coord[1], stride[0]));
}

pub fn offset_3d(i1: usize, i2: usize, i3: usize, s1: usize, s2: usize) -> usize {
    i1 * s1 + i2 * s2 + i3
}

#[test]
fn test_offset_3d() {
    let coord = vec![0, 0, 0];
    let stride = vec![0, 0];
    assert_eq!(
        0,
        offset_3d(coord[0], coord[1], coord[2], stride[0], stride[1])
    );
    let coord = vec![3, 5, 7];
    let stride = vec![4, 3];
    assert_eq!(
        34,
        offset_3d(coord[0], coord[1], coord[2], stride[0], stride[1])
    );
}

pub fn offset_4d(
    i1: usize,
    i2: usize,
    i3: usize,
    i4: usize,
    s1: usize,
    s2: usize,
    s3: usize,
) -> usize {
    i1 * s1 + i2 * s2 + i3 * s3 + i4
}

#[test]
fn test_offset_4d() {
    let coord = vec![0, 0, 0, 0];
    let stride = vec![0, 0, 0];
    assert_eq!(
        0,
        offset_4d(coord[0], coord[1], coord[2], coord[3], stride[0], stride[1], stride[2])
    );
    let coord = vec![1, 2, 3, 4];
    let stride = vec![5, 7, 9];
    assert_eq!(
        50,
        offset_4d(coord[0], coord[1], coord[2], coord[3], stride[0], stride[1], stride[2])
    );
}

pub fn offset_5d(
    i1: usize,
    i2: usize,
    i3: usize,
    i4: usize,
    i5: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
) -> usize {
    i1 * s1 + i2 * s2 + i3 * s3 + i4 * s4 + i5
}

#[test]
fn test_offset_5d() {
    let coord = vec![0, 0, 0, 0, 0];
    let stride = vec![0, 0, 0, 0];
    assert_eq!(
        0,
        offset_5d(
            coord[0], coord[1], coord[2], coord[3], coord[4], stride[0], stride[1], stride[2],
            stride[3]
        )
    );
    let coord = vec![1, 2, 3, 4, 5];
    let stride = vec![5, 7, 9, 11];
    assert_eq!(
        95,
        offset_5d(
            coord[0], coord[1], coord[2], coord[3], coord[4], stride[0], stride[1], stride[2],
            stride[3]
        )
    );
}

pub fn compute_strides(shape: &[usize]) -> Vec<usize> {
    let rank = shape.len();
    let mut strides = vec![0; rank - 1];
    if rank < 2 {
        return strides;
    }

    strides[rank - 2] = shape[rank - 1];

    for i in (0..rank - 2).rev() {
        strides[i] = strides[i + 1] * shape[i + 1];
    }

    strides
}

#[test]
fn test_compute_strides() {
    let shape = vec![5, 7];
    assert_eq!(vec![7], compute_strides(&shape));
    let shape = vec![3, 5, 7];
    assert_eq!(vec![35, 7], compute_strides(&shape));
    let shape = vec![3, 5, 7, 9];
    assert_eq!(vec![315, 63, 9], compute_strides(&shape));
    let shape = vec![2, 3, 5, 7, 9];
    assert_eq!(vec![945, 315, 63, 9], compute_strides(&shape));
    let shape = vec![2, 2, 2, 2, 2, 2];
    assert_eq!(vec![32, 16, 8, 4, 2], compute_strides(&shape));
}
