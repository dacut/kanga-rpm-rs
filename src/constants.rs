//! RPM specific constants
//!
//! These constants were extracted from the rpm upstream project
//! C headers.

use enum_display_derive::Display as EnumDisplay;
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::Display;

pub const HEADER_IMAGE: isize = 61;
pub const HEADER_SIGNATURES: isize = 62;
pub const HEADER_IMMUTABLE: isize = 63;
pub const HEADER_REGIONS: isize = 64;
pub const HEADER_I18NTABLE: isize = 100;
pub const HEADER_SIGBASE: isize = 256;
pub const HEADER_TAGBASE: isize = 1000;
pub const RPMTAG_SIG_BASE: isize = HEADER_SIGBASE;

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Copy, Clone, EnumDisplay)]
#[allow(non_camel_case_types)]
pub enum IndexTag {
    RPMTAG_HEADERIMAGE = HEADER_IMAGE,
    RPMTAG_HEADERSIGNATURES = HEADER_SIGNATURES,
    RPMTAG_HEADERIMMUTABLE = HEADER_IMMUTABLE,
    RPMTAG_HEADERREGIONS = HEADER_REGIONS,
    RPMTAG_HEADERI18NTABLE = HEADER_I18NTABLE,

    RPMTAG_SIGSIZE = RPMTAG_SIG_BASE,
    RPMTAG_SIGLEMD5_1 = RPMTAG_SIG_BASE + 2,
    RPMTAG_SIGPGP = RPMTAG_SIG_BASE + 3,
    RPMTAG_SIGLEMD5_2 = RPMTAG_SIG_BASE + 4,
    RPMTAG_SIGMD5 = RPMTAG_SIG_BASE + 5,
    RPMTAG_SIGGPG = RPMTAG_SIG_BASE + 6,
    RPMTAG_SIGPGP5 = RPMTAG_SIG_BASE + 7,
    RPMTAG_BADSHA1_1 = RPMTAG_SIG_BASE + 8,
    RPMTAG_BADSHA1_2 = RPMTAG_SIG_BASE + 9,
    RPMTAG_PUBKEYS = RPMTAG_SIG_BASE + 10,
    RPMTAG_DSAHEADER = RPMTAG_SIG_BASE + 11,
    RPMTAG_RSAHEADER = RPMTAG_SIG_BASE + 12,
    RPMTAG_SHA1HEADER = RPMTAG_SIG_BASE + 13,
    RPMTAG_LONGSIGSIZE = RPMTAG_SIG_BASE + 14,
    RPMTAG_LONGARCHIVESIZE = RPMTAG_SIG_BASE + 15,
    RPMTAG_SHA256HEADER = RPMTAG_SIG_BASE + 17,

