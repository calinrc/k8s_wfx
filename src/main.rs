use consts::FILETIME;
use consts::HANDLE;
use consts::INVALID_HANDLE;
use consts::WIN32_FIND_DATAA;
use iterators::FindDataUpdater;
use iterators::ResourcesItertatorFactory;
use std::path::Path;

mod consts;
mod iterators;
mod resources;
mod helper;


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

    let mut rit = ResourcesItertatorFactory::new(Path::new(""));
    unsafe {
        let handle = {
            let handle = match rit.next() {
                Some(_) => {
                    rit.update_find_data(&mut find_data);
                    let thin_ptr = Box::new(rit);
                    let mbrit = Box::into_raw(thin_ptr);
                    mbrit as *mut _ as HANDLE
                }
                None => INVALID_HANDLE,
            };
            handle
        };
        if handle != INVALID_HANDLE {
            let riit = handle as *mut Box<dyn FindDataUpdater>;
            //as *mut ResourcesIterator;
            match (*riit).next() {
                Some(_) => {
                    (*riit).update_find_data(&mut find_data);
                }
                None => println!("None elem"),
            }
        }
    }
    println!("End");
}
