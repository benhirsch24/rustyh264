extern crate h264nalparse;

fn main() {
    let mut parser = match h264nalparse::parser::H264NalParser::new("Message.h264") {
        Ok(p) => p,
        Err(e) => panic!("Couldn't create h264 nal parser :( {}", e)
    };
    let mut offset = 0;
    loop {
        let next_unit = match parser.parse_nalunit(offset) {
            Err(e) => { println!("Stopped parsing, {:?}", e); break; }
            Ok(u) => u
        };

        println!("Parsed nal unit: {:?} type: {:?} offset: {}", next_unit, next_unit.nal_unit_type, offset + next_unit.size);
        match next_unit.nal_unit_type {
            h264nalparse::H264NalUnitType::SPS => {
                match parser.parse_sps(next_unit.data_offset) {
                    Ok(sps) => println!("Parsed SPS: {}", sps),
                    Err(e) => println!("Error in parsing SPS: {:?}", e)
                };
            },
            h264nalparse::H264NalUnitType::PPS => {
                match parser.parse_pps(next_unit.data_offset) {
                    Ok(pps) => println!("Parsed PPS: {}", pps),
                    Err(e) => println!("Error in parsing PPS: {:?}", e)
                };
            },
            h264nalparse::H264NalUnitType::IDR => {
                match parser.parse_slice(next_unit.data_offset, &next_unit) {
                    Ok(slice) => println!("Parsed slice: {}", slice),
                    Err(e) => println!("Error in parsing slice: {:?}", e)
                };
            },
            _ =>   { println!("Parsed type {}", next_unit.nal_unit_type_num); }
        }
        offset += next_unit.size;
    }
}
