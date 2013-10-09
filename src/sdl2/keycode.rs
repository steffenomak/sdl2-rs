#[deriving(Eq)]
pub enum KeyCode {
    UnknownKey            = 0_i,
    BackspaceKey          = 8_i,
    TabKey                = 9_i,
    ReturnKey             = 13_i,
    EscapeKey             = 27_i,
    SpaceKey              = 32_i,
    ExclaimKey            = 33_i,
    QuotedblKey           = 34_i,
    HashKey               = 35_i,
    DollarKey             = 36_i,
    PercentKey            = 37_i,
    AmpersandKey          = 38_i,
    QuoteKey              = 39_i,
    LeftParenKey          = 40_i,
    RightParenKey         = 41_i,
    AsteriskKey           = 42_i,
    PlusKey               = 43_i,
    CommaKey              = 44_i,
    MinusKey              = 45_i,
    PeriodKey             = 46_i,
    SlashKey              = 47_i,
    Num0Key               = 48_i,
    Num1Key               = 49_i,
    Num2Key               = 50_i,
    Num3Key               = 51_i,
    Num4Key               = 52_i,
    Num5Key               = 53_i,
    Num6Key               = 54_i,
    Num7Key               = 55_i,
    Num8Key               = 56_i,
    Num9Key               = 57_i,
    ColonKey              = 58_i,
    SemicolonKey          = 59_i,
    LessKey               = 60_i,
    EqualsKey             = 61_i,
    GreaterKey            = 62_i,
    QuestionKey           = 63_i,
    AtKey                 = 64_i,
    LeftBracketKey        = 91_i,
    BackslashKey          = 92_i,
    RightBracketKey       = 93_i,
    CaretKey              = 94_i,
    UnderscoreKey         = 95_i,
    BackquoteKey          = 96_i,
    AKey                  = 97_i,
    BKey                  = 98_i,
    CKey                  = 99_i,
    DKey                  = 100_i,
    EKey                  = 101_i,
    FKey                  = 102_i,
    GKey                  = 103_i,
    HKey                  = 104_i,
    IKey                  = 105_i,
    JKey                  = 106_i,
    KKey                  = 107_i,
    LKey                  = 108_i,
    MKey                  = 109_i,
    NKey                  = 110_i,
    OKey                  = 111_i,
    PKey                  = 112_i,
    QKey                  = 113_i,
    RKey                  = 114_i,
    SKey                  = 115_i,
    TKey                  = 116_i,
    UKey                  = 117_i,
    VKey                  = 118_i,
    WKey                  = 119_i,
    XKey                  = 120_i,
    YKey                  = 121_i,
    ZKey                  = 122_i,
    DeleteKey             = 127_i,
    CapsLockKey           = 1073741881_i,
    F1Key                 = 1073741882_i,
    F2Key                 = 1073741883_i,
    F3Key                 = 1073741884_i,
    F4Key                 = 1073741885_i,
    F5Key                 = 1073741886_i,
    F6Key                 = 1073741887_i,
    F7Key                 = 1073741888_i,
    F8Key                 = 1073741889_i,
    F9Key                 = 1073741890_i,
    F10Key                = 1073741891_i,
    F11Key                = 1073741892_i,
    F12Key                = 1073741893_i,
    PrintScreenKey        = 1073741894_i,
    ScrollLockKey         = 1073741895_i,
    PauseKey              = 1073741896_i,
    InsertKey             = 1073741897_i,
    HomeKey               = 1073741898_i,
    PageUpKey             = 1073741899_i,
    EndKey                = 1073741901_i,
    PageDownKey           = 1073741902_i,
    RightKey              = 1073741903_i,
    LeftKey               = 1073741904_i,
    DownKey               = 1073741905_i,
    UpKey                 = 1073741906_i,
    NumLockClearKey       = 1073741907_i,
    KpDivideKey           = 1073741908_i,
    KpMultiplyKey         = 1073741909_i,
    KpMinusKey            = 1073741910_i,
    KpPlusKey             = 1073741911_i,
    KpEnterKey            = 1073741912_i,
    Kp1Key                = 1073741913_i,
    Kp2Key                = 1073741914_i,
    Kp3Key                = 1073741915_i,
    Kp4Key                = 1073741916_i,
    Kp5Key                = 1073741917_i,
    Kp6Key                = 1073741918_i,
    Kp7Key                = 1073741919_i,
    Kp8Key                = 1073741920_i,
    Kp9Key                = 1073741921_i,
    Kp0Key                = 1073741922_i,
    KpPeriodKey           = 1073741923_i,
    ApplicationKey        = 1073741925_i,
    PowerKey              = 1073741926_i,
    KpEqualsKey           = 1073741927_i,
    F13Key                = 1073741928_i,
    F14Key                = 1073741929_i,
    F15Key                = 1073741930_i,
    F16Key                = 1073741931_i,
    F17Key                = 1073741932_i,
    F18Key                = 1073741933_i,
    F19Key                = 1073741934_i,
    F20Key                = 1073741935_i,
    F21Key                = 1073741936_i,
    F22Key                = 1073741937_i,
    F23Key                = 1073741938_i,
    F24Key                = 1073741939_i,
    ExecuteKey            = 1073741940_i,
    HelpKey               = 1073741941_i,
    MenuKey               = 1073741942_i,
    SelectKey             = 1073741943_i,
    StopKey               = 1073741944_i,
    AgainKey              = 1073741945_i,
    UndoKey               = 1073741946_i,
    CutKey                = 1073741947_i,
    CopyKey               = 1073741948_i,
    PasteKey              = 1073741949_i,
    FindKey               = 1073741950_i,
    MuteKey               = 1073741951_i,
    VolumeUpKey           = 1073741952_i,
    VolumeDownKey         = 1073741953_i,
    KpCommaKey            = 1073741957_i,
    KpEqualsAS400Key      = 1073741958_i,
    AltEraseKey           = 1073741977_i,
    SysreqKey             = 1073741978_i,
    CancelKey             = 1073741979_i,
    ClearKey              = 1073741980_i,
    PriorKey              = 1073741981_i,
    Return2Key            = 1073741982_i,
    SeparatorKey          = 1073741983_i,
    OutKey                = 1073741984_i,
    OperKey               = 1073741985_i,
    ClearAgainKey         = 1073741986_i,
    CrSelKey              = 1073741987_i,
    ExSelKey              = 1073741988_i,
    Kp00Key               = 1073742000_i,
    Kp000Key              = 1073742001_i,
    ThousandsSeparatorKey = 1073742002_i,
    DecimalSeparatorKey   = 1073742003_i,
    CurrencyUnitKey       = 1073742004_i,
    CurrencySubUnitKey    = 1073742005_i,
    KpLeftParenKey        = 1073742006_i,
    KpRightParenKey       = 1073742007_i,
    KpLeftBraceKey        = 1073742008_i,
    KpRightBraceKey       = 1073742009_i,
    KpTabKey              = 1073742010_i,
    KpBackspaceKey        = 1073742011_i,
    KpAKey                = 1073742012_i,
    KpBKey                = 1073742013_i,
    KpCKey                = 1073742014_i,
    KpDKey                = 1073742015_i,
    KpEKey                = 1073742016_i,
    KpFKey                = 1073742017_i,
    KpXorKey              = 1073742018_i,
    KpPowerKey            = 1073742019_i,
    KpPercentKey          = 1073742020_i,
    KpLessKey             = 1073742021_i,
    KpGreaterKey          = 1073742022_i,
    KpAmpersandKey        = 1073742023_i,
    KpDblAmpersandKey     = 1073742024_i,
    KpVerticalBarKey      = 1073742025_i,
    KpDblVerticalBarKey   = 1073742026_i,
    KpColonKey            = 1073742027_i,
    KpHashKey             = 1073742028_i,
    KpSpaceKey            = 1073742029_i,
    KpAtKey               = 1073742030_i,
    KpExclamKey           = 1073742031_i,
    KpMemStoreKey         = 1073742032_i,
    KpMemRecallKey        = 1073742033_i,
    KpMemClearKey         = 1073742034_i,
    KpMemAddKey           = 1073742035_i,
    KpMemSubtractKey      = 1073742036_i,
    KpMemMultiplyKey      = 1073742037_i,
    KpMemDivideKey        = 1073742038_i,
    KpPlusMinusKey        = 1073742039_i,
    KpCearKey             = 1073742040_i,
    KpClearEntryKey       = 1073742041_i,
    KpBinaryKey           = 1073742042_i,
    KpOctalKey            = 1073742043_i,
    KpDecimalKey          = 1073742044_i,
    KpHexadecimalKey      = 1073742045_i,
    LCtrlKey              = 1073742048_i,
    LShiftKey             = 1073742049_i,
    LAltKey               = 1073742050_i,
    LGuiKey               = 1073742051_i,
    RCtrlKey              = 1073742052_i,
    RShiftKey             = 1073742053_i,
    RAltKey               = 1073742054_i,
    RGuiKey               = 1073742055_i,
    ModeKey               = 1073742081_i,
    AudioNextKey          = 1073742082_i,
    AudioPrevKey          = 1073742083_i,
    AudioStopKey          = 1073742084_i,
    AudioPlayKey          = 1073742085_i,
    AudioMuteKey          = 1073742086_i,
    MediaSelectKey        = 1073742087_i,
    WwwKey                = 1073742088_i,
    MailKey               = 1073742089_i,
    CalculatorKey         = 1073742090_i,
    ComputerKey           = 1073742091_i,
    AcSearchKey           = 1073742092_i,
    AcHomeKey             = 1073742093_i,
    AcBackKey             = 1073742094_i,
    AcForwardKey          = 1073742095_i,
    AcStopKey             = 1073742096_i,
    AcRefreshKey          = 1073742097_i,
    AcBookmarksKey        = 1073742098_i,
    BrightnessDownKey     = 1073742099_i,
    BrightnessUpKey       = 1073742100_i,
    DisplaySwitchKey      = 1073742101_i,
    KbdIllumToggleKey     = 1073742102_i,
    KbdIllumDownKey       = 1073742103_i,
    KbdIllumUpKey         = 1073742104_i,
    EjectKey              = 1073742105_i,
    SleepKey              = 1073742106_i,
}

