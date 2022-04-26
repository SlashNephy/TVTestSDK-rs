pub const DEFAULT_API_VERSION: Version = Version {
    major: 0,
    minor: 0,
    build: 14
};

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct Version {
    /// メジャーバージョン
    pub major: u32,
    /// マイナーバージョン
    pub minor: u32,
    /// ビルドナンバー
    pub build: u32
}

impl Version {
    // 上位8ビットがメジャーバージョン
    #[inline]
    pub fn get_major(version: u32) -> u32 {
        version >> 24
    }

    // 次の12ビットがマイナーバージョン
    #[inline]
    pub fn get_minor(version: u32) -> u32 {
        (version & 0x00FFF000) >> 12
    }

    // 下位12ビットがビルドナンバー
    #[inline]
    pub fn get_build(version: u32) -> u32 {
        version & 0x00000FFF
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
        (self.major << 24)  | (self.minor << 12) | self.build
    }
}
