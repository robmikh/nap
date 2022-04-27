pub fn split_args(input: &str) -> Vec<String> {
    #[cfg(target_os = "windows")]
    return split_args_windows(input);

    #[cfg(not(target_os = "windows"))]
    return split_args_not_windows(input);
}

#[cfg(target_os = "windows")]
fn split_args_windows(input: &str) -> Vec<String> {
    unsafe {
        let mut arg_count = 0;
        let arg_ptr = windows::Win32::UI::Shell::CommandLineToArgvW(input, &mut arg_count);

        if arg_ptr.is_null() {
            panic!();
        }

        let mut args = Vec::new();
        for i in 0..arg_count {
            let arg = read_utf16_string(*(arg_ptr.add(i as usize)));
            args.push(arg);
        }
        args
    }
}

#[cfg(target_os = "windows")]
fn read_utf16_string(pwstr: windows::core::PWSTR) -> String {
    let mut len: usize = 0;
    let string_start_ptr: *const u16 = pwstr.0;
    let mut string_end_ptr: *const u16 = string_start_ptr;
    unsafe {
        while *string_end_ptr != 0u16 {
            string_end_ptr = string_end_ptr.add(1);
            len += 1;
        }
    };
    let string = unsafe {
        let string_data = std::slice::from_raw_parts(string_start_ptr, len);
        String::from_utf16(string_data).unwrap()
    };
    string
}

#[cfg(not(target_os = "windows"))]
fn split_args_not_windows(input: &str) -> Vec<String> {
    shell_words::split(input).unwrap()
}
