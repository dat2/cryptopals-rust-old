fn hex_chars_to_hex_number(input: Vec<u8>) -> Vec<u8> {
  // ranges for characters
  const NUMERIC_BEGIN: u8 = '0' as u8;
  const NUMERIC_END: u8 = '9' as u8;
  const LOWER_LETTER_BEGIN: u8 = 'a' as u8;
  const LOWER_LETTER_END: u8 = 'f' as u8;
  const UPPER_LETTER_BEGIN: u8 = 'A' as u8;
  const UPPER_LETTER_END: u8 = 'F' as u8;

  let mut output = Vec::new();
  for byte in input {
    // convert the character data to the numeric representation
    let true_number = match byte {
      NUMERIC_BEGIN ... NUMERIC_END => byte - NUMERIC_BEGIN,
      LOWER_LETTER_BEGIN ... LOWER_LETTER_END => byte - LOWER_LETTER_BEGIN + 10,
      UPPER_LETTER_BEGIN ... UPPER_LETTER_END => byte - UPPER_LETTER_BEGIN + 10,
      _ => 0
    };
    output.push(true_number);
  }
  output
}

fn bit_value_to_base64(bit: u8) -> u8 {
  match bit {
    0 ... 25 => ('A' as u8) + bit,
    26 ... 51 => ('a' as u8) + (bit - 26),
    52 ... 61 => ('0' as u8) + (bit - 52),
    62 => '+' as u8,
    63 => '/' as u8,
    _ => '=' as u8
  }
}

fn hex_to_base64(input: Vec<u8>) -> Vec<u8> {
  const NUM_NYBBLES_TO_SHIFT: u8 = 5;

  let mut shift_index = 0;
  let mut temp_word: u32 = 0;

  let mut copy = input.clone();

  // fill in 0s at the end to ensure that we have a divisible by 3 value :)
  let mut n_push = 6 - copy.len() % 6;
  while n_push != 0 {
    copy.push(64); // the 64 is to trigger the bit_value_to_base64 function to return '='
    n_push = n_push - 1;
  }

  let mut triples = Vec::new();
  for byte in input {
    let true_number = byte as u32;

    // pack each hex number to a temporary u32 to keep track of 3 hex digits
    // eg. a: u8, b: u8, c: u8 get packed into a u32 as <0><a><b><c>
    let shift = NUM_NYBBLES_TO_SHIFT - shift_index;
    let shifted = true_number << (shift * 4);
    temp_word = temp_word | shifted;

    // if we have hit the third byte, then push the result into temp_word.
    if shift_index == 5 {
      triples.push(temp_word);
      temp_word = 0;
    }
    shift_index = (shift_index + 1) % 6;
  }

  // now we need to convert triples to an actual result
  let mut result = Vec::new();

  // for a value packed like <0><a><b><c>, where a,b,c are 8 bits
  // we need to convert 24 bits to 4 groups of 6 bits
  // eg. <a><b><c> == <w><x><y><z> where a,b,c are 8 bits, and w,x,y,z are 6 bits
  for quad in triples {
    let first = ((quad >> 18) & 0x003F) as u8;
    let second = ((quad >> 12) & 0x003F) as u8;
    let third = ((quad >> 6) & 0x003F) as u8;
    let fourth = (quad & 0x003F) as u8;

    result.push(bit_value_to_base64(first));
    result.push(bit_value_to_base64(second));
    result.push(bit_value_to_base64(third));
    result.push(bit_value_to_base64(fourth));
  }

  // result
  result
}

#[test]
fn test_challenge_1() {
  let input = hex_chars_to_hex_number(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_vec());
  let output = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  assert!(hex_to_base64(input.to_vec()) == output.to_vec());
}

fn fixed_xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
  a.iter().zip(b.iter()).map(|(aa,bb)| aa ^ bb).collect()
}

#[test]
fn test_challenge_2() {
  let a = hex_chars_to_hex_number(b"1c0111001f010100061a024b53535009181c".to_vec());
  let b = hex_chars_to_hex_number(b"686974207468652062756c6c277320657965".to_vec());

  let result = hex_chars_to_hex_number(b"746865206b696420646f6e277420706c6179".to_vec());
  assert!(fixed_xor(a,b) == result);
}
