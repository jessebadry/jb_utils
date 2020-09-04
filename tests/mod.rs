#[cfg(test)]
mod tests {
   use jb_utils::extensions::io::EasyRead;
   use jb_utils::extensions::strings::StringExt;
   use jb_utils::io_err;
   use std::io::ErrorKind;
   use std::io::Write;
   use std::result;

   type TResult<T = (), E = Box<dyn std::error::Error>> = result::Result<T, E>;
   static TEST_FILE: &str = "test_text.txt";
   fn write_test_content<T: Write>(writer: &mut T) -> TResult<()> {
      for _ in 0..1000 {
         writer.write(b"Bruh!\n")?;
      }

      Ok(())
   }

   #[test]
   fn make_err_pat1() {
      let msg = "Error Message";
      let err = io_err!(msg);
      assert_eq!(err.kind(), ErrorKind::Other);
      assert_eq!(err.to_string(), msg);
   }
   #[test]
   fn make_err_pat2() {
      let msg = "Had a Already exists exception";
      let err = io_err!(ErrorKind::AlreadyExists, msg);
      assert_eq!(ErrorKind::AlreadyExists, err.kind());
      assert_eq!(msg, err.to_string());
   }
   #[test]
   fn mult_string() {
      let string = String::from("bruh");
      assert_eq!(string.mul(3), "bruhbruhbruh".to_string());
      println!("string = {}", string);
   }

   mod io_tests {
      use super::*;

      use std::fs::File;
      fn gen_text_file(fname: &str) -> TResult<File> {
         let string = String::from("test");
         let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(fname)
            .and_then(|mut file| {
               let meta = file.metadata()?;

               if meta.len() <= 16 {
                  file.write_all(string.mul(100).as_bytes())?;
               }

               Ok(file)
            })?;

         Ok(file)
      }
      #[test]
      fn e_read_test() {
         let mut file = std::fs::File::open(TEST_FILE).unwrap_or_else(|_| {
            let mut file = std::fs::File::create(TEST_FILE).unwrap_or_else(|e| {
               panic!("Could not create test_text file for read_test. why: {}", e)
            });
            write_test_content(&mut file)
               .expect(&format!("Could not write test content, ln {}", line!()));
            file
         });
   
         let mut r = 0;
         let buffer = &mut [0u8; 1000];
         while file.e_read(buffer, &mut r).unwrap_or(0) > 0 {
            //
         }
      }
      #[test]
      fn inplace_read() -> TResult<()> {
         let mut file = gen_text_file("inplace_read.txt")?;
         //read 16 bytes
         let bytes = file.read_inplace(16)?;
         assert_eq!(bytes.len(), 16);
         Ok(())
      }
   }
}