impl KeyCode {
    /// Get the code
    pub fn code(&self) -> i32 {
        match *self {
            UnknownKey            => 0,
            BackspaceKey          => 8,
            TabKey                => 9,
            ReturnKey             => 13,
            EscapeKey             => 27,
            SpaceKey              => 32,
            ExclaimKey            => 33,
            QuotedblKey           => 34,
            HashKey               => 35,
            DollarKey             => 36,
            PercentKey            => 37,
            AmpersandKey          => 38,
            QuoteKey              => 39,
            LeftParenKey          => 40,
            RightParenKey         => 41,
            AsteriskKey           => 42,
            PlusKey               => 43,
            CommaKey              => 44,
            MinusKey              => 45,
            PeriodKey             => 46,
            SlashKey              => 47,
            Num0Key               => 48,
            Num1Key               => 49,
            Num2Key               => 50,
            Num3Key               => 51,
            Num4Key               => 52,
            Num5Key               => 53,
            Num6Key               => 54,
            Num7Key               => 55,
            Num8Key               => 56,
            Num9Key               => 57,
            ColonKey              => 58,
            SemicolonKey          => 59,
            LessKey               => 60,
            EqualsKey             => 61,
            GreaterKey            => 62,
            QuestionKey           => 63,
            AtKey                 => 64,
            LeftBracketKey        => 91,
            BackslashKey          => 92,
            RightBracketKey       => 93,
            CaretKey              => 94,
            UnderscoreKey         => 95,
            BackquoteKey          => 96,
            AKey                  => 97,
            BKey                  => 98,
            CKey                  => 99,
            DKey                  => 100,
            EKey                  => 101,
            FKey                  => 102,
            GKey                  => 103,
            HKey                  => 104,
            IKey                  => 105,
            JKey                  => 106,
            KKey                  => 107,
            LKey                  => 108,
            MKey                  => 109,
            NKey                  => 110,
            OKey                  => 111,
            PKey                  => 112,
            QKey                  => 113,
            RKey                  => 114,
            SKey                  => 115,
            TKey                  => 116,
            UKey                  => 117,
            VKey                  => 118,
            WKey                  => 119,
            XKey                  => 120,
            YKey                  => 121,
            ZKey                  => 122,
            DeleteKey             => 127,
            CapsLockKey           => 1073741881,
            F1Key                 => 1073741882,
            F2Key                 => 1073741883,
            F3Key                 => 1073741884,
            F4Key                 => 1073741885,
            F5Key                 => 1073741886,
            F6Key                 => 1073741887,
            F7Key                 => 1073741888,
            F8Key                 => 1073741889,
            F9Key                 => 1073741890,
            F10Key                => 1073741891,
            F11Key                => 1073741892,
            F12Key                => 1073741893,
            PrintScreenKey        => 1073741894,
            ScrollLockKey         => 1073741895,
            PauseKey              => 1073741896,
            InsertKey             => 1073741897,
            HomeKey               => 1073741898,
            PageUpKey             => 1073741899,
            EndKey                => 1073741901,
            PageDownKey           => 1073741902,
            RightKey              => 1073741903,
            LeftKey               => 1073741904,
            DownKey               => 1073741905,
            UpKey                 => 1073741906,
            NumLockClearKey       => 1073741907,
            KpDivideKey           => 1073741908,
            KpMultiplyKey         => 1073741909,
            KpMinusKey            => 1073741910,
            KpPlusKey             => 1073741911,
            KpEnterKey            => 1073741912,
            Kp1Key                => 1073741913,
            Kp2Key                => 1073741914,
            Kp3Key                => 1073741915,
            Kp4Key                => 1073741916,
            Kp5Key                => 1073741917,
            Kp6Key                => 1073741918,
            Kp7Key                => 1073741919,
            Kp8Key                => 1073741920,
            Kp9Key                => 1073741921,
            Kp0Key                => 1073741922,
            KpPeriodKey           => 1073741923,
            ApplicationKey        => 1073741925,
            PowerKey              => 1073741926,
            KpEqualsKey           => 1073741927,
            F13Key                => 1073741928,
            F14Key                => 1073741929,
            F15Key                => 1073741930,
            F16Key                => 1073741931,
            F17Key                => 1073741932,
            F18Key                => 1073741933,
            F19Key                => 1073741934,
            F20Key                => 1073741935,
            F21Key                => 1073741936,
            F22Key                => 1073741937,
            F23Key                => 1073741938,
            F24Key                => 1073741939,
            ExecuteKey            => 1073741940,
            HelpKey               => 1073741941,
            MenuKey               => 1073741942,
            SelectKey             => 1073741943,
            StopKey               => 1073741944,
            AgainKey              => 1073741945,
            UndoKey               => 1073741946,
            CutKey                => 1073741947,
            CopyKey               => 1073741948,
            PasteKey              => 1073741949,
            FindKey               => 1073741950,
            MuteKey               => 1073741951,
            VolumeUpKey           => 1073741952,
            VolumeDownKey         => 1073741953,
            KpCommaKey            => 1073741957,
            KpEqualsAS400Key      => 1073741958,
            AltEraseKey           => 1073741977,
            SysreqKey             => 1073741978,
            CancelKey             => 1073741979,
            ClearKey              => 1073741980,
            PriorKey              => 1073741981,
            Return2Key            => 1073741982,
            SeparatorKey          => 1073741983,
            OutKey                => 1073741984,
            OperKey               => 1073741985,
            ClearAgainKey         => 1073741986,
            CrSelKey              => 1073741987,
            ExSelKey              => 1073741988,
            Kp00Key               => 1073742000,
            Kp000Key              => 1073742001,
            ThousandsSeparatorKey => 1073742002,
            DecimalSeparatorKey   => 1073742003,
            CurrencyUnitKey       => 1073742004,
            CurrencySubUnitKey    => 1073742005,
            KpLeftParenKey        => 1073742006,
            KpRightParenKey       => 1073742007,
            KpLeftBraceKey        => 1073742008,
            KpRightBraceKey       => 1073742009,
            KpTabKey              => 1073742010,
            KpBackspaceKey        => 1073742011,
            KpAKey                => 1073742012,
            KpBKey                => 1073742013,
            KpCKey                => 1073742014,
            KpDKey                => 1073742015,
            KpEKey                => 1073742016,
            KpFKey                => 1073742017,
            KpXorKey              => 1073742018,
            KpPowerKey            => 1073742019,
            KpPercentKey          => 1073742020,
            KpLessKey             => 1073742021,
            KpGreaterKey          => 1073742022,
            KpAmpersandKey        => 1073742023,
            KpDblAmpersandKey     => 1073742024,
            KpVerticalBarKey      => 1073742025,
            KpDblVerticalBarKey   => 1073742026,
            KpColonKey            => 1073742027,
            KpHashKey             => 1073742028,
            KpSpaceKey            => 1073742029,
            KpAtKey               => 1073742030,
            KpExclamKey           => 1073742031,
            KpMemStoreKey         => 1073742032,
            KpMemRecallKey        => 1073742033,
            KpMemClearKey         => 1073742034,
            KpMemAddKey           => 1073742035,
            KpMemSubtractKey      => 1073742036,
            KpMemMultiplyKey      => 1073742037,
            KpMemDivideKey        => 1073742038,
            KpPlusMinusKey        => 1073742039,
            KpCearKey             => 1073742040,
            KpClearEntryKey       => 1073742041,
            KpBinaryKey           => 1073742042,
            KpOctalKey            => 1073742043,
            KpDecimalKey          => 1073742044,
            KpHexadecimalKey      => 1073742045,
            LCtrlKey              => 1073742048,
            LShiftKey             => 1073742049,
            LAltKey               => 1073742050,
            LGuiKey               => 1073742051,
            RCtrlKey              => 1073742052,
            RShiftKey             => 1073742053,
            RAltKey               => 1073742054,
            RGuiKey               => 1073742055,
            ModeKey               => 1073742081,
            AudioNextKey          => 1073742082,
            AudioPrevKey          => 1073742083,
            AudioStopKey          => 1073742084,
            AudioPlayKey          => 1073742085,
            AudioMuteKey          => 1073742086,
            MediaSelectKey        => 1073742087,
            WwwKey                => 1073742088,
            MailKey               => 1073742089,
            CalculatorKey         => 1073742090,
            ComputerKey           => 1073742091,
            AcSearchKey           => 1073742092,
            AcHomeKey             => 1073742093,
            AcBackKey             => 1073742094,
            AcForwardKey          => 1073742095,
            AcStopKey             => 1073742096,
            AcRefreshKey          => 1073742097,
            AcBookmarksKey        => 1073742098,
            BrightnessDownKey     => 1073742099,
            BrightnessUpKey       => 1073742100,
            DisplaySwitchKey      => 1073742101,
            KbdIllumToggleKey     => 1073742102,
            KbdIllumDownKey       => 1073742103,
            KbdIllumUpKey         => 1073742104,
            EjectKey              => 1073742105,
            SleepKey              => 1073742106,

        }
    }

