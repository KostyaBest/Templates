
#include "test1_Test1.h"

JNIEXPORT jdouble JNICALL Java_test1_Test1_multiply
  (JNIEnv * env, jobject object, jdouble a, jdouble b){
    return a*b;
  }

JNIEXPORT jdouble JNICALL Java_test1_Test1_division
  (JNIEnv * env, jobject object, jdouble a, jdouble b){
	if (b == 0){
		return 0;	
	}
	return a/b;
}
/*
 * Class:     test1_Test1
 * Method:    sum
 * Signature: (DD)D
 */
JNIEXPORT jdouble JNICALL Java_test1_Test1_sum
  (JNIEnv *env, jobject object, jdouble a, jdouble b){
	return a+b;
}

/*
 * Class:     test1_Test1
 * Method:    minus
 * Signature: (DD)D
 */
JNIEXPORT jdouble JNICALL Java_test1_Test1_minus
  (JNIEnv *env, jobject object, jdouble a, jdouble b){
	return a-b;
}

