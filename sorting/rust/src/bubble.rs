pub fn sort(array: &mut [i32]) -> () {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j+1] {
                let tmp = array[j];
                array[j] = array[j+1];
                array[j+1] = tmp;
            }
        }
    }

    return;
}
