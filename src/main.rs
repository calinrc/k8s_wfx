use consts::FILETIME;
use consts::WIN32_FIND_DATAA;
use iterators::ResourcesIteratorFactory;
use std::path::Path;

mod consts;
mod helper;
mod iterators;
mod resources;

fn main() {
    println!("Start");
    let mut find_data: WIN32_FIND_DATAA = WIN32_FIND_DATAA {
        // unsafe {
        //     MaybeUninit::zeroed().assume_init()
        // };
        dw_file_attributes: 0,
        ft_creation_time: FILETIME::default(),
        ft_last_access_time: FILETIME::default(),
        ft_last_write_time: FILETIME::default(),
        n_file_size_high: 0,
        n_file_size_low: 0,
        dw_reserved_0: 0,
        dw_reserved_1: 0,
        c_file_name: [0i8; 260],
        c_alternate_file_name: [0i8; 14],
    };

    let mut rit = ResourcesIteratorFactory::new(Path::new("/docker-desktop/pod"));
    let mut next = rit.next();
    while next.is_some() {
        unsafe {
            rit.update_find_data(&mut find_data);
        }
        // let thin_ptr = Box::new(rit);
        // let mbrit = Box::into_raw(thin_ptr);
        // handle = mbrit as *mut _ as HANDLE;
        next = rit.next()
    }

    let mut rit = ResourcesIteratorFactory::new(Path::new("/"));
    let mut next = rit.next();
    while next.is_some() {
        unsafe {
            rit.update_find_data(&mut find_data);
        }
        // let thin_ptr = Box::new(rit);
        // let mbrit = Box::into_raw(thin_ptr);
        // handle = mbrit as *mut _ as HANDLE;
        next = rit.next()
    }

    let mut rit = ResourcesIteratorFactory::new(Path::new("/docker-desktop"));
    let mut next = rit.next();
    while next.is_some() {
        unsafe {
            rit.update_find_data(&mut find_data);
        }
        // let thin_ptr = Box::new(rit);
        // let mbrit = Box::into_raw(thin_ptr);
        // handle = mbrit as *mut _ as HANDLE;
        next = rit.next()
    }

    let mut rit = ResourcesIteratorFactory::new(Path::new("/docker-desktop/pod"));
    let mut next = rit.next();
    while next.is_some() {
        unsafe {
            rit.update_find_data(&mut find_data);
        }
        // let thin_ptr = Box::new(rit);
        // let mbrit = Box::into_raw(thin_ptr);
        // handle = mbrit as *mut _ as HANDLE;
        next = rit.next()
    }



    println!("End");
}
