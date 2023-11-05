//

#[derive(Debug)]
pub enum AtomKind {
    Ftyp,
    Free,
    Moov,
    Mdat,
    Mvhd,
    Iods,
    Trak,
    Tkhd,
    Edts,
    Elst,
    Mdia,
    Mdhd,
    Hdlr,
    Minf,
    Vmhd,
    Dinf,
    Dref,
    Url,
    Stbl,
    Stsd,
    Stts,
    Stss,
    Sdtp,
    Stsc,
    Stsz,
    Stco,
    Sgpd,
    Sbgp,
    Avc1,
    AvcC,
    Pasp,
}

fn atom_kind_from_buf(data: [u8; 4]) -> AtomKind {
    let type_from_bytes: String = data.iter().map(|&c| c as char).collect();

    match type_from_bytes.as_str() {
        "ftyp" => AtomKind::Ftyp,
        "free" => AtomKind::Free,
        "moov" => AtomKind::Moov,
        "mdat" => AtomKind::Mdat,
        "mvhd" => AtomKind::Mvhd,
        "iods" => AtomKind::Iods,
        "trak" => AtomKind::Trak,
        "tkhd" => AtomKind::Tkhd,
        "edts" => AtomKind::Edts,
        "elst" => AtomKind::Elst,
        "mdia" => AtomKind::Mdia,
        "mdhd" => AtomKind::Mdhd,
        "hdlr" => AtomKind::Hdlr,
        "minf" => AtomKind::Minf,
        "vmhd" => AtomKind::Vmhd,
        "dinf" => AtomKind::Dinf,
        "dref" => AtomKind::Dref,
        "url " => AtomKind::Url,
        "stbl" => AtomKind::Stbl,
        "stsd" => AtomKind::Stsd,
        "stts" => AtomKind::Stts,
        "stss" => AtomKind::Stss,
        "sdtp" => AtomKind::Sdtp,
        "stsc" => AtomKind::Stsc,
        "stsz" => AtomKind::Stsz,
        "stco" => AtomKind::Stco,
        "sgpd" => AtomKind::Sgpd,
        "sbgp" => AtomKind::Sbgp,
        "avc1" => AtomKind::Avc1,
        "avcC" => AtomKind::AvcC,
        "pasp" => AtomKind::Pasp,
        unknown => {
            panic!("Unknown AtomType {}", unknown)
        }
    }
}

#[derive(Debug)]
pub struct AtomHeader {
    pub size: u32,
    pub kind: AtomKind,
}

impl AtomHeader {
    pub fn new(buf: &[u8]) -> AtomHeader {
        AtomHeader {
            size: u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
            kind: atom_kind_from_buf([buf[4], buf[5], buf[6], buf[7]]),
        }
    }
}

#[derive(Debug)]
pub struct AtomFtyp {
    pub header: AtomHeader,
    pub codec: [u8; 4],
    pub minor_version: u32,
    pub compat: [[u8; 4]; 4],
}
