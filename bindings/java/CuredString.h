#ifndef _JAVA_DECANCER_H
#define _JAVA_DECANCER_H

#include <jni.h>
#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT jlong JNICALL Java_com_github_null8626_decancer_CuredString_cure(JNIEnv *, jclass, jstring);
JNIEXPORT jboolean JNICALL Java_com_github_null8626_decancer_CuredString_equals(JNIEnv *, jobject, jstring);
JNIEXPORT jboolean JNICALL Java_com_github_null8626_decancer_CuredString_startsWith(JNIEnv *, jobject, jstring);
JNIEXPORT jboolean JNICALL Java_com_github_null8626_decancer_CuredString_endsWith(JNIEnv *, jobject, jstring);
JNIEXPORT jboolean JNICALL Java_com_github_null8626_decancer_CuredString_contains(JNIEnv *, jobject, jstring);
JNIEXPORT jstring JNICALL Java_com_github_null8626_decancer_CuredString_toString(JNIEnv *, jobject);
JNIEXPORT void JNICALL Java_com_github_null8626_decancer_CuredString_destroy(JNIEnv *, jobject);

#ifdef __cplusplus
}
#endif
#endif