    RPMTAG_NAME = 1000,
    RPMTAG_VERSION = 1001,
    RPMTAG_RELEASE = 1002,
    RPMTAG_EPOCH = 1003,
    RPMTAG_SUMMARY = 1004,
    RPMTAG_DESCRIPTION = 1005,
    RPMTAG_BUILDTIME = 1006,
    RPMTAG_BUILDHOST = 1007,
    RPMTAG_INSTALLTIME = 1008,
    RPMTAG_SIZE = 1009,
    RPMTAG_DISTRIBUTION = 1010,
    RPMTAG_VENDOR = 1011,
    RPMTAG_GIF = 1012,
    RPMTAG_XPM = 1013,
    RPMTAG_LICENSE = 1014,
    RPMTAG_PACKAGER = 1015,
    RPMTAG_GROUP = 1016,
    RPMTAG_CHANGELOG = 1017,
    RPMTAG_SOURCE = 1018,
    RPMTAG_PATCH = 1019,
    RPMTAG_URL = 1020,
    RPMTAG_OS = 1021,
    RPMTAG_ARCH = 1022,
    RPMTAG_PREIN = 1023,
    RPMTAG_POSTIN = 1024,
    RPMTAG_PREUN = 1025,
    RPMTAG_POSTUN = 1026,
    RPMTAG_OLDFILENAMES = 1027,
    RPMTAG_FILESIZES = 1028,
    RPMTAG_FILESTATES = 1029,
    RPMTAG_FILEMODES = 1030,
    RPMTAG_FILEUIDS = 1031,
    RPMTAG_FILEGIDS = 1032,
    RPMTAG_FILERDEVS = 1033,
    RPMTAG_FILEMTIMES = 1034,
    RPMTAG_FILEDIGESTS = 1035,
    RPMTAG_FILELINKTOS = 1036,
    RPMTAG_FILEFLAGS = 1037,
    RPMTAG_ROOT = 1038,
    RPMTAG_FILEUSERNAME = 1039,
    RPMTAG_FILEGROUPNAME = 1040,
    RPMTAG_EXCLUDE = 1041,
    RPMTAG_EXCLUSIVE = 1042,
    RPMTAG_ICON = 1043,
    RPMTAG_SOURCERPM = 1044,
    RPMTAG_FILEVERIFYFLAGS = 1045,
    RPMTAG_ARCHIVESIZE = 1046,
    RPMTAG_PROVIDENAME = 1047,
    RPMTAG_REQUIREFLAGS = 1048,
    RPMTAG_REQUIRENAME = 1049,
    RPMTAG_REQUIREVERSION = 1050,
    RPMTAG_NOSOURCE = 1051,
    RPMTAG_NOPATCH = 1052,
    RPMTAG_CONFLICTFLAGS = 1053,
    RPMTAG_CONFLICTNAME = 1054,
    RPMTAG_CONFLICTVERSION = 1055,
    RPMTAG_DEFAULTPREFIX = 1056,
    RPMTAG_BUILDROOT = 1057,
    RPMTAG_INSTALLPREFIX = 1058,
    RPMTAG_EXCLUDEARCH = 1059,
    RPMTAG_EXCLUDEOS = 1060,
    RPMTAG_EXCLUSIVEARCH = 1061,
    RPMTAG_EXCLUSIVEOS = 1062,
    RPMTAG_AUTOREQPROV = 1063,
    RPMTAG_RPMVERSION = 1064,
    RPMTAG_TRIGGERSCRIPTS = 1065,
    RPMTAG_TRIGGERNAME = 1066,
    RPMTAG_TRIGGERVERSION = 1067,
    RPMTAG_TRIGGERFLAGS = 1068,
    RPMTAG_TRIGGERINDEX = 1069,
    RPMTAG_VERIFYSCRIPT = 1079,
    RPMTAG_CHANGELOGTIME = 1080,
    RPMTAG_CHANGELOGNAME = 1081,
    RPMTAG_CHANGELOGTEXT = 1082,
    RPMTAG_BROKENMD5 = 1083,
    RPMTAG_PREREQ = 1084,
    RPMTAG_PREINPROG = 1085,
    RPMTAG_POSTINPROG = 1086,
    RPMTAG_PREUNPROG = 1087,
    RPMTAG_POSTUNPROG = 1088,
    RPMTAG_BUILDARCHS = 1089,
    RPMTAG_OBSOLETENAME = 1090,
    RPMTAG_VERIFYSCRIPTPROG = 1091,
    RPMTAG_TRIGGERSCRIPTPROG = 1092,
    RPMTAG_DOCDIR = 1093,
    RPMTAG_COOKIE = 1094,
    RPMTAG_FILEDEVICES = 1095,
    RPMTAG_FILEINODES = 1096,
    RPMTAG_FILELANGS = 1097,
    RPMTAG_PREFIXES = 1098,
    RPMTAG_INSTPREFIXES = 1099,
    RPMTAG_TRIGGERIN = 1100,
    RPMTAG_TRIGGERUN = 1101,
    RPMTAG_TRIGGERPOSTUN = 1102,
    RPMTAG_AUTOREQ = 1103,
    RPMTAG_AUTOPROV = 1104,
    RPMTAG_CAPABILITY = 1105,
    RPMTAG_SOURCEPACKAGE = 1106,
    RPMTAG_OLDORIGFILENAMES = 1107,
    RPMTAG_BUILDPREREQ = 1108,
    RPMTAG_BUILDREQUIRES = 1109,
    RPMTAG_BUILDCONFLICTS = 1110,
    RPMTAG_BUILDMACROS = 1111,
    RPMTAG_PROVIDEFLAGS = 1112,
    RPMTAG_PROVIDEVERSION = 1113,
    RPMTAG_OBSOLETEFLAGS = 1114,
    RPMTAG_OBSOLETEVERSION = 1115,
    RPMTAG_DIRINDEXES = 1116,
    RPMTAG_BASENAMES = 1117,
    RPMTAG_DIRNAMES = 1118,
    RPMTAG_ORIGDIRINDEXES = 1119,
    RPMTAG_ORIGBASENAMES = 1120,
    RPMTAG_ORIGDIRNAMES = 1121,
    RPMTAG_OPTFLAGS = 1122,
    RPMTAG_DISTURL = 1123,
    RPMTAG_PAYLOADFORMAT = 1124,
    RPMTAG_PAYLOADCOMPRESSOR = 1125,
    RPMTAG_PAYLOADFLAGS = 1126,
    RPMTAG_INSTALLCOLOR = 1127,
    RPMTAG_INSTALLTID = 1128,
    RPMTAG_REMOVETID = 1129,
    RPMTAG_SHA1RHN = 1130,
    RPMTAG_RHNPLATFORM = 1131,
    RPMTAG_PLATFORM = 1132,
    RPMTAG_PATCHESNAME = 1133,
    RPMTAG_PATCHESFLAGS = 1134,
    RPMTAG_PATCHESVERSION = 1135,
    RPMTAG_CACHECTIME = 1136,
    RPMTAG_CACHEPKGPATH = 1137,
    RPMTAG_CACHEPKGSIZE = 1138,
    RPMTAG_CACHEPKGMTIME = 1139,
    RPMTAG_FILECOLORS = 1140,
    RPMTAG_FILECLASS = 1141,
    RPMTAG_CLASSDICT = 1142,
    RPMTAG_FILEDEPENDSX = 1143,
    RPMTAG_FILEDEPENDSN = 1144,
    RPMTAG_DEPENDSDICT = 1145,
    RPMTAG_SOURCEPKGID = 1146,
    RPMTAG_FILECONTEXTS = 1147,
    RPMTAG_FSCONTEXTS = 1148,
    RPMTAG_RECONTEXTS = 1149,
    RPMTAG_POLICIES = 1150,
    RPMTAG_PRETRANS = 1151,
    RPMTAG_POSTTRANS = 1152,
    RPMTAG_PRETRANSPROG = 1153,
    RPMTAG_POSTTRANSPROG = 1154,
    RPMTAG_DISTTAG = 1155,
    RPMTAG_OLDSUGGESTSNAME = 1156,
    RPMTAG_OLDSUGGESTSVERSION = 1157,
    RPMTAG_OLDSUGGESTSFLAGS = 1158,
    RPMTAG_OLDENHANCESNAME = 1159,
    RPMTAG_OLDENHANCESVERSION = 1160,
    RPMTAG_OLDENHANCESFLAGS = 1161,
    RPMTAG_PRIORITY = 1162,
    RPMTAG_CVSID = 1163,
    RPMTAG_BLINKPKGID = 1164,
    RPMTAG_BLINKHDRID = 1165,
    RPMTAG_BLINKNEVRA = 1166,
    RPMTAG_FLINKPKGID = 1167,
    RPMTAG_FLINKHDRID = 1168,
    RPMTAG_FLINKNEVRA = 1169,
    RPMTAG_PACKAGEORIGIN = 1170,
    RPMTAG_TRIGGERPREIN = 1171,
    RPMTAG_BUILDSUGGESTS = 1172,
    RPMTAG_BUILDENHANCES = 1173,
    RPMTAG_SCRIPTSTATES = 1174,
    RPMTAG_SCRIPTMETRICS = 1175,
    RPMTAG_BUILDCPUCLOCK = 1176,
    RPMTAG_FILEDIGESTALGOS = 1177,
    RPMTAG_VARIANTS = 1178,
    RPMTAG_XMAJOR = 1179,
    RPMTAG_XMINOR = 1180,
    RPMTAG_REPOTAG = 1181,
    RPMTAG_KEYWORDS = 1182,
    RPMTAG_BUILDPLATFORMS = 1183,
    RPMTAG_PACKAGECOLOR = 1184,
    RPMTAG_PACKAGEPREFCOLOR = 1185,
    RPMTAG_XATTRSDICT = 1186,
    RPMTAG_FILEXATTRSX = 1187,
    RPMTAG_DEPATTRSDICT = 1188,
    RPMTAG_CONFLICTATTRSX = 1189,
    RPMTAG_OBSOLETEATTRSX = 1190,
    RPMTAG_PROVIDEATTRSX = 1191,
    RPMTAG_REQUIREATTRSX = 1192,
    RPMTAG_BUILDPROVIDES = 1193,
    RPMTAG_BUILDOBSOLETES = 1194,
    RPMTAG_DBINSTANCE = 1195,
    RPMTAG_NVRA = 1196,
    RPMTAG_FILENAMES = 5000,
    RPMTAG_FILEPROVIDE = 5001,
    RPMTAG_FILEREQUIRE = 5002,
    RPMTAG_FSNAMES = 5003,
    RPMTAG_FSSIZES = 5004,
    RPMTAG_TRIGGERCONDS = 5005,
    RPMTAG_TRIGGERTYPE = 5006,
    RPMTAG_ORIGFILENAMES = 5007,
    RPMTAG_LONGFILESIZES = 5008,
    RPMTAG_LONGSIZE = 5009,
    RPMTAG_FILECAPS = 5010,
    RPMTAG_FILEDIGESTALGO = 5011,
    RPMTAG_BUGURL = 5012,
    RPMTAG_EVR = 5013,
    RPMTAG_NVR = 5014,
    RPMTAG_NEVR = 5015,
    RPMTAG_NEVRA = 5016,
    RPMTAG_HEADERCOLOR = 5017,
    RPMTAG_VERBOSE = 5018,
    RPMTAG_EPOCHNUM = 5019,
    RPMTAG_PREINFLAGS = 5020,
    RPMTAG_POSTINFLAGS = 5021,
    RPMTAG_PREUNFLAGS = 5022,
    RPMTAG_POSTUNFLAGS = 5023,
    RPMTAG_PRETRANSFLAGS = 5024,
    RPMTAG_POSTTRANSFLAGS = 5025,
    RPMTAG_VERIFYSCRIPTFLAGS = 5026,
    RPMTAG_TRIGGERSCRIPTFLAGS = 5027,
    RPMTAG_COLLECTIONS = 5029,
    RPMTAG_POLICYNAMES = 5030,
    RPMTAG_POLICYTYPES = 5031,
    RPMTAG_POLICYTYPESINDEXES = 5032,
    RPMTAG_POLICYFLAGS = 5033,
    RPMTAG_VCS = 5034,
    RPMTAG_ORDERNAME = 5035,
    RPMTAG_ORDERVERSION = 5036,
    RPMTAG_ORDERFLAGS = 5037,
    RPMTAG_MSSFMANIFEST = 5038,
    RPMTAG_MSSFDOMAIN = 5039,
    RPMTAG_INSTFILENAMES = 5040,
    RPMTAG_REQUIRENEVRS = 5041,
    RPMTAG_PROVIDENEVRS = 5042,
    RPMTAG_OBSOLETENEVRS = 5043,
    RPMTAG_CONFLICTNEVRS = 5044,
    RPMTAG_FILENLINKS = 5045,
    RPMTAG_RECOMMENDNAME = 5046,
    RPMTAG_RECOMMENDVERSION = 5047,
    RPMTAG_RECOMMENDFLAGS = 5048,
    RPMTAG_SUGGESTNAME = 5049,
    RPMTAG_SUGGESTVERSION = 5050,
    RPMTAG_SUGGESTFLAGS = 5051,
    RPMTAG_SUPPLEMENTNAME = 5052,
    RPMTAG_SUPPLEMENTVERSION = 5053,
    RPMTAG_SUPPLEMENTFLAGS = 5054,
    RPMTAG_ENHANCENAME = 5055,
    RPMTAG_ENHANCEVERSION = 5056,
    RPMTAG_ENHANCEFLAGS = 5057,
    RPMTAG_RECOMMENDNEVRS = 5058,
    RPMTAG_SUGGESTNEVRS = 5059,
    RPMTAG_SUPPLEMENTNEVRS = 5060,
    RPMTAG_ENHANCENEVRS = 5061,
    RPMTAG_ENCODING = 5062,
    RPMTAG_FILETRIGGERIN = 5063,
    RPMTAG_FILETRIGGERUN = 5064,
    RPMTAG_FILETRIGGERPOSTUN = 5065,
    RPMTAG_FILETRIGGERSCRIPTS = 5066,
    RPMTAG_FILETRIGGERSCRIPTPROG = 5067,
    RPMTAG_FILETRIGGERSCRIPTFLAGS = 5068,
    RPMTAG_FILETRIGGERNAME = 5069,
    RPMTAG_FILETRIGGERINDEX = 5070,
    RPMTAG_FILETRIGGERVERSION = 5071,
    RPMTAG_FILETRIGGERFLAGS = 5072,
    RPMTAG_TRANSFILETRIGGERIN = 5073,
    RPMTAG_TRANSFILETRIGGERUN = 5074,
    RPMTAG_TRANSFILETRIGGERPOSTUN = 5075,
    RPMTAG_TRANSFILETRIGGERSCRIPTS = 5076,
    RPMTAG_TRANSFILETRIGGERSCRIPTPROG = 5077,
    RPMTAG_TRANSFILETRIGGERSCRIPTFLAGS = 5078,
    RPMTAG_TRANSFILETRIGGERNAME = 5079,
    RPMTAG_TRANSFILETRIGGERINDEX = 5080,
    RPMTAG_TRANSFILETRIGGERVERSION = 5081,
    RPMTAG_TRANSFILETRIGGERFLAGS = 5082,
    RPMTAG_REMOVEPATHPOSTFIXES = 5083,
    RPMTAG_FILETRIGGERPRIORITIES = 5084,
    RPMTAG_TRANSFILETRIGGERPRIORITIES = 5085,
    RPMTAG_FILETRIGGERCONDS = 5086,
    RPMTAG_FILETRIGGERTYPE = 5087,
    RPMTAG_TRANSFILETRIGGERCONDS = 5088,
    RPMTAG_TRANSFILETRIGGERTYPE = 5089,
    RPMTAG_FILESIGNATURES = 5090,
    RPMTAG_FILESIGNATURELENGTH = 5091,
    RPMTAG_PAYLOADDIGEST = 5092,
    RPMTAG_PAYLOADDIGESTALGO = 5093,
    RPMTAG_AUTOINSTALLED = 5094,
    RPMTAG_IDENTITY = 5095,
    RPMTAG_MODULARITYLABEL = 5096,
}

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Copy, Clone, EnumDisplay)]
#[allow(non_camel_case_types)]
pub enum IndexSignatureTag {
    HEADER_SIGNATURES = HEADER_SIGNATURES,
    // This tag specifies the combined size of the Header and Payload sections.
    RPMSIGTAG_SIZE = HEADER_TAGBASE,

