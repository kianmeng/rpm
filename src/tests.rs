use super::*;
use std::io;
use std::io::prelude::*;

fn test_rpm_file_path() -> std::path::PathBuf {
    let mut rpm_path = cargo_manifest_dir();
    rpm_path.push("test_assets/389-ds-base-devel-1.3.8.4-15.el7.x86_64.rpm");
    rpm_path
}

fn cargo_manifest_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn test_rpm_header() -> Result<(), Box<dyn std::error::Error>> {
    let rpm_file_path = test_rpm_file_path();
    let rpm_file = std::fs::File::open(rpm_file_path).expect("should be able to open rpm file");
    let mut buf_reader = std::io::BufReader::new(rpm_file);

    let package = RPMPackage::parse(&mut buf_reader)?;
    let metadata = &package.metadata;
    assert_eq!(7, metadata.signature.index_entries.len());
    assert!(metadata.signature.index_entries[0].num_items == 16);
    assert_eq!(1156, metadata.signature.index_header.header_size);

    let expected_data = vec![
        (
            16,
            IndexData::Bin(vec![
                0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x07, 0xff, 0xff, 0xff, 0x90, 0x00, 0x00,
                0x00, 0x10,
            ]),
            IndexSignatureTag::HEADER_SIGNATURES,
        ),
        (
            536,
            IndexData::Bin(vec![
                0x89, 0x02, 0x15, 0x03, 0x05, 0x00, 0x5b, 0xe9, 0x8c, 0x5b, 0x24, 0xc6, 0xa8, 0xa7,
                0xf4, 0xa8, 0x0e, 0xb5, 0x01, 0x08, 0xa8, 0x4c, 0x0f, 0xfd, 0x1a, 0x9d, 0xe3, 0x0f,
                0x7e, 0xbb, 0x74, 0xe3, 0x62, 0xef, 0xfd, 0x4d, 0x1c, 0x11, 0xa1, 0x68, 0x22, 0x0d,
                0xff, 0x4a, 0x72, 0x11, 0x18, 0xe4, 0xb0, 0x46, 0x6b, 0x11, 0x82, 0xc6, 0xd4, 0xd6,
                0xdb, 0x53, 0x64, 0x1b, 0x32, 0x33, 0x41, 0x95, 0xf3, 0x0c, 0xa6, 0xc2, 0x50, 0xee,
                0x81, 0x81, 0x6a, 0x08, 0x05, 0xfa, 0x3b, 0x26, 0x66, 0x63, 0x5c, 0xfa, 0x4b, 0x25,
                0x02, 0xe7, 0xad, 0x3f, 0x4f, 0x82, 0x7a, 0xa3, 0x4d, 0xad, 0x0d, 0xa0, 0x19, 0x63,
                0x77, 0xd2, 0x18, 0x30, 0x54, 0xc7, 0x14, 0x23, 0x22, 0x0b, 0x0d, 0xd8, 0xba, 0x1b,
                0x6c, 0x94, 0xb3, 0x0f, 0xb3, 0x82, 0x18, 0x62, 0x33, 0x51, 0x4e, 0xaa, 0xfa, 0x84,
                0x8a, 0x4b, 0xcd, 0x82, 0x72, 0xf1, 0x40, 0x94, 0x38, 0xc7, 0xbc, 0x48, 0x29, 0x4f,
                0x32, 0x98, 0xd9, 0xaf, 0x35, 0x1a, 0x0b, 0xf0, 0x87, 0x74, 0x39, 0xd6, 0xe7, 0x86,
                0x44, 0x9d, 0x5c, 0x7a, 0xde, 0x63, 0x1a, 0x16, 0xb2, 0x29, 0x1d, 0x46, 0x9e, 0x61,
                0xad, 0xff, 0x91, 0x6f, 0x51, 0x65, 0x8a, 0xb9, 0x37, 0x0e, 0x65, 0xb6, 0x77, 0x2f,
                0xb7, 0x74, 0x6a, 0x9c, 0x8a, 0xf0, 0x4b, 0x2d, 0x87, 0xbf, 0x61, 0xff, 0x70, 0xdc,
                0x29, 0xec, 0x9a, 0x0c, 0x7f, 0x12, 0xf6, 0x55, 0xea, 0x22, 0xb5, 0xf0, 0x1a, 0x0d,
                0xa5, 0xe8, 0xc6, 0x7f, 0x1b, 0x9c, 0x55, 0x1b, 0x35, 0x5c, 0xac, 0x72, 0x26, 0x86,
                0x89, 0x30, 0xd5, 0x2d, 0x08, 0x93, 0x0f, 0x9e, 0x1a, 0xfd, 0x8c, 0x7e, 0xdb, 0xca,
                0x57, 0x4f, 0xd9, 0x42, 0xd7, 0xf6, 0x74, 0xcd, 0xf6, 0x68, 0xef, 0xe3, 0x24, 0x66,
                0x92, 0x29, 0xda, 0x96, 0x87, 0x8e, 0xa2, 0x88, 0x23, 0x78, 0xee, 0xc3, 0xfc, 0x71,
                0xfd, 0xb6, 0x36, 0x6b, 0xad, 0xd7, 0x54, 0x55, 0x4d, 0xa0, 0xa3, 0x40, 0x70, 0x51,
                0xc2, 0x76, 0xde, 0x9f, 0xa3, 0xe5, 0x7f, 0x80, 0x72, 0xa9, 0xc3, 0x7f, 0x3e, 0x37,
                0xd7, 0x7a, 0x99, 0x98, 0xc4, 0xc6, 0x4b, 0x51, 0x93, 0xbc, 0xd0, 0xf2, 0x93, 0x09,
                0x73, 0x7f, 0x6e, 0x7a, 0xb4, 0x6b, 0x7b, 0x79, 0xe0, 0x45, 0x55, 0x39, 0xfc, 0x61,
                0xa7, 0xde, 0xa5, 0xff, 0x80, 0x31, 0x39, 0x14, 0xf6, 0xb6, 0x07, 0x6c, 0xd7, 0xa4,
                0x10, 0xa0, 0x87, 0x55, 0x4d, 0xe5, 0xa5, 0x26, 0xc1, 0x99, 0x0e, 0x58, 0x19, 0xae,
                0xc3, 0xbf, 0xe8, 0x16, 0x48, 0xe0, 0x85, 0x96, 0x51, 0x18, 0x72, 0xb8, 0x0f, 0x00,
                0x9f, 0x26, 0xde, 0xec, 0x12, 0x32, 0xec, 0xd0, 0x3c, 0xde, 0x31, 0x0b, 0xd6, 0xbf,
                0x4a, 0xc5, 0x66, 0x5c, 0xcd, 0xb0, 0x29, 0x3c, 0x6d, 0xc6, 0x18, 0x56, 0xd7, 0x17,
                0xb4, 0x4d, 0xeb, 0xdc, 0xbb, 0xe4, 0x4f, 0x1a, 0xf5, 0x72, 0x3a, 0x96, 0x44, 0x4d,
                0xf3, 0x14, 0xb1, 0x79, 0x75, 0xa4, 0x6a, 0xcc, 0x9d, 0x27, 0x47, 0xa9, 0x12, 0xa7,
                0x07, 0xa8, 0x30, 0xae, 0xf2, 0xde, 0xbc, 0x33, 0x87, 0xb5, 0x8c, 0x05, 0x3f, 0x45,
                0x4e, 0x64, 0x4a, 0x86, 0x6d, 0xc3, 0xf4, 0xfe, 0x05, 0x91, 0x81, 0x95, 0x2f, 0xad,
                0x81, 0xda, 0x1b, 0x39, 0xf8, 0xf0, 0xb8, 0x46, 0xf0, 0x38, 0x82, 0xa6, 0xf2, 0x35,
                0x34, 0x4d, 0x9e, 0x17, 0x9a, 0x97, 0xaf, 0xbd, 0x9b, 0x19, 0x31, 0x88, 0xd8, 0x3a,
                0x50, 0x2e, 0x91, 0x50, 0x45, 0x05, 0x92, 0x88, 0xb2, 0x07, 0x10, 0x9a, 0x6c, 0x44,
                0xa2, 0x72, 0x0f, 0xca, 0x68, 0x17, 0x99, 0x1a, 0x62, 0xcd, 0x66, 0x23, 0x0f, 0x90,
                0xa4, 0x14, 0xa6, 0x6c, 0x7d, 0x06, 0xc4, 0x4b, 0xbe, 0x81, 0x47, 0x72, 0xeb, 0xd4,
                0xa2, 0x3d, 0x63, 0x73, 0x86, 0xef, 0x0e, 0x2b, 0x78, 0xd4, 0x4f, 0x48, 0x2e, 0xb0,
                0x55, 0x8c, 0x8e, 0x5d,
            ]),
            IndexSignatureTag::RPMSIGTAG_RSA,
        ),
        (
            1,
            IndexData::StringTag("6178620331c1fe63c5dd3da7c118058e366e37d8".to_string()),
            IndexSignatureTag::RPMSIGTAG_SHA1,
        ),
        (
            1,
            IndexData::Int32(vec![275_904]),
            IndexSignatureTag::RPMSIGTAG_SIZE,
        ),
        (
            536,
            IndexData::Bin(vec![
                0x89, 0x02, 0x15, 0x03, 0x05, 0x00, 0x5b, 0xe9, 0x8c, 0x5b, 0x24, 0xc6, 0xa8, 0xa7,
                0xf4, 0xa8, 0x0e, 0xb5, 0x01, 0x08, 0x54, 0xe7, 0x10, 0x00, 0xc4, 0xbb, 0xc5, 0x5b,
                0xe7, 0xe3, 0x80, 0xbd, 0xe9, 0x0a, 0xc6, 0x32, 0x6a, 0x42, 0x4a, 0xb0, 0xa9, 0xf5,
                0x95, 0xf1, 0xa9, 0x31, 0x4a, 0x22, 0xfc, 0xf8, 0xdc, 0xcf, 0x89, 0xd8, 0x30, 0x19,
                0x83, 0x55, 0xf0, 0xb5, 0xa1, 0x0c, 0xd3, 0x6b, 0x69, 0x21, 0x8f, 0x05, 0xe5, 0x17,
                0x5c, 0x29, 0x99, 0x84, 0x84, 0xc6, 0xf2, 0xa7, 0xcf, 0xe9, 0xd4, 0x99, 0x42, 0x20,
                0x39, 0xf5, 0xd9, 0x96, 0x6a, 0xc3, 0x01, 0x13, 0xfa, 0x46, 0xee, 0x6d, 0xcb, 0x01,
                0xf7, 0xc9, 0x34, 0x26, 0x8e, 0x9e, 0xba, 0x5d, 0x89, 0xb9, 0xd9, 0x21, 0x15, 0x06,
                0x51, 0xa6, 0xad, 0x70, 0xc5, 0x3a, 0xd8, 0xa8, 0x84, 0x94, 0xbe, 0x29, 0xc1, 0x9b,
                0x53, 0x38, 0x26, 0x90, 0x8b, 0x7d, 0xd2, 0xa0, 0x7c, 0xcc, 0xa2, 0x77, 0x60, 0xfa,
                0xb9, 0x7f, 0x90, 0x77, 0xc7, 0xb9, 0xad, 0x7e, 0xab, 0xa0, 0xdb, 0xa3, 0x29, 0xec,
                0x72, 0xa0, 0x70, 0xd1, 0xed, 0x9a, 0x8c, 0x30, 0x6b, 0xdf, 0xc5, 0x8b, 0x0f, 0xc8,
                0x14, 0xca, 0xe1, 0x2b, 0x95, 0x14, 0x6a, 0x70, 0x21, 0x23, 0x49, 0x14, 0x70, 0xe6,
                0x84, 0xe1, 0xf1, 0xd0, 0x6f, 0xc0, 0x7d, 0xcd, 0xb7, 0xdf, 0xd4, 0xc6, 0xd3, 0xd0,
                0x17, 0x5d, 0xb3, 0xf4, 0xaf, 0xd3, 0xea, 0xaa, 0xed, 0x2f, 0x72, 0x02, 0xfb, 0xd4,
                0x46, 0x75, 0x2a, 0xc3, 0x38, 0x50, 0xd7, 0xb2, 0x5b, 0x61, 0x64, 0x25, 0x07, 0x8c,
                0x9b, 0x01, 0xf8, 0x6f, 0xeb, 0xbb, 0x5d, 0xb0, 0x02, 0x81, 0x30, 0xeb, 0x4b, 0x01,
                0xe1, 0xff, 0x9f, 0x24, 0xa7, 0xe3, 0xde, 0x71, 0x51, 0x96, 0x92, 0xd0, 0x60, 0x18,
                0xc3, 0x60, 0xd5, 0xae, 0xd7, 0x40, 0x26, 0x57, 0xf3, 0xdb, 0x6a, 0x81, 0x97, 0x64,
                0x10, 0x24, 0x05, 0x7d, 0x54, 0x95, 0x8d, 0x36, 0x5f, 0x23, 0xd7, 0x17, 0x1a, 0x83,
                0xca, 0xf0, 0xe6, 0x1d, 0x27, 0x22, 0xdc, 0xb6, 0x04, 0x0d, 0xe8, 0x25, 0xe6, 0xc4,
                0xe0, 0x26, 0x17, 0x42, 0x03, 0x36, 0xfe, 0xf8, 0xc7, 0xc2, 0xdb, 0xa2, 0xb7, 0x99,
                0x3a, 0xec, 0xe2, 0xd4, 0x93, 0x3d, 0x53, 0x0d, 0x26, 0x96, 0x84, 0x6e, 0x4b, 0xfa,
                0xb3, 0xca, 0x98, 0x8a, 0x65, 0xa8, 0x62, 0x7d, 0xbf, 0x1f, 0x80, 0xbf, 0xa3, 0xa6,
                0xe7, 0x03, 0x0e, 0x15, 0xb7, 0x73, 0x37, 0xdb, 0x35, 0x35, 0x6f, 0xce, 0x71, 0xd0,
                0x3c, 0x15, 0x76, 0x6d, 0x26, 0xe5, 0xf6, 0xae, 0x50, 0xc8, 0x28, 0xa5, 0xb3, 0xdf,
                0xd3, 0x24, 0xb9, 0x3f, 0xfd, 0xcc, 0x02, 0x60, 0xe4, 0xfd, 0x10, 0x71, 0x0a, 0xbe,
                0xdf, 0x19, 0x23, 0xa1, 0x71, 0xe6, 0x99, 0x3c, 0xef, 0xd5, 0x41, 0x20, 0x7a, 0x9a,
                0x8c, 0x24, 0xe8, 0x74, 0x83, 0xdd, 0xab, 0xea, 0x87, 0x38, 0xca, 0x8e, 0x3d, 0x60,
                0x14, 0x20, 0xc7, 0x02, 0xed, 0xa1, 0xdc, 0xd5, 0xcf, 0x22, 0x14, 0x14, 0x93, 0x9c,
                0x68, 0x95, 0xbf, 0x6e, 0xdd, 0x28, 0x3e, 0xfc, 0xa0, 0xfb, 0x37, 0xdf, 0x9c, 0x7c,
                0xef, 0x37, 0x11, 0x7a, 0xa3, 0x28, 0x71, 0xd5, 0xca, 0xa3, 0x17, 0x09, 0xa9, 0x92,
                0xc9, 0x1a, 0x2b, 0x5d, 0xac, 0x0e, 0xee, 0x10, 0xc4, 0x97, 0xad, 0x18, 0x4e, 0x1a,
                0xb7, 0x2a, 0xd2, 0x1c, 0xb6, 0x9d, 0x8b, 0x22, 0x91, 0x61, 0x9f, 0x6e, 0xe0, 0x06,
                0x9c, 0xc2, 0x21, 0x8f, 0x24, 0x95, 0x80, 0x19, 0x17, 0x15, 0x5c, 0xba, 0x27, 0x9f,
                0xa4, 0xc8, 0x19, 0xd1, 0xfb, 0x64, 0xf7, 0x36, 0x5e, 0x6b, 0x36, 0xba, 0x25, 0x27,
                0x3d, 0x31, 0x74, 0x9e, 0x53, 0xf7, 0x23, 0xe2, 0x00, 0x0c, 0x86, 0x9c, 0xab, 0x3f,
                0xf5, 0x44, 0x6e, 0xaa, 0xd8, 0x03, 0x8b, 0x2e, 0x8c, 0xca, 0x14, 0xfe, 0x1d, 0xad,
                0x6b, 0x5e, 0x60, 0x8d,
            ]),
            IndexSignatureTag::RPMSIGTAG_PGP,
        ),
        (
            16,
            IndexData::Bin(vec![
                0xdb, 0x6d, 0xf4, 0x9b, 0x40, 0x19, 0x6e, 0x84, 0x5e, 0xed, 0x42, 0xe2, 0x16, 0x62,
                0x28, 0x67,
            ]),
            IndexSignatureTag::RPMSIGTAG_MD5,
        ),
        (
            1,
            IndexData::Int32(vec![510_164]),
            IndexSignatureTag::RPMSIGTAG_PAYLOADSIZE,
        ),
    ];

    for (i, (len, data, tag)) in expected_data.iter().enumerate() {
        assert_eq!(*len as u32, metadata.signature.index_entries[i].num_items);
        assert_eq!(data, &metadata.signature.index_entries[i].data);
        assert_eq!(*tag, metadata.signature.index_entries[i].tag);
    }

    assert_eq!("cpio", metadata.header.get_payload_format()?);
    assert_eq!("xz", metadata.header.get_payload_compressor()?);

    let expected_file_checksums = vec![
        "",
        "3e4e2501e2a70343a661b0b85b82e27b2090a7e595dc3b5c91e732244ffc3272",
        "d36ab638ed0635afcb1582387d676b2e461c5a88ac05a6e2aada8b40b4175bc1",
        "9667aa81021c9f4d48690ef6fbb3e7d623bdae94e2da414abd044dc38e52f037",
        "1e8235e08aac746155c209c1e641e73bf7a4c34d9971aaa9f864226bd5de9d99",
        "53a1e216749208c0bdfc9e8ec70f4bb9459ad1ff224571a7a432e472d2202986",
        "2807bb4e77579c81dc7e283d60612a6ecc3ce56000691cac744a4bca73cea241",
        "",
        "",
        "",
        "",
        "",
        "a839e2870b7a212ca9dc6f92007907bc42de1984eac6c278a519d4115071f322",
        "3ca364e71a110cd0f2317fbaf99bc8552b8374dbeaf0a989695990f940d88bea",
        "eead9f55f0774559d37b20fbc5448f978e1a80d27f488768cbbb278a932e7e9f",
        "",
        "495b7c1e22dcc0f37d78076a1fcad786b69ac78f1e806466d798fd8fc4a5d10d",
        "8ceb4b9ee5adedde47b31e975c1d90c73ad27b6b165a1dcd80c7c545eb65b903",
        "a73b7d3598e98f46aeb0559e641d3e6ac83c0fc34e1e5fa98cb9d4a6050bacd9",
        "97a6a0413ce3664e192dff12a29bc3f690c24e8a0d48d986478c56cdfe370c3b",
        "d110052464fd35c5dc227b3f071606ec40c12ba773fec9ec88ad01430bd4a27b",
        "5c3adbdea58a8bb7663c65216dda7d1f38a17b067f718df46ece04ecb503f689",
        "005dc9d5aa85b10c3200535af8b0ed2123770e3a79d48be5067e81cc553d55bd",
        "aa7ea2def38dfc965b27ae20467006aca779e02ad366d50824c4615a7d43af27",
        "5ee25b47a83b1431f6ecb1d0a292a8e9a2917c1de9e87129c86cdda743be3f55",
        "413aae4fb264aad9d35db94eb28b5f70a7183101692943e81bc90d6718418d8e",
        "66004b2e338ce29e59d6a26467e251f092ae0a0f33b67dbba67d2ea9f3ec89f6",
        "3db4ad3317bff658a04a1bdbc01fab83cd348f76a1d44585b892fdb0223f2b77",
        "ccac76a229e6739ab318d9ede59f6b980d3200fc50669409d3b1e8a0ff1fa029",
        "5a3378c84c68e2a407add0f850c64d701af2aedcca67dd2489e86cb1e08dbb6b",
        "da188ece6801b97c98031b854d4000e348e969edea239cb1bcbfae7a194e3520",
        "28a93db2fe665e8b08494fe5adf3d8dc00c2f96a4994a09eb70cf982d912fa09",
        "ba92ea5c90389b38a3c003a5e4a7b09e57473cbd2fb3645c2c0012808023fd0b",
        "502dd15afe5609a113108cad047a810b7a97cc8819e830f1d5b00cb5bf65a295",
        "4445b3e6550a3d7da96a246e6138d3f349160420085ce14222d3f686eb29915c",
        "649f748bffe197539db9237d56da8a3e408731488550617596359cd32731ec06",
        "4bd801d053bf456c3dd2c94f9721d1bb0c44d2c119e233b8ad4c5189bd39b256",
        "d444bb47f4a83ebd0e6b669f73bb2d6d3dde804b70a0bbd2be66693d88ce8e16",
        "087be3693057db21a0b1d38844bb5efa8112f67f3572063546215f25f9fe8d9e",
        "2c639c8768e323f2ad4ea96f1667989cb97d49947e9bcebcd449163d9c9bb85c",
    ];

    let checksums = metadata.header.get_file_checksums()?;

    assert_eq!(expected_file_checksums, checksums);

    let mut buf = Vec::new();

    package.metadata.lead.write(&mut buf)?;
    assert_eq!(96, buf.len());

    let lead = Lead::parse(&buf)?;
    assert!(package.metadata.lead == lead);

    buf_reader.seek(io::SeekFrom::Start(0))?;
    let mut expected = vec![0; 96];
    // buf_reader.read_to_end(&mut expected);
    buf_reader.read_exact(&mut expected)?;

    for i in 0..expected.len() {
        assert_eq!(expected[i], buf[i]);
    }

    buf = Vec::new();
    package.metadata.signature.write_signature(&mut buf)?;
    let signature = Header::parse_signature(&mut buf.as_ref())?;

    assert_eq!(
        package.metadata.signature.index_header,
        signature.index_header
    );

    for i in 0..signature.index_entries.len() {
        assert_eq!(
            signature.index_entries[i],
            package.metadata.signature.index_entries[i]
        );
    }
    assert_eq!(
        package.metadata.signature.index_entries,
        signature.index_entries
    );

    buf = Vec::new();
    package.metadata.header.write(&mut buf)?;
    let header = Header::parse(&mut buf.as_ref())?;
    assert_eq!(package.metadata.header, header);

    buf = Vec::new();
    package.write(&mut buf)?;
    let second_pkg = RPMPackage::parse(&mut buf.as_ref())?;
    assert_eq!(package.content.len(), second_pkg.content.len());
    assert!(package.metadata == second_pkg.metadata);

    Ok(())
}

#[test]
fn test_region_tag() -> Result<(), Box<dyn std::error::Error>> {
    let region_entry = Header::create_region_tag(IndexSignatureTag::HEADER_SIGNATURES, 2, 400);

    let data = match region_entry.data {
        IndexData::Bin(d) => d,
        _ => return Err(Box::new(RPMError::new("should be bin"))),
    };

    let (_, entry) = IndexEntry::<IndexSignatureTag>::parse(&data)?;

    assert_eq!(entry.tag, IndexSignatureTag::HEADER_SIGNATURES);
    assert_eq!(entry.data.to_u32(), IndexData::Bin(Vec::new()).to_u32());
    assert_eq!(-48, entry.offset);

    Ok(())
}
