use iterator::ResourcesIterator;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;
use consts::HANDLE;
use consts::INVALID_HANDLE;
use consts::WIN32_FIND_DATAA;
use consts::FILETIME;


mod consts;
mod resources;
mod pods;
mod iterator;

fn main(){
    println!("Start");
    let mut find_data:WIN32_FIND_DATAA = WIN32_FIND_DATAA{
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
        c_file_name: [0i8;260],
        c_alternate_file_name: [0i8;14],
    };

    let mut rit = ResourcesIterator::new();
    let it = rit.iterator();
    let handle = {
        let handle = match it.next(){
            Some(next_elem) => {let mut rit = ManuallyDrop::new(rit);
                                                ResourcesIterator::update_find_data(&mut find_data, next_elem);
                                                &mut rit as *mut _ as HANDLE
            },
            None => INVALID_HANDLE,
        };
        handle
    };
    if handle != INVALID_HANDLE {
        let riit: &mut ResourcesIterator = unsafe { &mut *(handle as *mut ResourcesIterator) };
        //as *mut ResourcesIterator;
        let it = riit.iterator();
        match it.next() {
            Some(next_elem) => {
                ResourcesIterator::update_find_data(&mut find_data, next_elem);
                },
                None => println!("None elem"),
        }

    }
    println!("End");

}