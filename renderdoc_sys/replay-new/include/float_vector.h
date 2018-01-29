#ifndef FLOAT_VECTOR_H
#define FLOAT_VECTOR_H

#ifdef __cplusplus
#define EXTERN_C extern "C" {
#define EXTERN_C_END }
#else
#define EXTERN_C
#define EXTERN_C_END
#endif

EXTERN_C

typedef struct {
    float x;
    float y;
    float z;
    float w;
} FloatVector;

EXTERN_C_END

#endif // FLOAT_VECTOR_H
