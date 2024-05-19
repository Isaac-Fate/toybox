pub fn insertion_sort<T: PartialOrd>(ordered_array: &mut [T]) {
    for j in 1..ordered_array.len() {
        let value = &ordered_array[j];

        let mut i = j - 1;

        // k is the position to insert
        let k = loop {
            let other_value = &ordered_array[i];
            if value >= other_value {
                break i + 1;
            }

            if i == 0 {
                break 0;
            }

            i -= 1;
        };

        if k == j {
            continue;
        }

        // Insert the value to the desired position by rotating part of the array
        ordered_array[k..=j].rotate_right(1);
    }
}

unsafe fn _insertion_sort_unsafe<T: PartialOrd>(ordered_array: &mut [T]) {
    for j in 1..ordered_array.len() {
        let key = unsafe { std::ptr::read(&ordered_array[j]) };

        let mut i = j - 1;
        while key < ordered_array[i] {
            // let other = unsafe { std::ptr::read(&ordered_array[i]) };

            // ordered_array[i + 1] = other;

            unsafe {
                let other = std::ptr::read(&ordered_array[i]);

                println!("other's address: {:p}", &other);

                std::ptr::write(&mut ordered_array[i + 1], other);
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }

        let k = if i > 0 { i + 1 } else { 0 };

        ordered_array[k] = key;

        // unsafe {
        //     std::ptr::write(&mut ordered_array[k], key);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name && self.age == other.age
        }
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.age.partial_cmp(&other.age)
        }
    }

    #[test]
    fn test_rotate() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        array[2..9].rotate_right(1);
        println!("{:?}", array);
    }

    #[test]
    fn sort_float_numbers() {
        // let mut array = [1.0, 42.0, 3.0, 100.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mut array = [0, -1, 2, 1, 1];
        insertion_sort(&mut array);
        println!("{:?}", array);
    }

    #[test]
    fn sort_people() {
        let mut people = [
            Person {
                name: "Isaac".to_string(),
                age: 24,
            },
            Person {
                name: "Jane".to_string(),
                age: 18,
            },
            Person {
                name: "John".to_string(),
                age: 30,
            },
        ];

        insertion_sort(&mut people);
        println!("{:?}", people);
    }

    #[test]
    fn double_drop() {
        let s = "Hello, world!".to_string();

        let mut tmp = String::new();
        let tmp_ptr: *mut String = &mut tmp;
        unsafe {
            std::ptr::write(tmp_ptr, std::ptr::read(&s));
        }

        println!("s: {:?}", s.as_ptr());
        println!("tmp: {:?}", tmp.as_ptr());
        std::mem::forget(tmp);
    }
}
