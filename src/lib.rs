use std::io::{Error, Read};

#[derive(Debug)]
pub struct A(pub usize);

fn deserialize(data: &[u8]) -> Result<A, Error> {
    Ok(A(data.len()))
}

pub fn read_many0<'a, R: Read + 'a>(mut reader: R) -> impl Iterator<Item = Result<A, Error>> + 'a {
    (0..10).map(move |_| {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes)?;
        let length = u32::from_le_bytes(bytes) as usize;

        let mut data = vec![0; length];
        reader.read_exact(&mut data)?;
        deserialize(&data)
    })
}

/// A small wrapper around [`Vec<u8>`] that allows us to reuse memory once it is initialized.
/// This may improve performance of the [`Read`] trait.
#[derive(Clone, Default)]
pub struct ReadBuffer {
    data: Vec<u8>,
    // length to be read or is read
    length: usize,
}

impl ReadBuffer {
    /// Set the minimal length of the [`ReadBuf`]. Contrary to the
    /// method on `Vec` this is `safe` because this function guarantees that
    /// the underlying data always is initialized.
    #[inline]
    pub fn set_len(&mut self, length: usize) {
        if length > self.data.capacity() {
            // exponential growing strategy
            self.data = vec![0; 2 * length];
        } else if length > self.data.len() {
            self.data.resize(length, 0);
        }
        self.length = length;
    }
}

impl AsRef<[u8]> for ReadBuffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.data[..self.length]
    }
}

impl AsMut<[u8]> for ReadBuffer {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data[..self.length]
    }
}

pub fn read_many1<'a, R: Read + 'a>(mut reader: R) -> impl Iterator<Item = Result<A, Error>> + 'a {
    let mut data: ReadBuffer = Default::default();
    (0..10).map(move |_| {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes)?;
        let length = u32::from_le_bytes(bytes) as usize;

        data.set_len(length);
        reader.read_exact(data.as_mut())?;
        deserialize(data.as_ref())
    })
}

pub fn read_many2<R: Read>(mut reader: R) -> impl Iterator<Item = Result<A, Error>> {
    let mut data = Vec::new(); // vec![0; reasonable_expectation];
    (0..10).map(move |_| {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes)?;
        let length = u32::from_le_bytes(bytes) as usize;

        // Never shorten the known-initialized length
        // Maybe worth it to spell this out in an `if` to possibly
        // avoid a call to `resize`
        let max = data.len().max(length);
        data.resize(max, 0);

        // Check that these bound checks are eliminated or throw in an
        // assert! maybe
        let this = &mut data[..length];
        reader.read_exact(this)?;
        deserialize(this)
    })
}

pub fn read_many3<R: Read>(mut reader: R) -> impl Iterator<Item = Result<A, Error>> {
    let mut vec = Vec::new();
    (0..10).map(move |_| {
        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes)?;
        let length = u32::from_le_bytes(bytes) as usize;

        vec.clear();
        vec.try_reserve(length).unwrap();
        reader.by_ref().take(length as u64).read_to_end(&mut vec)?;
        assert_eq!(vec.len(), length);

        deserialize(&vec)
    })
}
