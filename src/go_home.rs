pub fn go_home(u_arr: Vec<usize>) -> Vec<usize> {
    let mut length: usize = u_arr[0] + 1;
    let mut o_arr: Vec<usize> = vec![0; length];

    for i in &u_arr {
        if length < i + 1 {
            let n_len: usize = (i + 1) - length;
            length = i + 1;
            o_arr.extend_from_slice(&vec![0; n_len]);
        }

        o_arr[*i] += 1;
    
    }

    let mut lst: Vec<usize> = Vec::new();
    for (indx, val) in o_arr.iter().enumerate() {
        if *val != 0 {
            lst.extend(vec![indx; *val]);
        }
    }

    return lst;
}
