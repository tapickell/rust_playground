use std::fmt::Debug;

mod b_rand;

// O(n^2) operations
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1)
            }
        }
    }
}

// worst case still O(n^2)
pub fn bubble_sort_2<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        let mut sorted = true;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

// close to O(n^2)
// can do one less element test on end for each iteration of p
// 10
// 9
// 8
pub fn bubble_sort_3<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

// use Vec so memory is on heap not stack
// pub fn merge_sort_1<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
//     // sort left O(n * ln(n))
//     // sort right O(n * ln(n))
//     // bring halves together O(n)

//     if v.len() <= 1 {
//         return v; // sorted
//     }

//     let mut res = Vec::with_capacity(v.len());
//     let b = v.split_off(v.len() / 2); // mutates v
//     let a = merge_sort_1(v);
//     let b = merge_sort_1(b);

//     let mut a_it = a.iter();
//     let mut b_it = b.iter();
//     let mut a_peek = a_it.next();
//     let mut b_peek = b_it.next();

//     loop {
//         match a_peek {
//             Some(ref a_val) => match b_peek {
//                 Some(ref b_val) => {
//                     if b_val < a_val {
//                         res.push(b_peek.take().unwrap());
//                         b_peek = b_it.next();
//                     } else {
//                         res.push(a_peek.take().unwrap());
//                         a_peek = a_it.next();
//                     }
//                 }
//                 None => {
//                     res.push(a_peek.take().unwrap());
//                     res.extend(a_it);
//                     return res;
//                 }
//             },
//             None => {
//                 if let Some(b_val) = b_peek {
//                     res.push(b_val);
//                 }
//                 res.extend(b_it);
//                 return res;
//             }
//         }
//     }
// }

// Move first element to the correct place
// Everything lower should go before it
// Everything higher should go after it
// return it's location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    p
}

// If already sorted runs at n^2
// If not then can be better
pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}

// uses 2^log(n) threads
pub fn quick_sort_threaded<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);
    unsafe {
        let handle = std::thread::spawn(move || {
            quick_sort_threaded(&mut *raw_s.0);
        });
        quick_sort_threaded(&mut b[1..]);
        handle.join().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let expected = vec![1, 3, 4, 6, 8, 11, 13];

        let mut v1 = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v1);

        assert_eq!(v1, expected);
    }

    #[test]
    fn two_works() {
        let expected = vec![1, 3, 4, 6, 8, 11, 13];
        let mut v2 = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort_2(&mut v2);

        assert_eq!(v2, expected);
    }

    #[test]
    fn three_works() {
        let expected = vec![1, 3, 4, 6, 8, 11, 13];
        let mut v3 = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort_3(&mut v3);

        assert_eq!(v3, expected);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
        // assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }

    #[test]
    fn test_quick_sort() {
        let expected = vec![1, 3, 4, 6, 8, 11, 13];

        let mut v1 = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v1);

        assert_eq!(v1, expected);
        println!("v1 completed");

        let mut sorted = vec![1, 3, 4, 6, 8, 11, 13];
        quick_sort(&mut sorted);

        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_quick_sort_threaded() {
        let expected = vec![1, 1, 3, 4, 5, 6, 8, 8, 11, 11, 13, 56];

        let mut v1 = vec![4, 6, 1, 8, 11, 13, 3, 5, 11, 56, 8, 1];
        quick_sort_threaded(&mut v1);

        assert_eq!(v1, expected);
    }
}