    fn to_int(&self) -> int {
        self.code() as int
    }

    fn from_int(n: int) -> KeyCode {
        match n {
            0 => UnknownKey,
            8 => BackspaceKey,
            9 => TabKey,
            13 => ReturnKey,
            27 => EscapeKey,
            32 => SpaceKey,
            33 => ExclaimKey,
            34 => QuotedblKey,
            35 => HashKey,
            36 => DollarKey,
            37 => PercentKey,
            38 => AmpersandKey,
            39 => QuoteKey,
            40 => LeftParenKey,
            41 => RightParenKey,
            42 => AsteriskKey,
            43 => PlusKey,
            44 => CommaKey,
            45 => MinusKey,
            46 => PeriodKey,
            47 => SlashKey,
            48 => Num0Key,
            49 => Num1Key,
            50 => Num2Key,
            51 => Num3Key,
            52 => Num4Key,
            53 => Num5Key,
            54 => Num6Key,
            55 => Num7Key,
            56 => Num8Key,
            57 => Num9Key,
            58 => ColonKey,
            59 => SemicolonKey,
            60 => LessKey,
            61 => EqualsKey,
            62 => GreaterKey,
            63 => QuestionKey,
            64 => AtKey,
            91 => LeftBracketKey,
            92 => BackslashKey,
            93 => RightBracketKey,
            94 => CaretKey,
            95 => UnderscoreKey,
            96 => BackquoteKey,
            97 => AKey,
            98 => BKey,
            99 => CKey,
            100 => DKey,
            101 => EKey,
            102 => FKey,
            103 => GKey,
            104 => HKey,
            105 => IKey,
            106 => JKey,
            107 => KKey,
            108 => LKey,
            109 => MKey,
            110 => NKey,
            111 => OKey,
            112 => PKey,
            113 => QKey,
            114 => RKey,
            115 => SKey,
            116 => TKey,
            117 => UKey,
            118 => VKey,
            119 => WKey,
            120 => XKey,
            121 => YKey,
            122 => ZKey,
            127 => DeleteKey,
            1073741881 => CapsLockKey,
            1073741882 => F1Key,
            1073741883 => F2Key,
            1073741884 => F3Key,
            1073741885 => F4Key,
            1073741886 => F5Key,
            1073741887 => F6Key,
            1073741888 => F7Key,
            1073741889 => F8Key,
            1073741890 => F9Key,
            1073741891 => F10Key,
            1073741892 => F11Key,
            1073741893 => F12Key,
            1073741894 => PrintScreenKey,
            1073741895 => ScrollLockKey,
            1073741896 => PauseKey,
            1073741897 => InsertKey,
            1073741898 => HomeKey,
            1073741899 => PageUpKey,
            1073741901 => EndKey,
            1073741902 => PageDownKey,
            1073741903 => RightKey,
            1073741904 => LeftKey,
            1073741905 => DownKey,
            1073741906 => UpKey,
            1073741907 => NumLockClearKey,
            1073741908 => KpDivideKey,
            1073741909 => KpMultiplyKey,
            1073741910 => KpMinusKey,
            1073741911 => KpPlusKey,
            1073741912 => KpEnterKey,
            1073741913 => Kp1Key,
            1073741914 => Kp2Key,
            1073741915 => Kp3Key,
            1073741916 => Kp4Key,
            1073741917 => Kp5Key,
            1073741918 => Kp6Key,
            1073741919 => Kp7Key,
            1073741920 => Kp8Key,
            1073741921 => Kp9Key,
            1073741922 => Kp0Key,
            1073741923 => KpPeriodKey,
            1073741925 => ApplicationKey,
            1073741926 => PowerKey,
            1073741927 => KpEqualsKey,
            1073741928 => F13Key,
            1073741929 => F14Key,
            1073741930 => F15Key,
            1073741931 => F16Key,
            1073741932 => F17Key,
            1073741933 => F18Key,
            1073741934 => F19Key,
            1073741935 => F20Key,
            1073741936 => F21Key,
            1073741937 => F22Key,
            1073741938 => F23Key,
            1073741939 => F24Key,
            1073741940 => ExecuteKey,
            1073741941 => HelpKey,
            1073741942 => MenuKey,
            1073741943 => SelectKey,
            1073741944 => StopKey,
            1073741945 => AgainKey,
            1073741946 => UndoKey,
            1073741947 => CutKey,
            1073741948 => CopyKey,
            1073741949 => PasteKey,
            1073741950 => FindKey,
            1073741951 => MuteKey,
            1073741952 => VolumeUpKey,
            1073741953 => VolumeDownKey,
            1073741957 => KpCommaKey,
            1073741958 => KpEqualsAS400Key,
            1073741977 => AltEraseKey,
            1073741978 => SysreqKey,
            1073741979 => CancelKey,
            1073741980 => ClearKey,
            1073741981 => PriorKey,
            1073741982 => Return2Key,
            1073741983 => SeparatorKey,
            1073741984 => OutKey,
            1073741985 => OperKey,
            1073741986 => ClearAgainKey,
            1073741987 => CrSelKey,
            1073741988 => ExSelKey,
            1073742000 => Kp00Key,
            1073742001 => Kp000Key,
            1073742002 => ThousandsSeparatorKey,
            1073742003 => DecimalSeparatorKey,
            1073742004 => CurrencyUnitKey,
            1073742005 => CurrencySubUnitKey,
            1073742006 => KpLeftParenKey,
            1073742007 => KpRightParenKey,
            1073742008 => KpLeftBraceKey,
            1073742009 => KpRightBraceKey,
            1073742010 => KpTabKey,
            1073742011 => KpBackspaceKey,
            1073742012 => KpAKey,
            1073742013 => KpBKey,
            1073742014 => KpCKey,
            1073742015 => KpDKey,
            1073742016 => KpEKey,
            1073742017 => KpFKey,
            1073742018 => KpXorKey,
            1073742019 => KpPowerKey,
            1073742020 => KpPercentKey,
            1073742021 => KpLessKey,
            1073742022 => KpGreaterKey,
            1073742023 => KpAmpersandKey,
            1073742024 => KpDblAmpersandKey,
            1073742025 => KpVerticalBarKey,
            1073742026 => KpDblVerticalBarKey,
            1073742027 => KpColonKey,
            1073742028 => KpHashKey,
            1073742029 => KpSpaceKey,
            1073742030 => KpAtKey,
            1073742031 => KpExclamKey,
            1073742032 => KpMemStoreKey,
            1073742033 => KpMemRecallKey,
            1073742034 => KpMemClearKey,
            1073742035 => KpMemAddKey,
            1073742036 => KpMemSubtractKey,
            1073742037 => KpMemMultiplyKey,
            1073742038 => KpMemDivideKey,
            1073742039 => KpPlusMinusKey,
            1073742040 => KpCearKey,
            1073742041 => KpClearEntryKey,
            1073742042 => KpBinaryKey,
            1073742043 => KpOctalKey,
            1073742044 => KpDecimalKey,
            1073742045 => KpHexadecimalKey,
            1073742048 => LCtrlKey,
            1073742049 => LShiftKey,
            1073742050 => LAltKey,
            1073742051 => LGuiKey,
            1073742052 => RCtrlKey,
            1073742053 => RShiftKey,
            1073742054 => RAltKey,
            1073742055 => RGuiKey,
            1073742081 => ModeKey,
            1073742082 => AudioNextKey,
            1073742083 => AudioPrevKey,
            1073742084 => AudioStopKey,
            1073742085 => AudioPlayKey,
            1073742086 => AudioMuteKey,
            1073742087 => MediaSelectKey,
            1073742088 => WwwKey,
            1073742089 => MailKey,
            1073742090 => CalculatorKey,
            1073742091 => ComputerKey,
            1073742092 => AcSearchKey,
            1073742093 => AcHomeKey,
            1073742094 => AcBackKey,
            1073742095 => AcForwardKey,
            1073742096 => AcStopKey,
            1073742097 => AcRefreshKey,
            1073742098 => AcBookmarksKey,
            1073742099 => BrightnessDownKey,
            1073742100 => BrightnessUpKey,
            1073742101 => DisplaySwitchKey,
            1073742102 => KbdIllumToggleKey,
            1073742103 => KbdIllumDownKey,
            1073742104 => KbdIllumUpKey,
            1073742105 => EjectKey,
            1073742106 => SleepKey,

            _   => { UnknownKey }
        }
    }
}
