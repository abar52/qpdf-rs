use std::{env, path::PathBuf};

const ZLIB_SRC: &[&str] = &[
    "adler32.c",
    "compress.c",
    "crc32.c",
    "deflate.c",
    "infback.c",
    "inffast.c",
    "inflate.c",
    "inftrees.c",
    "trees.c",
    "uncompr.c",
    "zutil.c",
];

const JPEG_SRC: &[&str] = &[
    "jaricom.c",
    "jcapimin.c",
    "jcapistd.c",
    "jcarith.c",
    "jccoefct.c",
    "jccolor.c",
    "jcdctmgr.c",
    "jchuff.c",
    "jcinit.c",
    "jcmainct.c",
    "jcmarker.c",
    "jcmaster.c",
    "jcomapi.c",
    "jcparam.c",
    "jcprepct.c",
    "jcsample.c",
    "jctrans.c",
    "jdapimin.c",
    "jdapistd.c",
    "jdarith.c",
    "jdatadst.c",
    "jdatasrc.c",
    "jdcoefct.c",
    "jdcolor.c",
    "jddctmgr.c",
    "jdhuff.c",
    "jdinput.c",
    "jdmainct.c",
    "jdmarker.c",
    "jdmaster.c",
    "jdmerge.c",
    "jdpostct.c",
    "jdsample.c",
    "jdtrans.c",
    "jerror.c",
    "jfdctflt.c",
    "jfdctfst.c",
    "jfdctint.c",
    "jidctflt.c",
    "jidctfst.c",
    "jidctint.c",
    "jmemmgr.c",
    "jmemnobs.c",
    "jquant1.c",
    "jquant2.c",
    "jutils.c",
];

const QPDF_SRC: &[&str] = &[
    "AES_PDF_native.cc",
    "BitStream.cc",
    "BitWriter.cc",
    "Buffer.cc",
    "BufferInputSource.cc",
    "ClosedFileInputSource.cc",
    "ContentNormalizer.cc",
    "CryptoRandomDataProvider.cc",
    "FileInputSource.cc",
    "InputSource.cc",
    "InsecureRandomDataProvider.cc",
    "JSON.cc",
    "JSONHandler.cc",
    "MD5.cc",
    "MD5_native.cc",
    "NNTree.cc",
    "OffsetInputSource.cc",
    "PDFVersion.cc",
    "Pipeline.cc",
    "Pl_AES_PDF.cc",
    "Pl_ASCII85Decoder.cc",
    "Pl_ASCIIHexDecoder.cc",
    "Pl_Base64.cc",
    "Pl_Buffer.cc",
    "Pl_Concatenate.cc",
    "Pl_Count.cc",
    "Pl_DCT.cc",
    "Pl_Discard.cc",
    "Pl_Flate.cc",
    "Pl_Function.cc",
    "Pl_LZWDecoder.cc",
    "Pl_MD5.cc",
    "Pl_OStream.cc",
    "Pl_PNGFilter.cc",
    "Pl_QPDFTokenizer.cc",
    "Pl_RC4.cc",
    "Pl_RunLength.cc",
    "Pl_SHA2.cc",
    "Pl_StdioFile.cc",
    "Pl_String.cc",
    "Pl_TIFFPredictor.cc",
    "QPDFAcroFormDocumentHelper.cc",
    "QPDFAnnotationObjectHelper.cc",
    "QPDFArgParser.cc",
    "QPDF_Array.cc",
    "QPDF_Bool.cc",
    "QPDF.cc",
    "qpdf-c.cc",
    "QPDFCrypto_gnutls.cc",
    "QPDFCrypto_native.cc",
    "QPDFCrypto_openssl.cc",
    "QPDFCryptoProvider.cc",
    "QPDF_Destroyed.cc",
    "QPDF_Dictionary.cc",
    "QPDFDocumentHelper.cc",
    "QPDFEFStreamObjectHelper.cc",
    "QPDFEmbeddedFileDocumentHelper.cc",
    "QPDF_encryption.cc",
    "QPDFExc.cc",
    "QPDFFileSpecObjectHelper.cc",
    "QPDFFormFieldObjectHelper.cc",
    "QPDF_InlineImage.cc",
    "QPDF_Integer.cc",
    "QPDFJob_argv.cc",
    "QPDFJob.cc",
    "qpdfjob-c.cc",
    "QPDFJob_config.cc",
    "QPDFJob_json.cc",
    "QPDF_json.cc",
    "QPDF_linearization.cc",
    "QPDFLogger.cc",
    "qpdflogger-c.cc",
    "QPDFMatrix.cc",
    "QPDF_Name.cc",
    "QPDFNameTreeObjectHelper.cc",
    "QPDF_Null.cc",
    "QPDFNumberTreeObjectHelper.cc",
    "QPDFObject.cc",
    "QPDFObjectHandle.cc",
    "QPDFObjectHelper.cc",
    "QPDFObjGen.cc",
    "QPDF_Operator.cc",
    "QPDF_optimization.cc",
    "QPDFOutlineDocumentHelper.cc",
    "QPDFOutlineObjectHelper.cc",
    "QPDFPageDocumentHelper.cc",
    "QPDFPageLabelDocumentHelper.cc",
    "QPDFPageObjectHelper.cc",
    "QPDF_pages.cc",
    "QPDFParser.cc",
    "QPDF_Real.cc",
    "QPDF_Reserved.cc",
    "QPDF_Stream.cc",
    "QPDFStreamFilter.cc",
    "QPDF_String.cc",
    "QPDFSystemError.cc",
    "QPDFTokenizer.cc",
    "QPDF_Unresolved.cc",
    "QPDFUsage.cc",
    "QPDFValue.cc",
    "QPDFWriter.cc",
    "QPDFXRefEntry.cc",
    "QTC.cc",
    "QUtil.cc",
    "RC4.cc",
    "RC4_native.cc",
    "ResourceFinder.cc",
    "rijndael.cc",
    "SecureRandomDataProvider.cc",
    "SF_FlateLzwDecode.cc",
    "SHA2_native.cc",
];