    //This  tag  specifies  the  uncompressed  size of the Payload archive, including the cpio headers.
    RPMSIGTAG_PAYLOADSIZE = HEADER_TAGBASE + 7,

    //This  index  contains  the  SHA1  checksum  of  the  entire  Header  Section,
    //including the Header Record, Index Records and Header store.
    RPMSIGTAG_SHA1 = 269,

    //This  tag  specifies  the  128-bit  MD5  checksum  of  the  combined  Header  and  Archive sections.
    RPMSIGTAG_MD5 = 1004,

    //The  tag  contains  the  DSA  signature  of  the  Header  section.
    // The  data  is  formatted  as  a  Version  3  Signature  Packet  as  specified  in  RFC  2440:  OpenPGP Message Format.
    // If this tag is present, then the SIGTAG_GPG tag shall also be present.
    RPMSIGTAG_DSA = 267,

    // The  tag  contains  the  RSA  signature  of  the  Header  section.
    // The  data  is  formatted  as  a  Version  3  Signature  Packet  as  specified  in  RFC  2440: OpenPGP  Message  Format.
    // If  this  tag  is  present,  then  the  SIGTAG_PGP  shall also be present.
    RPMSIGTAG_RSA = 268,

    // The tag contains the file signature of a file.
    // The data is formatted as a hex-encoded string.
    // If this tag is present, then the SIGTAG_FILESIGNATURE_LENGTH shall also be present.
    RPMSIGTAG_FILESIGNATURES = 274,

