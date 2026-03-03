pub(crate) mod translation;
pub(crate) use translation::Translation;

mod open_file;
pub(crate) use open_file::open_file;

pub(crate) trait ToSliceArr<T> {
    fn to_slice(&self) -> Vec<&[T]>;
}
impl<T> ToSliceArr<T> for Vec<Vec<T>> {
    fn to_slice(&self) -> Vec<&[T]> {
        self.iter().map(Vec::as_slice).collect()
    }
}
