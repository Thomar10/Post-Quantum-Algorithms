cmake_minimum_required(VERSION 3.0)
project(FalconC C)
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
add_library(codec STATIC codec.c api.h fpr.h inner.h)
add_library(common STATIC common.c api.h fpr.h inner.h)
add_library(fft STATIC fft.c api.h fpr.h inner.h)
add_library(fpr STATIC fpr.c api.h fpr.h inner.h)
add_library(nist STATIC nist.c api.h fpr.h inner.h katrng.c katrng.h)
add_library(keygen STATIC keygen.c api.h fpr.h inner.h)
add_library(rng STATIC rng.c api.h fpr.h inner.h)
add_library(shake STATIC shake.c api.h fpr.h inner.h)
add_library(sign STATIC sign.c api.h fpr.h inner.h)
add_library(vrfy STATIC vrfy.c api.h fpr.h inner.h)
add_library(testfalcon STATIC test_falcon.c fpr2.h inner2.h falcon2.h)
install(TARGETS codec DESTINATION .)
install(TARGETS common DESTINATION .)
install(TARGETS fft DESTINATION .)
install(TARGETS fpr DESTINATION .)
install(TARGETS nist DESTINATION .)
install(TARGETS keygen DESTINATION .)
install(TARGETS rng DESTINATION .)
install(TARGETS shake DESTINATION .)
install(TARGETS sign DESTINATION .)
install(TARGETS vrfy DESTINATION .)
install(TARGETS testfalcon DESTINATION .)