fn base_build() -> cc::Build {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut build = cc::Build::new();

    build
        .warnings(false)
        .extra_warnings(false)
        .include(root.join("include"));

    build
}

fn is_msvc() -> bool {
    env::var("TARGET").unwrap().ends_with("-msvc")
}

fn build_cc(name: &str, dir: &str, files: &[&str]) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let path = root.join(dir);

    let cpp_flags: &[&str] = if is_msvc() { &["/D_CRT_SECURE_NO_WARNINGS"] } else { &[] };

    let mut build = base_build();
    for flag in cpp_flags {
        build.flag(flag);
    }

    build
        .include(&path)
        .files(files.iter().map(|f| path.join(f)).collect::<Vec<_>>())
        .compile(name);
}

fn build_qpdf() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cpp_flags: &[&str] = if is_msvc() {
        &["/std:c++17", "/EHsc"]
    } else {
        &["-std=c++17", "-DPOINTERHOLDER_TRANSITION=4"]
    };

    let mut build = base_build();
    for flag in cpp_flags {
        build.flag(flag);
    }

    build
        .cpp(true)
        .include(root.join("zlib-1.3"))
        .include(root.join("jpeg-9d"))
        .include(root.join("qpdf").join("include"))
        .include(root.join("qpdf").join("libqpdf"))
        .files(
            QPDF_SRC
                .iter()
                .map(|f| root.join("qpdf").join("libqpdf").join(f))
                .collect::<Vec<_>>(),
        )
        .compile("qpdf");

    build_cc("sha2", "qpdf/libqpdf", &["sha2.c", "sha2big.c"]);
}

fn build_bindings() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let existing = root
        .join("bindings")
        .join(format!("{}.rs", env::var("TARGET").unwrap()));

    if existing.exists() {
        std::fs::copy(&existing, &out_path).unwrap();
    } else {
        let path = root.join("qpdf").join("include");
        let bindings = bindgen::builder()
            .clang_arg(format!("-I{}", path.display()))
            .header(format!("{}/qpdf/qpdf-c.h", path.display()))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .unwrap();

        bindings.write_to_file(&out_path).expect("Couldn't write bindings!");
    }
}

fn main() {
    build_bindings();
    build_cc("zlib", "zlib-1.3", ZLIB_SRC);
    build_cc("jpeg", "jpeg-9d", JPEG_SRC);
    build_qpdf();
}
