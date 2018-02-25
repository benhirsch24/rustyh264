extern crate h264nalparse;

fn main() {
    let mut parser = match h264nalparse::H264NalParser::new("Message.h264") {
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
                let sps = parser.parse_sps(next_unit.data_offset);
                println!("Parsed SPS: {:?}", sps);
            },
            h264nalparse::H264NalUnitType::PPS => { println!("Parsed PPS"); },
            h264nalparse::H264NalUnitType::IDR => {
                let idr = parser.parse_idr(next_unit.data_offset);
                println!("Parsed IDR {:?}", idr);
            },
            _ =>   { println!("Parsed type {}", next_unit.nal_unit_type_num); }
        }
        offset += next_unit.size;
    }
}
