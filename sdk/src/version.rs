pub const DEFAULT_API_VERSION: Version = Version {
    major: 0,
    minor: 0,
    build: 14
};

#[cfg_attr(test, derive(Debug))]
pub struct Version {
    pub major: u8,
    pub minor: u16,
    pub build: u16
}

impl Version {
    // 上位8ビットがメジャーバージョン
    #[inline]
    pub fn get_major(version: u32) -> u8 {
        (version >> 24) as u8
    }

    // 次の12ビットがマイナーバージョン
    #[inline]
    pub fn get_minor(version: u32) -> u16 {
        ((version & 0x00FFF000) >> 12) as u16
    }

    // 下位12ビットがビルドナンバー
    #[inline]
    pub fn get_build(version: u32) -> u16 {
        (version & 0x00000FFF) as u16
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        let major = Version::get_major(value);
        let minor = Version::get_minor(value);
        let build = Version::get_build(value);

        Version {
            major, minor, build
        }
    }
}

impl Into<u32> for Version {
    fn into(self) -> u32 {
        ((self.major as u32) << 24)  | ((self.minor as u32) << 12) | self.build as u32
    }
}