    // The tag contains the length of the file signatures in total.
    // If this tag is present, then the SIGTAG_FILESIGNATURE shall also be present.
    RPMSIGTAG_FILESIGNATURE_LENGTH = 275,

    // This  tag  specifies  the  RSA  signature  of  the  combined  Header  and  Payload  sections.
    // The data is formatted as a Version 3 Signature Packet as specified in RFC 2440: OpenPGP Message Format.
    RPMSIGTAG_PGP = 1002,

    // The  tag  contains  the  DSA  signature  of  the  combined  Header  and  Payload  sections.
    // The data is formatted as a Version 3 Signature Packet as specified in RFC 2440: OpenPGP Message Format.
    RPMSIGTAG_GPG = 1005,

    //This  index  contains  the  SHA256  checksum  of  the  entire  Header  Section,
    //including the Header Record, Index Records and Header store.
    RPMSIGTAG_SHA256 = IndexTag::RPMTAG_SHA256HEADER as isize,

    // A silly tag for a date.
    RPMTAG_INSTALLTIME = IndexTag::RPMTAG_INSTALLTIME as isize,
}

pub trait TypeName {
    fn type_name() -> &'static str;
}

impl TypeName for IndexTag {
    fn type_name() -> &'static str {
        "IndexTag"
    }
}

