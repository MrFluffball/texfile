use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use byteorder::{LittleEndian, ReadBytesExt};
use bcndecode::{BcnDecoderFormat, BcnEncoding, decode};
use png_encode_mini::write_rgba_from_u8;


struct Mipmap {
  width: u16,
  height: u16,
  pitch: u16,
  data_size: u32,
  data: Option<Vec<u8>>
}


fn get_tex_data(path: &str) -> Result<Vec<u8>, io::Error> {
  let file = File::open(path)?;
  let mut reader = BufReader::new(file);
  let mut buffer = Vec::new();
  
  reader.read_to_end(&mut buffer)?;

  Ok(buffer)
}


fn get_main_mipmap(path: &str) -> Result<Mipmap, io::Error> {
  let mut reader = Cursor::new(get_tex_data(path)?);

  // todo: might want to check if the "ktex" header exists instead of skipping over it?
  reader.set_position(4);

  let header = reader.read_u32::<LittleEndian>().unwrap();
  let num_mips = (header >> 13) & 31;


  let mut mipmap = Mipmap { 
    width: reader.read_u16::<LittleEndian>().unwrap(), 
    height: reader.read_u16::<LittleEndian>().unwrap(), 
    pitch: reader.read_u16::<LittleEndian>().unwrap(), 
    data_size: reader.read_u32::<LittleEndian>().unwrap(),
    // put this in stasis for now
    data: None 
  };

  // there's other mipmap data besides the one we just read, skip over it so we can get to the raw data.
  let offset: i64 = ((num_mips-1) * 10).into();
  reader.seek(SeekFrom::Current(offset))?;

  // fill in the mipmap data
  let mut buf: Vec<u8> = vec![];
  let mut chunk = reader.take(mipmap.data_size.into());

  chunk.read_to_end(&mut buf)?;
  mipmap.data = Some(buf);

  Ok(mipmap)
}


fn decompress_mipmap(mipmap: &Mipmap) -> Result<Vec<u8>, bcndecode::Error> {
  let decoded = decode(
    mipmap.data.as_ref().unwrap(), 
    mipmap.width.into(), 
    mipmap.height.into(), 
    BcnEncoding::Bc3, // DXT5
    BcnDecoderFormat::RGBA
  )?;

  Ok(decoded)
}


pub fn tex_to_png(path: &str, name: &str) {
  let mipmap = get_main_mipmap(path).unwrap(); 
  let buffer = decompress_mipmap(&mipmap).unwrap();

  let mut file = File::create(name).expect("failed to create file");

  write_rgba_from_u8(
    &mut file, 
    &buffer, 
    mipmap.width.into(), 
    mipmap.height.into()
  ).expect("failed to convert buffer to png");
}