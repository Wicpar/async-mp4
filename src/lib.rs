pub(crate) mod id;
pub(crate) mod error;
pub(crate) mod header;
pub(crate) mod mp4box;
pub(crate) mod matrix;
pub(crate) mod r#type;
pub(crate) mod size;
pub(crate) mod bytes_write;
pub(crate) mod bytes_read;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
