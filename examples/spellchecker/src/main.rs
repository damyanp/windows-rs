use bindings::Windows::Win32;
use Win32::Intl;
use Win32::SystemServices::PWSTR;

fn main() -> windows::Result<()> {
    let input = std::env::args()
        .nth(1)
        .expect("Expected one command line argument for text to be spell-corrected");
    // Initialize the COM runtime for this thread
    windows::initialize_mta()?;

    // Create ISpellCheckerFactory
    let factory: Intl::ISpellCheckerFactory = windows::create_instance(&Intl::SpellCheckerFactory)?;

    // Make sure that the "en-US" locale is supported
    let supported = unsafe { factory.IsSupported("en-US")? };
    supported.expect("en-US is supported");

    // Create a ISpellChecker
    let checker = unsafe { factory.CreateSpellChecker("en-US")? };

    println!("Checking the text: '{}'", input);

    // Get errors enumerator for the supplied string
    let errors = unsafe { checker.ComprehensiveCheck(input.clone())? };

    // Loop through all the errors
    loop {
        // Get the next error in the enumerator
        let result = unsafe { errors.Next() };

        let error = if let Ok(error) = result {
            error
        } else {
            break;
        };

        // Get the start index and length of the error
        let start_index = unsafe { error.get_StartIndex()? };
        let length = unsafe { error.get_Length()? };

        // Get the substring from the ut8 encoded string
        let substring = &input[start_index as usize..(start_index + length) as usize];

        // Get the corrective action
        let action = unsafe { error.get_CorrectiveAction()? };
        println!("{:?}", action);

        match action {
            Intl::CORRECTIVE_ACTION::CORRECTIVE_ACTION_DELETE => {
                println!("Delete '{}'", substring);
            }
            Intl::CORRECTIVE_ACTION::CORRECTIVE_ACTION_REPLACE => {
                // Get the replacement as a widestring and convert to a Rust String
                let replacement = unsafe { error.get_Replacement()? };

                println!("Replace: {} with {}", substring, unsafe {
                    read_to_string(replacement)
                });
            }
            Intl::CORRECTIVE_ACTION::CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                // Get an enumerator for all the suggestions for a substring
                let suggestions = unsafe { checker.Suggest(substring)? };

                // Loop through the suggestions
                loop {
                    // Get the next suggestion breaking if the call to `Next` failed
                    let mut suggestion = PWSTR::default();
                    let result =
                        unsafe { suggestions.Next(1, &mut suggestion, std::ptr::null_mut()) };
                    if result == windows::ErrorCode::S_FALSE {
                        break;
                    }
                    result.ok()?;

                    println!("Maybe replace: {} with {}", substring, unsafe {
                        read_to_string(suggestion)
                    });
                }
            }
            _ => {}
        }
    }
    Ok(())
}

unsafe fn read_to_string(ptr: PWSTR) -> String {
    let mut len = 0usize;
    let mut cursor = ptr;
    loop {
        let val = cursor.0.read();
        if val == 0 {
            break;
        }
        len += 1;
        cursor = PWSTR(cursor.0.add(1));
    }

    let slice = std::slice::from_raw_parts(ptr.0, len);
    String::from_utf16(slice).unwrap()
}
