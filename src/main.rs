mod raw_pointer {
    pub fn run() {
        let mut num: i32 = 10;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // unsafe block
        unsafe {
            *r2 = 20;
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
}

mod unsafe_function {

    pub unsafe fn dangerous() {
        println!("dangerous function called");
    }
}

mod safe_abstraction {
    pub fn safe_function() {
        let mut v = vec![1, 2, 3, 4, 5];
        
        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);
        println!("r is: {:?}", r);
    }

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();// slice as *mut [i32] as *mut i32

        assert!(mid <= len);

        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    
}

#[link(name = "src/my_c_lib")]
extern {
    fn add_two_numbers(a: i32, b: i32) -> i32;
}

pub extern fn call_add_two_numbers(a: i32, b: i32) -> i32 {
    unsafe {
        add_two_numbers(a, b)
    }
}

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}


fn main() {
    raw_pointer::run();

    // unsafe_function::dangerous(); this will not work
    unsafe {
        unsafe_function::dangerous();
    }

    safe_abstraction::safe_function();

    let input1 = 10;
    let input2 = 20;
    let output = unsafe {
        add_two_numbers(input1, input2)
    }; // or call_add_two_numbers(input1, input2);
    println!("output is: {}", output);

    let u = MyUnion {f1: 42};
    let f = unsafe {
        u.f2
    };

    unsafe {
        match u {
            MyUnion {f1: 43} => println!("f1 is 42"),
            MyUnion {f2} => println!("f2 is {}", f2),
        }
    }
}
