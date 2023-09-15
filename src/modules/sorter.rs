pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot_index = partition(arr);
    sort(&mut arr[0..pivot_index]);
    sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = choose_pivot_index(len);
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

/// Choose a pivot strategy here. 
/// We default to using the last element as pivots.
fn choose_pivot_index(len: usize) -> usize {
    len - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort(){
        let mut s = vec![6, 2, 1, 7, 3];
        sort(&mut s);

        println!("{:?}", s)
    }
}