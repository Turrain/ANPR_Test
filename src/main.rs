use libc::c_int;
use std::{
    ffi::{c_char, CStr, CString},
    mem,
    os::raw::c_void,
    ptr,
};
use ANPR_bind as cv;

fn allocate_array(all: usize) -> *mut *mut c_char {
    unsafe {
        // Allocate an array of `*mut c_char`
        let res = libc::malloc(all * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;

        if res.is_null() {
            return ptr::null_mut();
        }

        // Allocate each entry with a pointer to a new array of 20 `c_char`
        for j in 0..all {
            let inner_array = libc::malloc(20 * std::mem::size_of::<c_char>()) as *mut c_char;
            if inner_array.is_null() {
                // In case of failure, deallocate previously allocated memory
                for k in 0..j {
                    libc::free(*res.add(k) as *mut libc::c_void);
                }
                libc::free(res as *mut libc::c_void);
                return ptr::null_mut();
            }
            *res.add(j) = inner_array;
        }

        res
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments. For help print {} /?", args[0]);
        return Ok(());
    } else if args[1] == "help" || args[1] == "-help" || args[1] == "--help" || args[1] == "/?" {
        return Ok(());
    } else if args.len() < 3 {
        println!("Too few arguments. For help print {} /?", args[0]);
        return Ok(());
    }
    let img_path = CString::new(args[2].clone()).expect("CString::new failed");
    let save = CString::new("gray.jpg").expect("CString::new failed");
    let img = unsafe { cv::cvLoadImage(img_path.as_ptr(), cv::CV_LOAD_IMAGE_COLOR) };

   // unsafe { cv::cvSaveImage(save.as_ptr(), img as *mut c_void, ptr::null_mut()) };

    println!("{:?}", img);
    let mut all: c_int = 100;
    let mut rects = vec![
        cv::CvRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0
        };
        100
    ];
    let rects_ptr: *mut cv::CvRect = rects.as_mut_ptr();
    let res = allocate_array(all.try_into().unwrap());
    let mut ver = CString::new("1.6.0").unwrap();
    let ver_raw_ptr: *mut i8 = ver.into_raw();
    let a = cv::ANPR_OPTIONS {
        sign1: b'i' as i8,
        sign2: b'a' as i8,
        sign3: b'1' as i8,
        min_plate_size: 500,
        max_plate_size: 50000,
        Detect_Mode: cv::ANPR_DETECTCOMPLEXMODE as i32,
        max_text_size: 20,
        type_number: 104,
        flags: 0,
        custom: ptr::null_mut(),
        vers: ver_raw_ptr,
        alpha: 90.0,
        beta: 90.0,
        gamma: 90.0,
        max_threads: 1,
    };
    let anpr_full_types = vec![4, 7, 9, 310, 311, 911]; 
    let is_full_type = false; //TODO:
    let i;
    if is_full_type {
        unsafe {
            i = cv::anprPlate(img, a, &mut all, rects_ptr, res, std::ptr::null_mut());
            println!("{:?}", i);
        }
    } else {
        unsafe {
            let size = cv::CvSize{width:1193,height:671}; //TODO: CVGETSIZE dont work
            let gray = cv::cvCreateImage(size, 8, 1);
            cv::cvCvtColor(img as *mut cv::CvArr, gray as *mut cv::CvArr, cv::CV_BGR2GRAY);
            cv::cvSaveImage(save.as_ptr(), gray as *mut c_void, ptr::null_mut());
            i = cv::anprPlate(
                gray,
                a,
                &mut all,
                rects_ptr,
                res,
                std::ptr::null_mut(),
            );
        }
    }
    if i == 0 {
        unsafe {
            for j in 0..all as usize {
                if !(*res.add(j)).is_null() {
                    let c_str = CStr::from_ptr(*res.add(j));
                    println!("{}", c_str.to_str()?);
                }
            }
        }
    } else {
        println!("Error: {}", i);
    }
    unsafe {
        for j in 0..all {
            libc::free(*res.add(j.try_into().unwrap()) as *mut libc::c_void);
        }
        libc::free(res as *mut libc::c_void);
    }
    Ok(())
}
