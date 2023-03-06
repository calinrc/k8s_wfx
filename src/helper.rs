
pub fn to_file_time(milisec: i64)->i64{
    milisec * 10000 + 116444736000000000i64
}

/***    
    * milisec - in milliseconds since January 1, 1970 UTC.
	 * @return fileTime - number of 100-nanosecond intervals since January 1,
	 *         1601.
 */
pub fn to_split_file_time(milisec: i64) -> (i32, i32){
    let ft = to_file_time(milisec);
    (ft as i32, (ft >> 32) as i32 )
}