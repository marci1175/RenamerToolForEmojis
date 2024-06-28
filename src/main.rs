use std::{env::args, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args();
    let self_path = PathBuf::from(args.next().unwrap());
    let self_name = self_path.file_name().unwrap();

    if let Some(arg1) = args.next() {
        println!("WARNING: THIS PROGRAM DOES DESTRUCTIVE CHANGES\n(IT WILL TRIM THE FIRST 5 CHARACTERS OF EVERY FILE OF THE FILES CONTAINED IN THE FOLDER'S PATH PASSED IN)");
        println!("Entered path: {arg1}");

        std::io::stdin().read_line(&mut String::new())?;

        let read_dir = fs::read_dir(arg1)?;

        for path_item in read_dir {
            let file_name = path_item?.file_name();

            //We dont want to rename ourselves
            if file_name == *self_name {
                continue;
            }

            let trimmed_name = file_name.to_string_lossy();

            let trimmed_name_final = trimmed_name.get(5..).unwrap_or_default();

            fs::rename(file_name.clone(), trimmed_name_final)?;

            println!("Renamed {:?} to {:?}", file_name.clone(), trimmed_name_final)
        }
    }
    else {
        println!("Did not input a valid base dierectory path!");
    }

    

    Ok(())
}
