# Async mp4

An mp4 muxer demuxer for mp4, made to work in WASM and async contexts.

## State  
- [x] Standard for Fragmented mp4
- [ ] Easy to use api
- [ ] Performant async reader (too many async calls currently)

## Why not [mp4-rust](https://github.com/alfg/mp4-rust) ?

- Their code does not implement all the necessary features for fragmented mp4 streams
- Their reader is not async
- We set flags automatically according to data
- We use a very clean macro for box definition for easy implementation: 
```rust
full_box! {
    box (b"mvhd", Mvhd, MvhdBox, u32)
    data {
        creation_time: Mp4DateTime,
        modification_time: Mp4DateTime,
        timescale: u32,
        duration: Mp4Duration,
        rate: I16F16,
        volume: I8F8,
        _r1: u16, // reserved
        _r2: [u32; 2], // reserved
        matrix: MP4Matrix,
        _r3: [u32; 6], // reserved
        next_track_id: u32
    }
}
```

## Usage
```rust
let ftyp = FtypBox {
    major_brand: *b"iso5",
    minor_version: 1,
    compatible_brands: vec![*b"isom", *b"avc1", *b"iso2", *b"iso5"],
};
let moov: MoovBox = Moov {
    mvhd: Some(Default::default()),
    mvex: Some(Mvex {
        trex: traks.iter().map(|trak| {
            Trex {
                track_id: trak.tkhd.as_ref().map(|it|it.track_id).unwrap_or(0),
                default_sample_description_index: 1,
                default_sample_duration: 0,
                default_sample_size: 0,
                default_sample_flags: Default::default(),
            }.into()
        }).collect(),
    }.into()),
    traks: vec![Trak { /* trak info... */ }.into()],
}.into();

let mut buf = std::io::Cursor::new(vec![]);
ftyp.write(&mut buf)?;
moov.write(&mut buf)?;
return buf.into_inner();
```

## Todo

- Make async reader read full box chunks once the size is known and decode using a synchronous reader (so we don't allocate for every byte)
- Make the box macro generate a View structure of the box so that existing data can be read without decoding everything and preserving the original structure
- Make an easy-to-use builder to generate files from a stream of codec wrapping boxes and headers
