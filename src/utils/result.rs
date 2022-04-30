use std::fmt::Display;
use std::io::{Error as IoError, Result as IoResult};

/// A trait for describing errors when calling functions that return [`Result`]s.
///
/// See the [`describe_err`] function documentation for details.
///
/// [`describe_err`]: trait.DescribeErr.html#tymethod.describe_err
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub trait DescribeErr {
  /// Prepends `description` to the error message if the result is an [`Err`]
  /// value, leaving an [`Ok`] value untouched.
  ///
  /// # Why?
  ///
  /// Consider the following function:
  ///
  /// ```
  /// fn copy_file(src: &Path, dest: &Path) -> io::Result<u64> {
  ///   let mut src_file = File::open(src)?;
  ///   let mut dest_file = File::create(dest)?;
  ///   io::copy(&mut src_file, &mut dest_file)
  /// }
  /// ```
  ///
  /// It's pretty simple: it just opens a source file, creates a destination file,
  /// copies contents of the first file to the second one and returns how many
  /// bytes were copied. But what if one of the files has wrong permissions? Well,
  /// this function will return an error, but the user won't know what file caused
  /// this error, so he/she would have to manually check both files, though
  /// **error logs must help to find a bug that has caused the error**.
  ///
  /// So, let's add the [`describe_err`] function to solve this problem:
  ///
  /// ```
  /// use utils::result::DescribeErr;
  ///
  /// fn copy_file(src: &Path, dest: &Path) -> io::Result<u64> {
  ///   let mut src_file = File::open(src)
  ///     .describe_err("cannot open source file")?;
  ///   let mut dest_file = File::create(dest)
  ///     .describe_err("cannot create destination file")?;
  ///   io::copy(&mut src_file, &mut dest_file)
  ///     .describe_err("cannot copy")
  /// }
  /// ```
  ///
  /// Now, if something bad happens, this function will return an error with
  /// message like this:
  ///
  /// ```
  /// cannot open source file: No such file or directory
  /// ```
  ///
  /// We can improve the code even more by passing paths to [`describe_err`],
  /// because it can use any type that implements [`Display`]:
  ///
  /// ```
  /// use utils::result::DescribeErr;
  ///
  /// fn copy_file(src: &Path, dest: &Path) -> io::Result<u64> {
  ///   let mut src_file = File::open(src)
  ///     .describe_err(src.display())?;
  ///   let mut dest_file = File::create(dest)
  ///     .describe_err(dest.display())?;
  ///   io::copy(&mut src_file, &mut dest_file)
  ///     .describe_err("cannot copy")
  /// }
  /// ```
  ///
  /// This version will give you an error like this:
  ///
  /// ```
  /// path/to/file/a: Permission denied
  /// ```
  ///
  /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
  /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
  /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
  /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
  /// [`describe_err`]: trait.DescribeErr.html#tymethod.describe_err
  fn describe_err<D: Display + Sized>(self, description: D) -> Self;
}

impl<T> DescribeErr for IoResult<T> {
  fn describe_err<D: Display + Sized>(self, description: D) -> Self {
    self.map_err(|error| {
      let kind = error.kind();
      let msg = format!("{}: {}", description, error);
      IoError::new(kind, msg)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn io_result_ok() {
    let ok: IoResult<u8> = Ok(123);
    assert_eq!(ok.describe_err("err").unwrap(), 123);
  }

  #[test]
  fn io_result_err() {
    use std::io::ErrorKind;

    let err: IoResult<u8> = Err(IoError::new(ErrorKind::Other, "abc"));
    assert_eq!(err.describe_err("err").unwrap_err().to_string(), "err: abc");
  }
}
