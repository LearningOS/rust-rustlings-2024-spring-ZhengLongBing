/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


use std::vec;

fn sort<T:Ord+Copy+core::fmt::Debug>(array: &mut [T]){
	fn quick_sort<T:Ord+Copy>(arr:Vec<T>)->Vec<T>{
        match arr.as_slice() {
            []=>vec![],
            [x]=>vec![*x],
            [prev,next @ .. ]=>{
                let front=next.iter().filter(|&x| x<=prev).copied().collect();
                let back=next.iter().filter(|&x| x>prev).copied().collect();
                let front=quick_sort(front).into_iter();
                let back=quick_sort(back).into_iter();
                let mid=Some(*prev).into_iter();
                front.chain(mid).chain(back).collect()
            }
        }
    }
    let vec=quick_sort(array.iter().copied().collect());
    println!("{:?}",vec);
    array.iter_mut().zip(vec.into_iter())
        .map(|(x,y)|*x=y)
        .for_each(|_|{});
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}