impl TypeName for IndexSignatureTag {
    fn type_name() -> &'static str {
        "IndexSignatureTag"
    }
}

/// lead header size
pub const LEAD_SIZE: usize = 96;
/// rpm magic as part of the lead header
pub const RPM_MAGIC: [u8; 4] = [0xed, 0xab, 0xee, 0xdb];

/// header magic recognition (not the lead!)
pub const HEADER_MAGIC: [u8; 3] = [0x8e, 0xad, 0xe8];

pub const RPMSENSE_ANY: u32 = 0;
pub const RPMSENSE_LESS: u32 = 1 << 1;
pub const RPMSENSE_GREATER: u32 = 1 << 2;
pub const RPMSENSE_EQUAL: u32 = 1 << 3;

// there is no use yet for those constants. But they are part of the official package
// so I will leave them in in case we need them later.

// const RPMSENSE_POSTTRANS: u32 = (1 << 5);
// const RPMSENSE_PREREQ: u32 = (1 << 6);
// const RPMSENSE_PRETRANS: u32 = (1 << 7);
// const RPMSENSE_INTERP: u32 = (1 << 8);
// const RPMSENSE_SCRIPT_PRE: u32 = (1 << 9);
// const RPMSENSE_SCRIPT_POST: u32 = (1 << 10);
// const RPMSENSE_SCRIPT_PREUN: u32 = (1 << 11);
// const RPMSENSE_SCRIPT_POSTUN: u32 = (1 << 12);
// const RPMSENSE_SCRIPT_VERIFY: u32 = (1 << 13);
// const RPMSENSE_FIND_REQUIRES: u32 = (1 << 14);
// const RPMSENSE_FIND_PROVIDES: u32 = (1 << 15);
// const RPMSENSE_TRIGGERIN: u32 = (1 << 16);
// const RPMSENSE_TRIGGERUN: u32 = (1 << 17);
// const RPMSENSE_TRIGGERPOSTUN: u32 = (1 << 18);
// const RPMSENSE_MISSINGOK: u32 = (1 << 19);

// // for some weird reason, centos packages have another value for rpm lib sense. We have to observe this.
// const RPMSENSE_RPMLIB: u32 = (1 << 24); //0o100000012;
// const RPMSENSE_TRIGGERPREIN: u32 = (1 << 25);
// const RPMSENSE_KEYRING: u32 = (1 << 26);
// const RPMSENSE_CONFIG: u32 = (1 << 28);

pub const RPMFILE_CONFIG: i32 = 1;
pub const RPMFILE_DOC: i32 = 1 << 1;
// const RPMFILE_DONOTUSE: i32 = (1 << 2);
// const RPMFILE_MISSINGOK: i32 = (1 << 3);
// const RPMFILE_NOREPLACE: i32 = (1 << 4);
// const RPMFILE_SPECFILE: i32 = (1 << 5);
// const RPMFILE_GHOST: i32 = (1 << 6);
// const RPMFILE_LICENSE: i32 = (1 << 7);
// const RPMFILE_README: i32 = (1 << 8);
// const RPMFILE_EXCLUDE: i32 = (1 << 9);
