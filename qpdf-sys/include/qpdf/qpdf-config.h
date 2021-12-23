#define DEFAULT_CRYPTO "native"
#define USE_CRYPTO_NATIVE 1
#define HAVE_EXTERN_LONG_TIMEZONE 1
#define HAVE_INTTYPES_H 1
#define HAVE_LOCALTIME_R 1
#define HAVE_STDINT_H 1
#define HAVE_TM_GMTOFF 1
#define RANDOM_DEVICE "/dev/urandom"

#ifdef __linux__
#define HAVE_FSEEKO64 1
#endif
