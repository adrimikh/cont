use std::fmt;
use std::process::exit;

#[allow(unreachable_patterns)] //Tells the compiler to suppress warnings about match patterns that cannot be reached.

// Allows to display a variant with the format {:?}
#[derive(Debug)]
//Enum that contains all possible errors in the program.
pub enum Errcode {
    ArgumentInvalid(&'static str), //static here means that str must live for the whole program.
}

//Impl block because methods can only be defined in impl of enum.
impl Errcode{
  pub fn get_retcode(&self) -> i32 {
      1 // Everything different than 0 will be treated as an error.
  }
}

//Implementing Display trait (from std::fmt) for Errcode. Errcode -> Human-Readable String.
impl fmt::Display for Errcode {

  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      // Define what behaviour for each variant of the enum
      match &self{
        Errcode::ArgumentInvalid(el) => write!(f, "ArgumentInvalid: {}", el),
        _ => write!(f, "{:?}", self) // For any variant not previously covered
      }
  }
}

//Get the result from a function and exit the process with an error code.
pub fn exit_with_errcode(res: Result<(), Errcode>) {
  match res {
      Ok(_) => {
          log::debug!("Exit without any error, returning 0");
          exit(0);
      }

      Err(e) => {
          let retcode = e.get_retcode();
          log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
          exit(retcode);
      }
  }
}